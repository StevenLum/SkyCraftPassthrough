package dev.passthrough.mixin;

import dev.passthrough.BridgeDeath;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.world.damagesource.DamageSource;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;

/** Observe Minecraft's own death; nothing about it is changed here. */
@Mixin(ServerPlayer.class)
public abstract class BridgeDeathMixin {
    @Inject(method="die",at=@At("HEAD"))
    private void passthrough$died(DamageSource source,CallbackInfo ci) {
        BridgeDeath.died((ServerPlayer)(Object)this,source);
    }
}
