package dev.passthrough;

import net.minecraft.advancements.triggers.CriteriaTriggers;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.world.InteractionResult;
import net.minecraft.world.item.BucketItem;
import net.minecraft.world.item.Item;
import net.minecraft.world.item.Items;
import net.minecraft.world.item.ItemUtils;
import net.minecraft.world.item.context.UseOnContext;
import org.slf4j.LoggerFactory;

/** Filled buckets use the validated terrain destination, without a fake support block. */
public final class TerrainBucket {
    private TerrainBucket() {}
    public static boolean supported(Item item) { return item==Items.WATER_BUCKET || item==Items.LAVA_BUCKET; }
    public static InteractionResult place(UseOnContext context) {
        var player=context.getPlayer(); var level=context.getLevel();
        var cell=context.getClickedPos(); var stack=context.getItemInHand();
        if(player==null || !level.mayInteract(player,cell)
            || !player.mayUseItemAt(cell,context.getClickedFace(),stack)) return result(context,"permission",InteractionResult.FAIL);
        if(!TerrainPlacement.validate(context)) return result(context,"invalid_target",InteractionResult.FAIL);
        var bucket=(BucketItem)stack.getItem();
        // Null hit prevents vanilla's fallback from placing in a different adjacent cell.
        if(!bucket.emptyContents(player,level,cell,null)) return result(context,"empty_failed",InteractionResult.FAIL);
        bucket.checkExtraContent(player,level,stack,cell);
        if(player instanceof ServerPlayer serverPlayer) CriteriaTriggers.PLACED_BLOCK.trigger(serverPlayer,cell,stack);
        // ItemStack.useOn awards the item-use statistic and applies component side effects.
        var replacement=ItemUtils.createFilledResult(stack,player,BucketItem.getEmptySuccessItem(stack,player));
        return result(context,"emptied",InteractionResult.SUCCESS.heldItemTransformedTo(replacement));
    }
    private static InteractionResult result(UseOnContext context,String reason,InteractionResult result) {
        LoggerFactory.getLogger("PassthroughPractice").info(
            "event=SCENE_BUCKET side={} decision={} cell={} item={} fluid={} result={}",
            context.getLevel().isClientSide()?"client":"server",reason,context.getClickedPos(),
            context.getItemInHand().getItem(),context.getLevel().getFluidState(context.getClickedPos()),result);
        return result;
    }
}
