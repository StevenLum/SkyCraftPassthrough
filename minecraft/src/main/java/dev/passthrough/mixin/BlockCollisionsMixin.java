package dev.passthrough.mixin;

import dev.passthrough.SkyCollision;
import java.util.Iterator;
import java.util.function.BiFunction;
import net.minecraft.core.BlockPos;
import net.minecraft.world.level.BlockCollisions;
import net.minecraft.world.level.CollisionGetter;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.shapes.VoxelShape;
import org.spongepowered.asm.mixin.*;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;

/** Extra iterator elements; vanilla still resolves movement and reads Minecraft blocks. */
@Mixin(BlockCollisions.class)
public abstract class BlockCollisionsMixin<T> {
    @Shadow @Final private AABB box;
    @Shadow @Final private CollisionGetter collisionGetter;
    @Shadow @Final private boolean onlySuffocatingBlocks;
    @Shadow @Final private BiFunction<BlockPos.MutableBlockPos,VoxelShape,T> resultProvider;
    @Unique private Iterator<SkyCollision.Shape> passthrough$extra;
    @Inject(method="computeNext",at=@At("HEAD"),cancellable=true)
    private void passthrough$collision(CallbackInfoReturnable<T> callback) {
        if (onlySuffocatingBlocks) return;
        if (passthrough$extra==null) passthrough$extra=SkyCollision.query(collisionGetter,box).iterator();
        if (passthrough$extra.hasNext()) {
            var shape=passthrough$extra.next();
            var p=new BlockPos.MutableBlockPos((int)Math.floor(shape.box().minX),
                (int)Math.floor(shape.box().minY),(int)Math.floor(shape.box().minZ));
            callback.setReturnValue(resultProvider.apply(p,shape.voxel()));
        }
    }
}
