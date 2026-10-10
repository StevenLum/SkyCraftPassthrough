package dev.passthrough;

import java.nio.ByteBuffer;
import java.util.ArrayList;
import java.util.Map;
import net.minecraft.client.Minecraft;
import net.minecraft.client.model.geom.builders.UVPair;
import net.minecraft.client.renderer.block.dispatch.BlockStateModelPart;
import net.minecraft.client.renderer.texture.TextureAtlasSprite;
import net.minecraft.client.resources.model.geometry.BakedQuad;
import net.minecraft.core.Direction;
import net.minecraft.util.RandomSource;
import net.minecraft.world.item.BlockItem;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.level.block.RenderShape;

/** Small default-state block models. Unsupported geometry uses the icon fallback. */
final class BlockDropCapture {
    private static final RandomSource RANDOM=RandomSource.create(0);
    private static final int[] TRIANGLES={0,1,2,0,2,3};
    private BlockDropCapture() {}

    static int append(ByteBuffer mesh,TranslucentMesh translucent,int limit,ItemStack stack,Map<TextureAtlasSprite,float[]> sprites,
                      float x,float bottom,float z,float spin) {
        if(!(stack.getItem() instanceof BlockItem item)) return 0;
        var state=item.getBlock().defaultBlockState();
        if(state.getRenderShape()!=RenderShape.MODEL) return 0;
        var mc=Minecraft.getInstance();
        var parts=new ArrayList<BlockStateModelPart>();
        RANDOM.setSeed(0);
        mc.getModelManager().getBlockStateModelSet().get(state).collectParts(RANDOM,parts);
        var quads=new ArrayList<BakedQuad>();
        for(var part:parts) {
            quads.addAll(part.getQuads(null));
            for(var direction:Direction.values()) quads.addAll(part.getQuads(direction));
            if(quads.size()>96) return 0;
        }
        if(quads.isEmpty() || mesh.position()+(translucent.vertices()+quads.size()*6)*24>128+limit*24) return 0;
        for(var face:quads) {
            if(!sprites.containsKey(face.materialInfo().sprite())) return 0;
            var sprite=face.materialInfo().sprite();
            if(sprite.getU1()<=sprite.getU0() || sprite.getV1()<=sprite.getV0()) return 0;
            for(int k=0;k<4;k++) {
                var p=face.position(k);
                if(!Float.isFinite(p.x()) || !Float.isFinite(p.y()) || !Float.isFinite(p.z()) ||
                   p.x()<0 || p.x()>1 || p.y()<0 || p.y()>1 || p.z()<0 || p.z()>1) return 0;
            }
        }
        float cos=(float)Math.cos(spin),sin=(float)Math.sin(spin);
        for(var quad:quads) {
            var material=quad.materialInfo(); var sprite=material.sprite(); var rect=sprites.get(sprite);
            int color=material.isTinted()?mc.getBlockColors().getTintSource(state,material.tintIndex()).color(state):-1;
            boolean transparent=material.layer().translucent();
            var target=transparent?translucent.buffer():mesh;
            float cx=0,cy=0,cz=0;
            for(int k:TRIANGLES) {
                var p=quad.position(k); float px=(p.x()-0.5f)*0.25f,pz=(p.z()-0.5f)*0.25f;
                long uv=quad.packedUV(k);
                float u=(UVPair.unpackU(uv)-sprite.getU0())/(sprite.getU1()-sprite.getU0());
                float v=(UVPair.unpackV(uv)-sprite.getV0())/(sprite.getV1()-sprite.getV0());
                float vx=x+px*cos-pz*sin,vy=bottom+p.y()*0.25f,vz=z+px*sin+pz*cos;
                target.putFloat(vx).putFloat(vy).putFloat(vz);
                target.putFloat(rect[0]+u*(rect[2]-rect[0])).putFloat(rect[1]+v*(rect[3]-rect[1]));
                target.put((byte)(color>>16)).put((byte)(color>>8)).put((byte)color).put((byte)255);
                // For planar quads the midpoint of opposite corners is their centre.
                if(k==1 || k==3) { cx+=vx*0.5f; cy+=vy*0.5f; cz+=vz*0.5f; }
            }
            if(transparent) translucent.finishQuad(cx,cy,cz);
        }
        return quads.size()*6;
    }
}
