package dev.passthrough;

import com.mojang.blaze3d.platform.NativeImage;
import java.io.IOException;
import java.nio.ByteBuffer;
import net.minecraft.client.Minecraft;
import net.minecraft.resources.Identifier;
import net.minecraft.world.level.block.RenderShape;
import org.slf4j.LoggerFactory;

/** Runtime-only textures and a bounded overlay for full opaque blocks being mined. */
final class CrackCapture {
    static final int STRIP_HEIGHT=16, MAX_BLOCKS=32;
    private static final org.slf4j.Logger LOG=LoggerFactory.getLogger("PassthroughPractice");
    private static final int[][] FACES={
        {0,2,3,1},{4,5,7,6},{0,4,6,2},{1,3,7,5},{0,1,5,4},{2,6,7,3}
    };
    private static final int[] TRIANGLES={0,1,2,0,2,3};
    private CrackCapture() {}

    static void copyTextures(ByteBuffer atlas,int width,int blockHeight) throws IOException {
        var resources=Minecraft.getInstance().getResourceManager();
        for(int stage=0;stage<10;stage++) {
            var id=Identifier.withDefaultNamespace("textures/block/destroy_stage_"+stage+".png");
            var resource=resources.getResource(id).orElseThrow(()->new IOException("Missing crack texture: "+id));
            try(var input=resource.open();var image=NativeImage.read(input)) {
                for(int y=0;y<16;y++) for(int x=0;x<16;x++) {
                    int color=image.getPixel(x*image.getWidth()/16,y*image.getHeight()/16);
                    int at=128+((blockHeight+y)*width+stage*16+x)*4;
                    atlas.put(at,(byte)(color>>16)).put(at+1,(byte)(color>>8)).put(at+2,(byte)color).put(at+3,(byte)(color>>>24));
                }
            }
        }
        LOG.info("event=SCENE_CRACK_TEXTURES stages=10 size=16");
    }

    static int append(ByteBuffer mesh,int limit,int ox,int oy,int oz,int side,
                      int width,int blockHeight,int height,long snapshot) {
        var level=Minecraft.getInstance().level;
        int vertices=0,blocks=0,skipped=0;
        for(var stages:level.destructionProgress().values()) {
            if(stages.isEmpty()) continue;
            var progress=stages.last();
            int stage=progress.getProgress(); var pos=progress.getPos();
            if(stage<0 || stage>9 || pos.getX()<ox || pos.getX()>=ox+side ||
               pos.getY()<oy || pos.getY()>=oy+side || pos.getZ()<oz || pos.getZ()>=oz+side) continue;
            var state=level.getBlockState(pos);
            if(state.isAir()) continue;
            if(state.getRenderShape()!=RenderShape.MODEL || !state.isSolidRender() || !state.isCollisionShapeFullBlock(level,pos)) { skipped++; continue; }
            if(blocks==MAX_BLOCKS || mesh.position()+36*24>128+limit*24) { skipped++; continue; }
            float x=pos.getX()-ox,y=pos.getY()-oy,z=pos.getZ()-oz;
            // Inflate slightly so the overlay wins against its own solid faces.
            float[][] points=new float[8][3];
            for(int i=0;i<8;i++) points[i]=new float[]{x+((i&1)==0?-0.002f:1.002f),y+((i&2)==0?-0.002f:1.002f),z+((i&4)==0?-0.002f:1.002f)};
            float u0=(stage*16+0.5f)/width,u1=(stage*16+15.5f)/width;
            float v0=(blockHeight+0.5f)/height,v1=(blockHeight+15.5f)/height;
            for(var face:FACES) for(int k:TRIANGLES) {
                var p=points[face[k]];
                mesh.putFloat(p[0]).putFloat(p[1]).putFloat(p[2]);
                mesh.putFloat(k==0 || k==3?u0:u1).putFloat(k<2?v0:v1);
                mesh.putInt(-1);
            }
            blocks++; vertices+=36;
            LOG.info("event=SCENE_CRACK snapshot={} x={} y={} z={} stage={}",snapshot,pos.getX(),pos.getY(),pos.getZ(),stage);
        }
        LOG.info("event=SCENE_CRACKS snapshot={} blocks={} vertices={} skipped={}",snapshot,blocks,vertices,skipped);
        return vertices;
    }
}
