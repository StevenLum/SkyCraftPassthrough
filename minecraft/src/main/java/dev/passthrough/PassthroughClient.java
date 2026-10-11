package dev.passthrough;

import net.fabricmc.api.ClientModInitializer;
import net.fabricmc.loader.api.FabricLoader;
import net.minecraft.client.Minecraft;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public final class PassthroughClient implements ClientModInitializer {
    private static final Logger LOG = LoggerFactory.getLogger("PassthroughPractice");
    private static NativeLink link;
    private static Object lastLevel;
    private static long world;
    private static long frame;
    private static boolean initialized;
    private static boolean failed;
    static NativeLink overlayLink() { return initialized && !failed ? link : null; }
    static long worldId() { return world; }
    static boolean sameLevel(Object level) { return level==lastLevel; }

    @Override public void onInitializeClient() {
        try {
            var dll = FabricLoader.getInstance().getGameDir().resolve("mods/passthrough_link.dll");
            link = new NativeLink(dll);
            long off = link.switches();
            Switches.set(off);
            LOG.info("Passthrough Fabric plugin loaded");
            LOG.info("event=SCENE_SWITCHES off_mask={}", Long.toBinaryString(off));
        } catch (Throwable error) {
            failed = true;
            LOG.error("Passthrough native library could not load; position sending disabled", error);
        }
    }

    private static long costFrames, costTotal, costWorst, costSince;
    /** Time spent in each part of the frame's work: sending, receiving and collision, hand and HUD capture, block capture. */
    private static final long[] costParts = new long[4];
    /** One log line every five seconds: how long the bridge's own after-render work took, and where. */
    private static void cost(long nanos) {
        long now = System.nanoTime();
        if (costFrames == 0) costSince = now;
        costFrames++; costTotal += nanos; costWorst = Math.max(costWorst, nanos);
        if (now - costSince < 5_000_000_000L) return;
        LOG.info("event=SCENE_COST side=minecraft frames={} avg_us={} max_us={} period_ms={} send_us={} world_us={} overlay_us={} blocks_us={}",
            costFrames, costTotal / costFrames / 1000, costWorst / 1000, (now - costSince) / 1_000_000L,
            costParts[0] / costFrames / 1000, costParts[1] / costFrames / 1000, costParts[2] / costFrames / 1000, costParts[3] / costFrames / 1000);
        costFrames = 0; costTotal = 0; costWorst = 0;
        java.util.Arrays.fill(costParts, 0);
    }

    /** Called AFTER GameRenderer.render on every rendered frame, never from a tick event. */
    public static void afterRender() {
        if (failed || link == null) return;
        long began = System.nanoTime();
        try { bridgeFrame(); } finally { cost(System.nanoTime() - began); }
    }
    private static void bridgeFrame() {
        try {
            // Initialize on the render thread, which owns the Rust connection.
            if (!initialized) {
                int result = link.initialize();
                if (result == 0) return; // Shared lock busy; retry next frame.
                if (result < 0) throw new IllegalStateException("Native connection initialization failed");
                initialized = true;
            }
            Minecraft mc = Minecraft.getInstance();
            if (mc.level != lastLevel) { BridgeDamage.reset();BridgeRecovery.clear();SkyCollision.clear(); lastLevel = mc.level; ++world; }
            var server = mc.getSingleplayerServer();
            BridgeWorld.notice(mc);
            boolean active = mc.player != null && mc.level != null && server != null
                && BridgeWorld.eligible(mc.level) && !server.isPublished() && !mc.isPaused() && !mc.player.isDeadOrDying();
            SkyrimMenus.refresh(link,active);
            double partial = 0.0, x = 0.0, y = 0.0, z = 0.0, yaw = 0.0, pitch = 0.0;
            if (active) {
                float fraction = mc.getDeltaTracker().getGameTimeDeltaPartialTick(false);
                var feet = mc.player.getPosition(fraction);
                partial = fraction; x = feet.x; y = feet.y; z = feet.z;
                yaw = ((mc.player.getViewYRot(fraction) % 360.0) + 540.0) % 360.0 - 180.0;
                pitch = Math.clamp((double)mc.player.getViewXRot(fraction), -90.0, 90.0);
            }
            long t0 = System.nanoTime();
            int result = link.send(world, ++frame, partial, x, y, z, yaw, pitch, active);
            if (result < 0) throw new IllegalStateException("Native position send failed; see project logs/step2");
            long t1 = System.nanoTime();
            if (active && result > 0 && !SkyrimMenus.active()) SkyCollision.accept(link.receiveWorld(), mc.level, server, world);
            else {BridgeRecovery.clear();SkyCollision.clear();}
            if (active && result > 0) SkyCollision.reportOverlap(mc.player);
            long t2 = System.nanoTime();
            OverlayCapture.afterRender(active && result>0 && SkyCollision.admitted());
            long t3 = System.nanoTime();
            SceneCapture.frame(active && result>0 && SkyCollision.admitted());
            long t4 = System.nanoTime();
            costParts[0] += t1 - t0; costParts[1] += t2 - t1; costParts[2] += t3 - t2; costParts[3] += t4 - t3;
        } catch (Throwable error) {
            failed = true;
            BridgeRecovery.clear();
            SkyCollision.clear();
            OverlayCapture.disable(error);
            LOG.error("Passthrough sender disabled", error);
        }
    }
}

