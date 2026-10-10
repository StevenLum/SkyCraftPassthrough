package dev.passthrough.mixin;
import dev.passthrough.BridgeWorld;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.entity.MoverType;
import net.minecraft.world.entity.player.Player;
import net.minecraft.world.phys.Vec3;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.*;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;
@Mixin(Entity.class)
public abstract class BridgeMotionMixin {
    @Inject(method="move",at=@At("HEAD"),cancellable=true)
    private void passthrough$hold(MoverType type,Vec3 movement,CallbackInfo ci) {
        if((Object)this instanceof Player player && BridgeWorld.hold(player,movement)) ci.cancel();
    }
}
