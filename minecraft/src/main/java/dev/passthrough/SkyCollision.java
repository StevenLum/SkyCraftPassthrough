package dev.passthrough;

import java.nio.ByteBuffer;
import java.util.*;
import java.util.concurrent.atomic.AtomicLong;
import net.minecraft.client.server.IntegratedServer;
import net.minecraft.resources.ResourceKey;
import net.minecraft.core.BlockPos;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.CollisionGetter;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.shapes.Shapes;
import net.minecraft.world.phys.shapes.VoxelShape;
import org.slf4j.LoggerFactory;

/** Immutable world snapshots shared by the client and the local server. No physics replacement. */
public final class SkyCollision {
    public record Shape(AABB box, VoxelShape voxel, boolean terrain) {}
    private record Field(long sender, long epoch, long sequence, long expires,
        Level clientLevel, IntegratedServer server, ResourceKey<Level> dimension,
        Map<BlockPos,List<Shape>> buckets, int boxes, int npcs, AABB coverage, int cellId, int worldspaceId) {}
    private static volatile Field field;
    private static volatile long lastRender;
    private static final AtomicLong clientLog = new AtomicLong(), serverLog = new AtomicLong();
    private static final AtomicLong coverageLog = new AtomicLong();
    private static final AtomicLong targetClientLog = new AtomicLong(), targetServerLog = new AtomicLong();
    private SkyCollision() {}
    public static void clear() { field = null; lastRender = 0; TerrainPlacement.clear(); BridgeDamage.clear(); SkyrimAttack.clear(); }
    public record TerrainHit(net.minecraft.world.phys.BlockHitResult hit,net.minecraft.world.phys.Vec3 surface,long sender,long epoch,long snapshot) {}
    private static TerrainHit targetMiss(Level level,String reason,net.minecraft.world.phys.Vec3 start,net.minecraft.world.phys.Vec3 end) {
        var clock=level.isClientSide()?targetClientLog:targetServerLog;
        long now=System.nanoTime(),previous=clock.get();
        if(now-previous>1_000_000_000L && clock.compareAndSet(previous,now))
            LoggerFactory.getLogger("PassthroughPractice").info(
                "event=SCENE_TARGET_MISS side={} reason={} eye={} end={} reach={} coverage_radius=4",
                level.isClientSide()?"client":"server",reason,start,end,start.distanceTo(end));
        return null;
    }
    public static TerrainHit target(Level level,net.minecraft.world.phys.Vec3 start,net.minecraft.world.phys.Vec3 end) {
        Field before=field;
        var shapes=query(level,new AABB(start,end).inflate(.001));
        if(before==null || field!=before) return targetMiss(level,"missing_or_changed_snapshot",start,end);
        net.minecraft.world.phys.BlockHitResult nearest=null;
        double distance=start.distanceToSqr(end);
        for(var shape:shapes) {
            if(!shape.terrain || shape.box.contains(start)) continue;
            var hit=shape.voxel.clip(start,end,BlockPos.ZERO);
            if(hit==null || hit.isInside()) continue;
            double d=start.distanceToSqr(hit.getLocation());
            if(d>=distance) continue;
            distance=d; nearest=hit;
        }
        if(nearest==null) return targetMiss(level,"no_fresh_terrain_hit",start,end);
        var at=nearest.getLocation(); var face=nearest.getDirection();
        int x=(int)Math.floor(at.x),y=(int)Math.floor(at.y),z=(int)Math.floor(at.z);
        // First whole cell outside the sampled surface, avoiding embedding the new block.
        switch(face) {
            case UP -> y=(int)Math.ceil(at.y-1e-6);
            case DOWN -> y=(int)Math.floor(at.y+1e-6)-1;
            case EAST -> x=(int)Math.ceil(at.x-1e-6);
            case WEST -> x=(int)Math.floor(at.x+1e-6)-1;
            case SOUTH -> z=(int)Math.ceil(at.z-1e-6);
            case NORTH -> z=(int)Math.floor(at.z+1e-6)-1;
        }
        var cell=new BlockPos(x,y,z);
        // Packet coordinates stay inside the destination. The original surface travels
        // separately in the offline intent mailbox; this point is not an aiming ray.
        var click=new net.minecraft.world.phys.Vec3(
            Math.clamp(at.x,x+.001,x+.999),Math.clamp(at.y,y+.001,y+.999),Math.clamp(at.z,z+.001,z+.999));
        return new TerrainHit(new net.minecraft.world.phys.BlockHitResult(click,face,cell,false),at,
            before.sender,before.epoch,before.sequence);
    }

    public static void accept(ByteBuffer b, Level client, IntegratedServer server, long world) {
        if (b == null || server.isPublished() || b.getLong(24) != world) { BridgeRecovery.clear();clear(); return; }
        BridgePairing.observe(b,server);
        BridgeRecovery.observe(b,client,server,world);
        SkyrimAttack.observe(b);
        if(b.getInt(120)!=1) { clear(); return; }
        long now = System.nanoTime(); lastRender = now;
        long sender = b.getLong(8), epoch = b.getLong(32), seq = b.getLong(40);
        Field previous = field;
        if (previous != null && previous.sender == sender && previous.epoch == epoch && previous.sequence == seq) {BridgeDamage.observe(b,client,server,world);return;}
        var player=net.minecraft.client.Minecraft.getInstance().player;
        LoggerFactory.getLogger("PassthroughPractice").info(
            "event=SCENE_HEALTH_CLIENT sender={} world={} epoch={} snapshot={} skyrim_health={} minecraft_health={}",
            sender,world,epoch,seq,b.getFloat(68),player==null ? "unavailable" : player.getHealth());
        int count = b.getInt(60), npcCount = b.getInt(64);
        if (count < 0 || count > 8192 || npcCount < 0 || npcCount > 128) throw new IllegalArgumentException("Invalid snapshot counts");
        Map<BlockPos,List<Shape>> buckets = new HashMap<>();
        for (int i=0; i<count; i++) {
            int p=128+i*48;
            add(buckets,new AABB(b.getDouble(p),b.getDouble(p+8),b.getDouble(p+16),
                b.getDouble(p+24),b.getDouble(p+32),b.getDouble(p+40)),true);
        }
        // Position-only NPC feed: explicitly approximate human-sized body boxes.
        for (int i=0; i<npcCount; i++) {
            int p=128+8192*48+i*32;
            double x=b.getDouble(p+8), y=b.getDouble(p+16), z=b.getDouble(p+24);
            add(buckets,new AABB(x-.3,y,z-.3,x+.3,y+1.8,z+.3),false);
        }
        buckets.replaceAll((key,list)->List.copyOf(list));
        long remaining = Math.max(0,Math.min(2500,b.getLong(72)));
        field = new Field(sender,epoch,seq,now+remaining*1_000_000L,client,server,client.dimension(),Map.copyOf(buckets),count,npcCount,
            new AABB(b.getDouble(80)-b.getDouble(104),b.getDouble(88)-b.getDouble(104),b.getDouble(96)-b.getDouble(104),
                b.getDouble(80)+b.getDouble(104),b.getDouble(88)+b.getDouble(104),b.getDouble(96)+b.getDouble(104)),b.getInt(112),b.getInt(116));
        BridgeDamage.observe(b,client,server,world);
        if(previous==null || previous.sender!=sender || previous.epoch!=epoch
            || previous.cellId!=field.cellId || previous.worldspaceId!=field.worldspaceId)
            LoggerFactory.getLogger("PassthroughPractice").info(
                "event=SCENE_LOCATION_RECV sender={} epoch={} snapshot={} cell_id={} worldspace_id={}",
                sender,epoch,seq,String.format("%08X",field.cellId),String.format("%08X",field.worldspaceId));
        long logged=coverageLog.get();
        if(now-logged>1_000_000_000L && coverageLog.compareAndSet(logged,now))
            LoggerFactory.getLogger("PassthroughPractice").info(
                "event=SCENE_BRIDGE_COVERAGE sender={} epoch={} snapshot={} bounds={} remaining_ms={} boxes={}",
                sender,epoch,seq,field.coverage,remaining,count);
    }
    public static boolean admitted() {
        Field f=field;long now=System.nanoTime();
        return f!=null && now<=f.expires && now-lastRender<=250_000_000L;
    }
    public static boolean covers(Level level,AABB box) {
        return coverageReason(level,box).equals("ready");
    }
    public static String coverageReason(Level level,AABB box) {
        Field f=field; long now=System.nanoTime();
        if(f==null) return "no_snapshot";
        if(now>f.expires) return "snapshot_expired";
        if(now-lastRender>250_000_000L) return "render_heartbeat_expired";
        if(f.server.isPublished() || !level.dimension().equals(f.dimension)
            || !(level.isClientSide()?level==f.clientLevel:level.getServer()==f.server)) return "world_mismatch";
        return box.minX>=f.coverage.minX && box.maxX<=f.coverage.maxX
            && box.minY>=f.coverage.minY && box.maxY<=f.coverage.maxY
            && box.minZ>=f.coverage.minZ && box.maxZ<=f.coverage.maxZ ? "ready":"outside_bounds";
    }
    public static String coverageDetails() {
        Field f=field;
        return f==null?"none":"snapshot="+f.sequence+" epoch="+f.epoch+" bounds="+f.coverage
            +" remaining_ms="+Math.max(0,(f.expires-System.nanoTime())/1_000_000L);
    }
    private static void add(Map<BlockPos,List<Shape>> buckets,AABB box,boolean terrain) {
        Shape shape = new Shape(box,Shapes.create(box),terrain);
        for (int x=(int)Math.floor(box.minX); x<=(int)Math.floor(box.maxX); x++)
            for (int y=(int)Math.floor(box.minY); y<=(int)Math.floor(box.maxY); y++)
                for (int z=(int)Math.floor(box.minZ); z<=(int)Math.floor(box.maxZ); z++)
                    buckets.computeIfAbsent(new BlockPos(x,y,z),key->new ArrayList<>()).add(shape);
    }
    public static List<Shape> query(CollisionGetter getter,AABB box) {
        Field f=field; long now=System.nanoTime();
        if (f==null || now>f.expires || now-lastRender>250_000_000L || !(getter instanceof Level level)
            || !level.dimension().equals(f.dimension) || f.server.isPublished()
            || (level.isClientSide() ? level!=f.clientLevel : level.getServer()!=f.server)) return List.of();
        // Bound query traversal; large queries use the finite snapshot directly.
        Set<Shape> found=new HashSet<>();
        if (box.getXsize()>32 || box.getYsize()>32 || box.getZsize()>32) {
            for (var list:f.buckets.values()) for (var shape:list) if (shape.box.intersects(box)) found.add(shape);
        } else {
            for (int x=(int)Math.floor(box.minX); x<=(int)Math.floor(box.maxX); x++)
                for (int y=(int)Math.floor(box.minY); y<=(int)Math.floor(box.maxY); y++)
                    for (int z=(int)Math.floor(box.minZ); z<=(int)Math.floor(box.maxZ); z++)
                        for (var shape:f.buckets.getOrDefault(new BlockPos(x,y,z),List.of()))
                            if (shape.box.intersects(box)) found.add(shape);
        }
        AtomicLong clock=level.isClientSide()?clientLog:serverLog;
        long before=clock.get();
        if (now-before>1_000_000_000L && clock.compareAndSet(before,now)) {
            LoggerFactory.getLogger("PassthroughPractice").info(
                "event=COLLISION_QUERY side={} sender={} epoch={} snapshot={} matches={} boxes={} npcs={} query={}",
                level.isClientSide()?"client":"server",f.sender,f.epoch,f.sequence,found.size(),f.boxes,f.npcs,box);
        }
        return List.copyOf(found);
    }
}





