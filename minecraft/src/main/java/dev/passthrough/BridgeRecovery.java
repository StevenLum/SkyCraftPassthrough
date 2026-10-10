package dev.passthrough;

import java.nio.ByteBuffer;
import net.minecraft.client.Minecraft;
import net.minecraft.client.server.IntegratedServer;
import net.minecraft.world.level.Level;
import net.minecraft.world.phys.Vec3;
import org.slf4j.LoggerFactory;

/** The integrated server teleports normally; acknowledgement waits for the client. */
public final class BridgeRecovery {
    private static volatile Request pending;
    private static Request recent;
    private static long rejected;
    private static final class Request {
        final long token,sender,world;
        final Level level;
        final IntegratedServer server;
        final java.util.UUID player;
        final Vec3 target;
        volatile long freshUntil;
        volatile boolean serverReady;
        boolean logged,queued,applied;
        Request(long token,long sender,long world,Level level,IntegratedServer server,java.util.UUID player,Vec3 target) {
            this.token=token;this.sender=sender;this.world=world;this.level=level;
            this.server=server;this.player=player;this.target=target;
        }
    }
    private BridgeRecovery() {}
    public static boolean holding() {return pending!=null;}
    public static synchronized void clear() {pending=null;}
    private static boolean current(Request r) {
        var mc=Minecraft.getInstance();
        return pending==r && System.nanoTime()<r.freshUntil && mc.level==r.level
            && mc.getSingleplayerServer()==r.server && !r.server.isPublished()
            && PassthroughClient.worldId()==r.world && mc.player!=null && mc.player.getUUID().equals(r.player);
    }
    public static synchronized void observe(ByteBuffer b,Level level,IntegratedServer server,long world) {
        var mc=Minecraft.getInstance();
        if(b==null || mc.player==null || b.getLong(24)!=world || server.isPublished()) {clear();return;}
        long token=b.getLong(NativeLink.WORLD_SIZE-48);
        if(token==0 || b.getInt(124)!=5 || b.getInt(120)!=0) {clear();return;}
        var id=BridgePairing.expected(server);
        if((id[0]==0 && id[1]==0) || id[0]!=b.getLong(NativeLink.WORLD_SIZE-16)
            || id[1]!=b.getLong(NativeLink.WORLD_SIZE-8) || b.getLong(72)<2250) {clear();return;}
        var target=new Vec3(b.getDouble(NativeLink.WORLD_SIZE-40),b.getDouble(NativeLink.WORLD_SIZE-32),b.getDouble(NativeLink.WORLD_SIZE-24));
        if(!Double.isFinite(target.x) || !Double.isFinite(target.y) || !Double.isFinite(target.z)
            || Math.abs(target.x)>29_999_000 || Math.abs(target.z)>29_999_000
            || target.y<level.getMinY() || target.y+2>level.getMaxY()) {
            // Say why once per request instead of holding silently: the Skyrim place lies
            // outside what this Minecraft world can represent (most often its height).
            if(token!=rejected) {
                rejected=token;
                LoggerFactory.getLogger("PassthroughPractice").info("event=SCENE_RECOVERY state=rejected token={} reason=outside_minecraft_world target={} min_y={} max_y={}",token,target,level.getMinY(),level.getMaxY());
                mc.player.sendSystemMessage(net.minecraft.network.chat.Component.literal("Skyrim bridge: this Skyrim place is outside the height or area the Minecraft world can hold. Move to a higher or lower place using Skyrim's own controls."));
            }
            clear();return;
        }
        // Keep deduplication across transient missing snapshots/lock contention.
        Request r=recent;
        if(r!=null && r.token==token && r.sender==b.getLong(8) && r.world==world && r.level==level && r.server==server) {
            if(r.target.distanceToSqr(target)>1e-12) {clear();return;}
        } else {
            r=new Request(token,b.getLong(8),world,level,server,mc.player.getUUID(),target);
            recent=r;
            LoggerFactory.getLogger("PassthroughPractice").info("event=SCENE_RECOVERY state=received token={} sender={} world={} target={}",token,r.sender,world,target);
        }
        r.freshUntil=System.nanoTime()+250_000_000L;pending=r;
        if(r.queued || r.applied) return;
        r.queued=true;
        Request request=r;
        server.execute(()->{
          synchronized(BridgeRecovery.class) {
            request.queued=false;
            if(!current(request)) return;
            var player=server.getPlayerList().getPlayer(request.player);
            if(player==null || !player.level().dimension().equals(level.dimension()) || player.isDeadOrDying()) return;
            request.applied=true;
            player.connection.teleport(target.x,target.y,target.z,player.getYRot(),player.getXRot());
            player.setDeltaMovement(Vec3.ZERO);player.resetFallDistance();
            request.serverReady=player.position().distanceToSqr(target)<=0.0025;
            LoggerFactory.getLogger("PassthroughPractice").info("event=SCENE_RECOVERY state=server_applied token={} ready={} position={}",request.token,request.serverReady,player.position());
          }
        });
    }
    public static synchronized long acknowledgement(boolean enabled) {
        var r=pending;var mc=Minecraft.getInstance();
        if(!enabled || r==null || !current(r) || !r.serverReady || mc.player.position().distanceToSqr(r.target)>0.0025) return 0;
        if(!r.logged) {
            r.logged=true;
            LoggerFactory.getLogger("PassthroughPractice").info("event=SCENE_RECOVERY state=client_ready token={} position={}",r.token,mc.player.position());
        }
        return r.token;
    }
}
