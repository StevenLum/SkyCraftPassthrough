package dev.passthrough.mixin;
import net.minecraft.world.level.levelgen.flat.FlatLevelGeneratorSettings;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.gen.Accessor;
@Mixin(FlatLevelGeneratorSettings.class)
public interface FlatSettingsAccess {
    @Accessor("decoration") boolean passthrough$decoration();
    @Accessor("addLakes") boolean passthrough$lakes();
}
