package dev.passthrough.mixin;
import dev.passthrough.BridgeWorld;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.atomic.AtomicInteger;
import net.minecraft.world.level.chunk.ChunkAccess;
import net.minecraft.world.level.levelgen.FlatLevelSource;
import org.slf4j.LoggerFactory;
import org.spongepowered.asm.mixin.*;
import org.spongepowered.asm.mixin.injection.*;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;
@Mixin(FlatLevelSource.class)
public abstract class BridgeChunkMixin {
    @Unique private final AtomicInteger passthrough$logged=new AtomicInteger();
    @Inject(method="buildTerrain",at=@At("RETURN"),cancellable=true)
    private void passthrough$count(CallbackInfoReturnable<CompletableFuture<ChunkAccess>> ci) {
        if(!BridgeWorld.signature((FlatLevelSource)(Object)this) || passthrough$logged.getAndIncrement()>=16) return;
        ci.setReturnValue(ci.getReturnValue().thenApply(chunk->{
            int count=0;
            for(var section:chunk.getSections()) if(!section.hasOnlyAir())
                for(int x=0;x<16;x++) for(int y=0;y<16;y++) for(int z=0;z<16;z++)
                    if(!section.getBlockState(x,y,z).isAir()) count++;
            LoggerFactory.getLogger("PassthroughPractice").info("event=SCENE_BRIDGE_CHUNK chunk={} non_air_blocks={} stage=terrain",chunk.getPos(),count);
            return chunk;
        }));
    }
}
