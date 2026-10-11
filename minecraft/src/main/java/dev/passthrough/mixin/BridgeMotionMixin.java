package dev.passthrough.mixin;
import dev.passthrough.BridgeWorld;
import dev.passthrough.Switches;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.entity.MoverType;
import net.minecraft.world.entity.player.Player;
import net.minecraft.world.phys.Vec3;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.*;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;
/**
 * Holds the player still while Skyrim's ground is not known. Only the client's player is
 * held: it is the side that moves. The server's copy follows the steps the client reports,
 * and holding it too would make the two disagree (unless server_trust is switched off).
 */
@Mixin(Entity.class)
public abstract class BridgeMotionMixin {
    @Inject(method="move",at=@At("HEAD"),cancellable=true)
    private void passthrough$hold(MoverType type,Vec3 movement,CallbackInfo ci) {
        if((Object)this instanceof Player player
            && (player.level().isClientSide() || !Switches.on(Switches.SERVER_TRUST))
            && BridgeWorld.hold(player,movement)) ci.cancel();
    }
}
