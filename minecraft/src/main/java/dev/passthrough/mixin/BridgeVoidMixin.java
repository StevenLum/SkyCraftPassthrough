package dev.passthrough.mixin;

import dev.passthrough.BridgeWorld;
import dev.passthrough.Switches;
import net.minecraft.world.entity.LivingEntity;
import net.minecraft.world.entity.player.Player;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;

/**
 * In a bridge world the ground is Skyrim's, at any depth, and the player is held in place
 * whenever that ground is not known. Minecraft's "fell out of the world" damage (applied by
 * LivingEntity.onBelowWorld once an entity is more than 64 blocks below the world's floor)
 * would only punish being in a low Skyrim place, so a player in a bridge world is exempt.
 * Every other world and every other creature keeps Minecraft's rule.
 */
@Mixin(LivingEntity.class)
public abstract class BridgeVoidMixin {
    @Inject(method="onBelowWorld",at=@At("HEAD"),cancellable=true)
    private void passthrough$noVoid(CallbackInfo ci) {
        if((Object)this instanceof Player player && Switches.on(Switches.VOID_GUARD) && BridgeWorld.eligible(player.level())) ci.cancel();
    }
}
