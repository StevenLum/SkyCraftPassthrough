package dev.passthrough.mixin;

import dev.passthrough.BridgeWorld;
import dev.passthrough.SkyCollision;
import dev.passthrough.Switches;
import net.minecraft.world.entity.Pose;
import net.minecraft.world.entity.player.Player;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;

/**
 * Minecraft forces a crouch or a crawl when the standing player does not fit where it is.
 * Skyrim has no place a standing character must duck through, and its shapes move (a
 * character's stand-in box, a door), so a shape overlapping the upper body would slow the
 * player to a crawl and stop a jump for no reason Skyrim knows. In a bridge world, Skyrim's
 * shapes are left out while Minecraft asks whether the player fits. Minecraft's own blocks
 * still count, and sneaking with the key is unchanged.
 */
@Mixin(Player.class)
public abstract class BridgePoseMixin {
    @Inject(method="canPlayerFitWithinBlocksAndEntitiesWhen",at=@At("HEAD"))
    private void passthrough$fitAsked(Pose pose,CallbackInfoReturnable<Boolean> ci) {
        if(Switches.on(Switches.POSE_FREE) && BridgeWorld.eligible(((Player)(Object)this).level()))
            SkyCollision.quiet(SkyCollision.POSE_CHECK,true);
    }
    @Inject(method="canPlayerFitWithinBlocksAndEntitiesWhen",at=@At("RETURN"))
    private void passthrough$fitAnswered(Pose pose,CallbackInfoReturnable<Boolean> ci) {
        SkyCollision.quiet(SkyCollision.POSE_CHECK,false);
    }
}
