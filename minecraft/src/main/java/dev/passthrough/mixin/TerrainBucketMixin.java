package dev.passthrough.mixin;

import dev.passthrough.TerrainBucket;
import dev.passthrough.TerrainPlacement;
import net.minecraft.world.item.Item;
import net.minecraft.world.item.context.UseOnContext;
import net.minecraft.world.InteractionResult;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;

/** BucketItem inherits this default useOn; normal bucket use/raycast stays untouched. */
@Mixin(Item.class)
public abstract class TerrainBucketMixin {
    @Inject(method="useOn",at=@At("HEAD"),cancellable=true)
    private void passthrough$bucket(UseOnContext context,CallbackInfoReturnable<InteractionResult> ci) {
        if(TerrainBucket.supported((Item)(Object)this) && TerrainPlacement.local(context.getLevel())
            && context.getLevel().getBlockState(context.getClickedPos()).isAir())
            ci.setReturnValue(TerrainBucket.place(context));
    }
}
