package dev.passthrough.mixin;

import dev.passthrough.SkyrimAttack;
import net.minecraft.world.entity.LivingEntity;
import net.minecraft.world.item.BowItem;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.level.Level;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;

/**
 * Observes the player letting go of a drawn bow, on the client. Minecraft's own shot is not
 * changed: the server still makes its arrow and uses one up. The bridge only learns that a
 * shot was made and how far the bow was drawn, and asks Skyrim what it hits.
 */
@Mixin(BowItem.class)
public abstract class SkyrimBowMixin {
    @Inject(method="releaseUsing",at=@At("HEAD"))
    private void passthrough$shot(ItemStack stack,Level level,LivingEntity entity,int timeLeft,CallbackInfoReturnable<Boolean> ci) {
        if(level.isClientSide()) SkyrimAttack.shot((BowItem)(Object)this,stack,entity,timeLeft);
    }
}
