package dev.passthrough;

import java.nio.ByteBuffer;
import net.minecraft.client.Minecraft;
import net.minecraft.client.server.IntegratedServer;
import net.minecraft.world.damagesource.DamageSource;
import net.minecraft.world.level.Level;
import net.minecraft.world.phys.Vec3;
import org.slf4j.LoggerFactory;

/** One attempt per native damage event, exclusively on the integrated server. */
public final class BridgeDamage {
    public record Status(long epoch,long ack,int ready) {}
    private record Hit(long sequence,float fraction,float raw) {}
    private static State state;
    private static final class State {
        final long sender,epoch,world;
        final Level level;
        final IntegratedServer server;
        final java.util.UUID player;
        long ack,freshUntil,readyUntil,lastQueue;
        boolean queued,failed;
        State(long sender,long epoch,long world,Level level,IntegratedServer server,java.util.UUID player) {
            this.sender=sender;this.epoch=epoch;this.world=world;this.level=level;this.server=server;this.player=player;
        }
    }
    private static boolean warned;
    private BridgeDamage() {}
    /** Position of the closest Skyrim character listed in this snapshot, or null when none is. */
    private static Vec3 nearest(ByteBuffer b,Vec3 from) {
        int count=b.getInt(64);
        if(count<=0 || count>128) return null;
        Vec3 best=null;double closest=Double.MAX_VALUE;
        for(int i=0;i<count;i++) {
            int at=128+8192*48+i*32;
            var position=new Vec3(b.getDouble(at+8),b.getDouble(at+16),b.getDouble(at+24));
            double distance=position.distanceToSqr(from);
            if(Double.isFinite(distance) && distance<closest) {closest=distance;best=position;}
        }
        return best;
    }
    /**
     * Skyrim hits arrive as an ordinary mob attack, so armour and a raised shield act on them.
     * Minecraft's "generic" type skips both. Any lookup problem falls back to generic.
     */
    private static DamageSource source(IntegratedServer server,net.minecraft.server.level.ServerPlayer player,Vec3 from) {
        try {
            var type=server.overworld().registryAccess().lookupOrThrow(net.minecraft.core.registries.Registries.DAMAGE_TYPE)
                .getOrThrow(net.minecraft.world.damagesource.DamageTypes.MOB_ATTACK);
            return from==null ? new DamageSource(type) : new DamageSource(type,from);
        } catch(RuntimeException error) {
            if(!warned) {
                warned=true;
                LoggerFactory.getLogger("PassthroughPractice").warn("event=SCENE_DAMAGE_SOURCE state=fallback_generic",error);
            }
            return player.damageSources().generic();
        }
    }
    public static synchronized void clear() {if(state!=null) {state.freshUntil=0;state.readyUntil=0;}}
    public static synchronized void reset() {state=null;}
    private static boolean current(State s) {
        var mc=Minecraft.getInstance();
        return state==s && !s.failed && System.nanoTime()<s.freshUntil && mc.level==s.level
            && mc.getSingleplayerServer()==s.server && !s.server.isPublished() && mc.player!=null
            && mc.player.getUUID().equals(s.player) && !mc.player.isDeadOrDying() && !mc.isPaused()
            && !SkyrimMenus.active() && !BridgeRecovery.holding() && SkyCollision.admitted()
            && PassthroughClient.worldId()==s.world;
    }
    public static synchronized Status status(boolean active) {
        var s=state;
        if(!active || s==null || !current(s)) return new Status(0,0,0);
        return new Status(s.epoch,s.ack,System.nanoTime()<s.readyUntil ? 1 : 0);
    }
    public static synchronized void observe(ByteBuffer b,Level level,IntegratedServer server,long world) {
        var mc=Minecraft.getInstance();
        if(b==null || mc.player==null || b.getInt(120)!=1 || b.getLong(24)!=world || b.getLong(72)<2000 || !BridgeWorld.marked(server)) {clear();return;}
        var id=BridgePairing.expected(server);
        if((id[0]==0 && id[1]==0) || id[0]!=b.getLong(NativeLink.WORLD_SIZE-16) || id[1]!=b.getLong(NativeLink.WORLD_SIZE-8)) {clear();return;}
        int base=NativeLink.WORLD_SIZE-48-528;
        long epoch=b.getLong(base),sender=b.getLong(8);
        if(epoch==0) {clear();return;}
        if(state==null || state.epoch!=epoch || state.sender!=sender || state.world!=world || state.level!=level || state.server!=server || !state.player.equals(mc.player.getUUID())) {
            state=new State(sender,epoch,world,level,server,mc.player.getUUID());
            LoggerFactory.getLogger("PassthroughPractice").info("event=SCENE_DAMAGE_RECEIVER epoch={} sender={} world={}",epoch,sender,world);
        }
        State s=state;long now=System.nanoTime();s.freshUntil=now+250_000_000L;
        if(s.failed || s.queued || now-s.lastQueue<100_000_000L) return;
        int count=b.getInt(base+8);if(count<0 || count>32) throw new IllegalArgumentException("Invalid damage count");
        Hit[] hits=new Hit[count];long previous=0;
        for(int i=0;i<count;i++) {
            int at=base+16+i*16;long sequence=b.getLong(at);float fraction=b.getFloat(at+8),raw=b.getFloat(at+12);
            if(sequence<=previous || !Float.isFinite(fraction) || fraction<=0 || !Float.isFinite(raw) || raw<=0) throw new IllegalArgumentException("Invalid damage event");
            hits[i]=new Hit(sequence,fraction,raw);previous=sequence;
        }
        // The closest Skyrim character gives the hit a direction for shields and knockback.
        Vec3 attacker=nearest(b,mc.player.position());
        s.queued=true;s.lastQueue=now;
        server.execute(()->{
          synchronized(BridgeDamage.class) {
            s.queued=false;
            if(!current(s)) return;
            var player=server.getPlayerList().getPlayer(s.player);
            if(player==null || player.isDeadOrDying() || player.isSpectator() || player.level()!=server.overworld()) {s.readyUntil=0;return;}
            try {
                var source=source(server,player,attacker);
                for(var hit:hits) {
                    if(hit.sequence<=s.ack) continue;
                    if(player.isDeadOrDying()) {s.readyUntil=0;return;}
                    float requested=hit.fraction*player.getMaxHealth();
                    if(!Float.isFinite(requested) || requested<=0) throw new IllegalArgumentException("Invalid scaled damage");
                    // Commit consumption before calling gameplay code; never retry a partial call.
                    s.ack=hit.sequence;float before=player.getHealth();
                    boolean blocking=player.isBlocking();int armor=player.getArmorValue();
                    boolean accepted=player.hurtServer(server.overworld(),source,requested);
                    LoggerFactory.getLogger("PassthroughPractice").info(
                        "event=SCENE_DAMAGE_APPLY epoch={} sequence={} skyrim_damage={} fraction={} requested={} before={} after={} accepted={} source={} attacker={} armor={} blocking={}",
                        s.epoch,hit.sequence,hit.raw,hit.fraction,requested,before,player.getHealth(),accepted,source.getMsgId(),attacker==null ? "none" : attacker,armor,blocking);
                }
                s.readyUntil=player.isDeadOrDying() ? 0 : System.nanoTime()+500_000_000L;
            } catch(Exception error) {
                s.failed=true;s.readyUntil=0;
                LoggerFactory.getLogger("PassthroughPractice").error("event=SCENE_DAMAGE_DISABLED epoch="+s.epoch,error);
            }
          }
        });
    }
}
