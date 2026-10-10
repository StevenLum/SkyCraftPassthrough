package dev.passthrough.mixin;

import dev.passthrough.SkyrimMenus;
import net.minecraft.client.MouseHandler;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.Shadow;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;

/** While a Skyrim menu is open, the mouse drives that menu instead of the Minecraft player. */
@Mixin(MouseHandler.class)
public abstract class SkyrimCursorMixin {
    @Shadow private double accumulatedDX;
    @Shadow private double accumulatedDY;
    @Inject(method="turnPlayer",at=@At("HEAD"),cancellable=true)
    private void passthrough$cursor(double time,CallbackInfo ci) {
        if(SkyrimMenus.mouse(accumulatedDX,accumulatedDY)) ci.cancel();
    }
    @Inject(method="onScroll",at=@At("HEAD"),cancellable=true)
    private void passthrough$wheel(long window,double horizontal,double vertical,CallbackInfo ci) {
        if(SkyrimMenus.scroll(vertical)) ci.cancel();
    }
}
