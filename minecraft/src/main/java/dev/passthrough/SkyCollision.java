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
    public record Shape(AABB box, VoxelShape voxel) {}
    private record Field(long sender, long epoch, long sequence, long expires,
        Level clientLevel, IntegratedServer server, ResourceKey<Level> dimension,
        Map<BlockPos,List<Shape>> buckets, int boxes, int npcs) {}
    private static volatile Field field;
    private static volatile long lastRender;
    private static final AtomicLong clientLog = new AtomicLong(), serverLog = new AtomicLong();
    private SkyCollision() {}
    public static void clear() { field = null; lastRender = 0; }

    public static void accept(ByteBuffer b, Level client, IntegratedServer server, long world) {
        if (b == null || server.isPublished() || b.getLong(24) != world) { clear(); return; }
        long now = System.nanoTime(); lastRender = now;
        long sender = b.getLong(8), epoch = b.getLong(32), seq = b.getLong(40);
        Field previous = field;
        if (previous != null && previous.sender == sender && previous.epoch == epoch && previous.sequence == seq) return;
        int count = b.getInt(60), npcCount = b.getInt(64);
        if (count < 0 || count > 8192 || npcCount < 0 || npcCount > 128) throw new IllegalArgumentException("Invalid snapshot counts");
        Map<BlockPos,List<Shape>> buckets = new HashMap<>();
        for (int i=0; i<count; i++) {
            int p=128+i*48;
            add(buckets,new AABB(b.getDouble(p),b.getDouble(p+8),b.getDouble(p+16),
                b.getDouble(p+24),b.getDouble(p+32),b.getDouble(p+40)));
        }
        // Position-only NPC feed: explicitly approximate human-sized body boxes.
        for (int i=0; i<npcCount; i++) {
            int p=128+8192*48+i*32;
            double x=b.getDouble(p+8), y=b.getDouble(p+16), z=b.getDouble(p+24);
            add(buckets,new AABB(x-.3,y,z-.3,x+.3,y+1.8,z+.3));
        }
        buckets.replaceAll((key,list)->List.copyOf(list));
        long remaining = Math.max(0,Math.min(2500,b.getLong(72)));
        field = new Field(sender,epoch,seq,now+remaining*1_000_000L,client,server,client.dimension(),Map.copyOf(buckets),count,npcCount);
    }
    private static void add(Map<BlockPos,List<Shape>> buckets,AABB box) {
        Shape shape = new Shape(box,Shapes.create(box));
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
