package dev.passthrough.mixin;

import dev.passthrough.TerrainPlacement;
import net.minecraft.client.Minecraft;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;

@Mixin(Minecraft.class)
public abstract class TerrainPickMixin {
    @Inject(method="pick",at=@At("TAIL"))
    private void passthrough$terrain(float partial,CallbackInfo ci) {
        var mc=(Minecraft)(Object)this;
        if(mc.player==null || mc.hitResult==null || mc.getCameraEntity()!=mc.player) return;
        var hit=TerrainPlacement.pick(mc.player,mc.player.blockInteractionRange(),partial,false,mc.hitResult);
        if(hit!=mc.hitResult) { mc.hitResult=hit; mc.crosshairPickEntity=null; }
    }
}
