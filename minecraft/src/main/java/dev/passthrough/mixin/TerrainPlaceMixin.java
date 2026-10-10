package dev.passthrough.mixin;

import dev.passthrough.TerrainPlacement;
import net.minecraft.world.item.BlockItem;
import net.minecraft.world.item.context.UseOnContext;
import net.minecraft.world.InteractionResult;
import org.slf4j.LoggerFactory;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;

@Mixin(BlockItem.class)
public abstract class TerrainPlaceMixin {
    @Inject(method="useOn",at=@At("HEAD"),cancellable=true)
    private void passthrough$validate(UseOnContext context,CallbackInfoReturnable<InteractionResult> ci) {
        // Vanilla block targeting cannot hit air. Our terrain target deliberately uses an empty cell.
        if(TerrainPlacement.local(context.getLevel()) && context.getLevel().getBlockState(context.getClickedPos()).isAir()
            && !TerrainPlacement.validate(context)) ci.setReturnValue(InteractionResult.FAIL);
    }
    @Inject(method="useOn",at=@At("RETURN"))
    private void passthrough$result(UseOnContext context,CallbackInfoReturnable<InteractionResult> ci) {
        if(!TerrainPlacement.local(context.getLevel())) return;
        LoggerFactory.getLogger("PassthroughPractice").info(
            "event=SCENE_PLACE_RESULT side={} cell={} result={} state={}",
            context.getLevel().isClientSide()?"client":"server",context.getClickedPos(),
            ci.getReturnValue(),context.getLevel().getBlockState(context.getClickedPos()));
    }
}
