package dev.passthrough.mixin;
import java.util.Map;
import net.minecraft.resources.Identifier;
import net.minecraft.client.renderer.texture.TextureAtlas;
import net.minecraft.client.renderer.texture.TextureAtlasSprite;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.gen.Accessor;
@Mixin(TextureAtlas.class)
public interface AtlasAccess {
    @Accessor("texturesByName") Map<Identifier,TextureAtlasSprite> passthrough$sprites();
    @Accessor("width") int passthrough$width();
    @Accessor("height") int passthrough$height();
}
