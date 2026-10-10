package dev.passthrough;

import com.mojang.blaze3d.systems.RenderSystem;
import com.mojang.renderpearl.api.buffers.GpuBuffer;
import java.lang.foreign.MemorySegment;
import net.minecraft.client.Minecraft;
import org.joml.Vector4f;
import org.slf4j.LoggerFactory;

/** Asynchronous hand/HUD readback. No images are saved to disk. Render thread only. */
public final class OverlayCapture {
    private static final org.slf4j.Logger LOG=LoggerFactory.getLogger("PassthroughPractice");
    private static final class Slot {
        GpuBuffer buffer; int width,height; volatile int state;
        long frame,world,captured,started;
    }
    private static final Slot[] slots={new Slot(),new Slot(),new Slot()};
    private static boolean enabled,failed,cleared;
    private static long captured,sequence,lastPublished,lastCapture,lastReport;
    private static int skipped;
    private OverlayCapture() {}
    public static void disable(Throwable error) {
        if (!failed) LOG.error("event=OVERLAY_DISABLED; normal Minecraft world rendering restored",error);
        failed=true; enabled=false;
    }
    public static void beforeRender() {
        cleared=false;
        try {
            Minecraft mc=Minecraft.getInstance(); NativeLink link=PassthroughClient.overlayLink();
            boolean next=false;
            if (!failed && link!=null && mc.player!=null && mc.level!=null && PassthroughClient.sameLevel(mc.level)
                && BridgeWorld.eligible(mc.level) && mc.getSingleplayerServer()!=null && !mc.getSingleplayerServer().isPublished()
                && !mc.isPaused() && !mc.player.isDeadOrDying()) {
                var target=mc.gameRenderer.mainRenderTarget();
                boolean size=target.width>0 && target.height>0 && target.width<=2560 && target.height<=1440;
                captured=link.overlayReady(); next=size && captured!=0;
                if (!size && System.nanoTime()-lastReport>5_000_000_000L) {
                    lastReport=System.nanoTime(); LOG.warn("event=OVERLAY_SIZE_UNSUPPORTED width={} height={} maximum=2560x1440",target.width,target.height);
                }
            }
            if (enabled!=next) LOG.info("event=OVERLAY_MODE active={}",next);
            enabled=next;
        } catch (Throwable error) { disable(error); }
    }
    public static boolean clearWorld() {
        if (!enabled) return false;
        try {
            var target=Minecraft.getInstance().gameRenderer.mainRenderTarget();
            RenderSystem.getDevice().createCommandEncoder().clearColorAndDepthTextures(
                target.getColorTexture(),new Vector4f(0,0,0,0),target.getDepthTexture(),0.0);
            cleared=true; return true;
        } catch (Throwable error) { disable(error); return false; }
    }
    public static void afterRender(boolean active) {
        try {
            if (!enabled || !cleared || !active || failed) {
                for (var s:slots) if (s.state==2) s.state=0;
                return;
            }
            NativeLink link=PassthroughClient.overlayLink();
            if (link==null) return;
            Slot newest=null;
            for (var s:slots) if (s.state==2 && (newest==null || s.frame>newest.frame)) newest=s;
            if (newest!=null) {
                if (newest.world==PassthroughClient.worldId() && newest.frame>lastPublished) {
                    int result;
                    try (var view=newest.buffer.map(true,false)) {
                        var pixels=MemorySegment.ofBuffer(view.data());
                        int length=newest.width*newest.height*4;
                        if (pixels.byteSize()<length) throw new IllegalStateException("short GPU readback");
                        result=link.overlay(newest.world,newest.frame,newest.captured,newest.width,newest.height,pixels,length);
                    }
                    if (result<0) throw new IllegalStateException("Native overlay publish failed; see project log");
                    lastPublished=newest.frame;
                    LOG.info("event=OVERLAY_CAPTURE frame={} readback_age_us={} sent={} skipped={}",newest.frame,(System.nanoTime()-newest.started)/1000,result,skipped);
                }
                for (var s:slots) if (s.state==2 && s.frame<=newest.frame) s.state=0;
            }
            long now=System.nanoTime(); if (now-lastCapture<33_333_333L) return;
            Slot free=null; for (var s:slots) if (s.state==0) { free=s; break; }
            if (free==null) { skipped++; return; }
            var target=Minecraft.getInstance().gameRenderer.mainRenderTarget(); var color=target.getColorTexture();
            if (color==null) return;
            if (color.getFormat()!=com.mojang.renderpearl.api.GpuFormat.RGBA8_UNORM) throw new IllegalStateException("Unsupported overlay format: "+color.getFormat());
            if (target.width<=0 || target.height<=0 || target.width>2560 || target.height>1440) return;
            if (free.buffer==null || free.width!=target.width || free.height!=target.height) {
                if (free.buffer!=null) free.buffer.close();
                free.width=target.width; free.height=target.height;
                free.buffer=RenderSystem.getDevice().createBuffer(()->"Passthrough overlay readback",9,(long)free.width*free.height*4);
                LOG.info("event=OVERLAY_BUFFER width={} height={} format={}",free.width,free.height,color.getFormat());
            }
            final Slot slot=free; slot.frame=++sequence; slot.world=PassthroughClient.worldId();
            slot.captured=captured; slot.started=now; slot.state=1;
            RenderSystem.getDevice().createCommandEncoder().copyTextureToBuffer(color,slot.buffer,0L,()->slot.state=2,0);
            lastCapture=now;
        } catch (Throwable error) { disable(error); }
    }
}

