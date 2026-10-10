package dev.passthrough;

import net.minecraft.client.Minecraft;
import net.minecraft.client.server.IntegratedServer;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.world.damagesource.DamageSource;
import net.minecraft.world.level.gamerules.GameRules;
import org.slf4j.LoggerFactory;

/** Each Minecraft death in a bridge world is reported to Skyrim once, for a short time. */
public final class BridgeDeath {
    public record Report(long sequence,int ageMs) {}
    private static final long LIFETIME_MS=10_000;
    private static volatile Event latest;
    private record Event(long sequence,long at,IntegratedServer server) {}
    private static long sequence;
    private BridgeDeath() {}
    /** Server thread: the player is dying under Minecraft's own rules. */
    public static synchronized void died(ServerPlayer player,DamageSource source) {
        if(!(player.level().getServer() instanceof IntegratedServer server) || server.isPublished() || !BridgeWorld.marked(server)) return;
        latest=new Event(++sequence,System.nanoTime(),server);
        var rules=server.getGameRules();
        LoggerFactory.getLogger("PassthroughPractice").info("event=SCENE_DEATH_SEND sequence={} cause={} health={} immediate_respawn={} keep_inventory={}",
            sequence,source.getMsgId(),player.getHealth(),rules.get(GameRules.IMMEDIATE_RESPAWN),rules.get(GameRules.KEEP_INVENTORY));
    }
    /** Render thread: the death to publish with this frame, or none. */
    public static Report frame(boolean active) {
        var event=latest;
        if(!active || event==null || Minecraft.getInstance().getSingleplayerServer()!=event.server) return new Report(0,0);
        long age=(System.nanoTime()-event.at)/1_000_000L;
        return age<0 || age>LIFETIME_MS ? new Report(0,0) : new Report(event.sequence,(int)age);
    }
}
