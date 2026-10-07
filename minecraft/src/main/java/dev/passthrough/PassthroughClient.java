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

    @Override public void onInitializeClient() {
        try {
            var dll = FabricLoader.getInstance().getGameDir().resolve("mods/passthrough_link.dll");
            link = new NativeLink(dll);
            LOG.info("Passthrough Fabric plugin loaded");
        } catch (Throwable error) {
            failed = true;
            LOG.error("Passthrough native library could not load; position sending disabled", error);
        }
    }

    /** Called AFTER GameRenderer.render on every rendered frame, never from a tick event. */
    public static void afterRender() {
        if (failed || link == null) return;
        try {
            // Initialize on the render thread, which owns the Rust connection.
            if (!initialized) {
                int result = link.initialize();
                if (result == 0) return; // Shared lock busy; retry next frame.
                if (result < 0) throw new IllegalStateException("Native connection initialization failed");
                initialized = true;
            }
            Minecraft mc = Minecraft.getInstance();
            if (mc.level != lastLevel) { SkyCollision.clear(); lastLevel = mc.level; ++world; }
            var server = mc.getSingleplayerServer();
            boolean active = mc.player != null && mc.level != null && server != null
                && !server.isPublished() && !mc.isPaused() && !mc.player.isDeadOrDying();
            double partial = 0.0, x = 0.0, y = 0.0, z = 0.0, yaw = 0.0, pitch = 0.0;
            if (active) {
                float fraction = mc.getDeltaTracker().getGameTimeDeltaPartialTick(false);
                var feet = mc.player.getPosition(fraction);
                partial = fraction; x = feet.x; y = feet.y; z = feet.z;
                yaw = ((mc.player.getViewYRot(fraction) % 360.0) + 540.0) % 360.0 - 180.0;
                pitch = Math.clamp((double)mc.player.getViewXRot(fraction), -90.0, 90.0);
            }
            int result = link.send(world, ++frame, partial, x, y, z, yaw, pitch, active);
            if (result < 0) throw new IllegalStateException("Native position send failed; see project logs/step2");
            if (active && result > 0) SkyCollision.accept(link.receiveWorld(), mc.level, server, world);
            else SkyCollision.clear();
        } catch (Throwable error) {
            failed = true;
            SkyCollision.clear();
            LOG.error("Passthrough sender disabled", error);
        }
    }
}
