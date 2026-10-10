package dev.passthrough;

import com.mojang.blaze3d.vertex.QuadInstance;
import dev.passthrough.mixin.AtlasAccess;
import dev.passthrough.mixin.SpriteAccess;
import java.lang.foreign.MemorySegment;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import net.minecraft.client.Minecraft;
import net.minecraft.client.model.geom.builders.UVPair;
import net.minecraft.client.renderer.block.BlockQuadOutput;
import net.minecraft.client.renderer.block.ModelBlockRenderer;
import net.minecraft.client.resources.model.geometry.BakedQuad;
import net.minecraft.core.BlockPos;
import net.minecraft.data.AtlasIds;
import net.minecraft.world.level.block.RenderShape;
import org.slf4j.LoggerFactory;

/** Nearby static block models. Uses Minecraft's tessellator; no game data is saved. */
public final class SceneCapture implements BlockQuadOutput {
    private static final org.slf4j.Logger LOG=LoggerFactory.getLogger("PassthroughPractice");
    private static final SceneCapture INSTANCE=new SceneCapture();
    private static final int HEADER=128, LIMIT=262140, SIDE=16;
    private final ByteBuffer mesh=ByteBuffer.allocateDirect(HEADER+LIMIT*24).order(ByteOrder.LITTLE_ENDIAN);
    private final TranslucentMesh translucent=new TranslucentMesh(LIMIT);
    private int staticTranslucent=-1;
    private ByteBuffer atlas;
    private Object sprites,itemSprites;
    private final java.util.Map<net.minecraft.client.renderer.texture.TextureAtlasSprite,float[]> spriteRects=new java.util.IdentityHashMap<>();
    private long world=-1,atlasId,sequence,lastStart,started;
    private int cursor=-1,count,ox,oy,oz,skipped,atlasWidth,blockWidth,blockHeight,atlasHeight,crackY;
    private boolean failed,atlasSent;
    private ModelBlockRenderer renderer;
    private net.minecraft.client.renderer.block.FluidRenderer fluidRenderer;
    private final FluidCapture fluids=new FluidCapture(this::fluidQuad);
    private int fluidCells,waterCells,lavaCells,fluidOpaque,fluidTransparent;
    private SceneCapture() {}
    public static void frame(boolean active) {
        try { INSTANCE.capture(active); }
        catch(Throwable error) { INSTANCE.failed=true; LOG.error("event=SCENE_DISABLED; hands/HUD and movement remain active",error); }
    }
    private static ByteBuffer header(ByteBuffer b,int kind,long world,long sequence,long atlas,int count,int height,int bytes) {
        b.clear(); for(int i=0;i<HEADER;i++) b.put((byte)0);
        b.put(0,(byte)'P').put(1,(byte)'T').put(2,(byte)'S').put(3,(byte)'C').put(4,(byte)'E').put(5,(byte)'0').put(6,(byte)'1').put(7,(byte)'2');
        b.putLong(16,world).putLong(24,sequence).putInt(40,kind).putInt(44,count).putInt(48,bytes).putInt(52,height).putLong(64,atlas);
        return b;
    }
    private void atlas(AtlasAccess access,AtlasAccess items) throws java.io.IOException {
        int w=access.passthrough$width(),h=access.passthrough$height();
        int iw=items.passthrough$width(),ih=items.passthrough$height();
        if(w<=0 || h<=0 || iw<=0 || ih<=0 || w>4096 || h>4096 || iw>4096 || ih>4096) throw new IllegalStateException("Invalid source atlas dimensions");
        int ix=0,iy=h;
        atlasWidth=Math.max(160,Math.max(w,iw)); crackY=h+ih;
        if(crackY+CrackCapture.STRIP_HEIGHT>4096) { ix=w; iy=0; atlasWidth=Math.max(160,w+iw); crackY=Math.max(h,ih); }
        atlasHeight=crackY+CrackCapture.STRIP_HEIGHT;
        if(atlasWidth>4096 || atlasHeight>4096) throw new IllegalStateException("Combined block/item/crack atlas exceeds 4096x4096");
        blockWidth=w; blockHeight=h;
        atlas=header(ByteBuffer.allocateDirect(HEADER+atlasWidth*atlasHeight*4).order(ByteOrder.LITTLE_ENDIAN),2,world,++atlasId,atlasId,atlasWidth,atlasHeight,atlasWidth*atlasHeight*4);
        spriteRects.clear(); copyAtlas(access,0,0); copyAtlas(items,ix,iy);
        CrackCapture.copyTextures(atlas,atlasWidth,crackY);
        sprites=access.passthrough$sprites(); itemSprites=items.passthrough$sprites(); atlasSent=false; cursor=-1;
        LOG.info("event=SCENE_ATLAS_BUILT atlas={} width={} height={} bytes={} item_offset_x={} item_offset_y={}",atlasId,atlasWidth,atlasHeight,atlasWidth*atlasHeight*4,ix,iy);
    }
    private void copyAtlas(AtlasAccess access,int offsetX,int offsetY) {
        int w=access.passthrough$width(),h=access.passthrough$height();
        for(var sprite:access.passthrough$sprites().values()) {
            var image=((SpriteAccess)sprite.contents()).passthrough$image();
            int sw=Math.min(sprite.contents().width(),image.getWidth()),sh=Math.min(sprite.contents().height(),image.getHeight());
            int x0=Math.round(sprite.getU0()*w),y0=Math.round(sprite.getV0()*h);
            for(int y=0;y<sh;y++) for(int x=0;x<sw;x++) {
                int tx=x0+x,ty=y0+y; if(tx<0 || ty<0 || tx>=w || ty>=h) continue;
                int c=image.getPixel(x,y),at=HEADER+((ty+offsetY)*atlasWidth+tx+offsetX)*4;
                atlas.put(at,(byte)(c>>16)).put(at+1,(byte)(c>>8)).put(at+2,(byte)c).put(at+3,(byte)(c>>>24));
            }
            spriteRects.put(sprite,new float[]{(x0+offsetX+0.5f)/atlasWidth,(y0+offsetY+0.5f)/atlasHeight,
                (x0+offsetX+sw-0.5f)/atlasWidth,(y0+offsetY+sh-0.5f)/atlasHeight});
        }
    }
    private void capture(boolean active) throws Throwable {
        if(failed) return;
        if(!active) { cursor=-1; return; }
        NativeLink link=PassthroughClient.overlayLink(); if(link==null || link.overlayReady()==0) { cursor=-1; return; }
        Minecraft mc=Minecraft.getInstance();
        if(world!=PassthroughClient.worldId()) { world=PassthroughClient.worldId(); sprites=null; cursor=-1; }
        var access=(AtlasAccess)mc.getAtlasManager().getAtlasOrThrow(AtlasIds.BLOCKS);
        var items=(AtlasAccess)mc.getAtlasManager().getAtlasOrThrow(AtlasIds.ITEMS);
        if(sprites!=access.passthrough$sprites() || itemSprites!=items.passthrough$sprites()) {
            atlas(access,items); renderer=new ModelBlockRenderer(mc.options.ambientOcclusion().get(),true,mc.getBlockColors());
            fluidRenderer=new net.minecraft.client.renderer.block.FluidRenderer(mc.getModelManager().getFluidStateModelSet());
        }
        if(!atlasSent) {
            int result=link.scene(MemorySegment.ofBuffer(atlas.duplicate().clear()),atlas.capacity());
            if(result<0) throw new IllegalStateException("Atlas publication failed");
            if(result==0) return; atlasSent=true;
        }
        long now=System.nanoTime();
        if(cursor<0) {
            if(now-lastStart<100_000_000L) return;
            lastStart=started=now; cursor=0; count=skipped=0;
            fluidCells=waterCells=lavaCells=fluidOpaque=fluidTransparent=0;
            translucent.clear();
            staticTranslucent=-1;
            ox=mc.player.getBlockX()-SIDE/2; oy=mc.player.getBlockY()-SIDE/2; oz=mc.player.getBlockZ()-SIDE/2;
            header(mesh,1,world,++sequence,atlasId,0,0,0);
            mesh.putDouble(72,ox).putDouble(80,oy).putDouble(88,oz);
        }
        long deadline=now+3_000_000L;
        while(cursor<SIDE*SIDE*SIDE && System.nanoTime()<deadline) {
            int i=cursor++,x=i%SIDE,z=(i/SIDE)%SIDE,y=i/(SIDE*SIDE);
            BlockPos pos=new BlockPos(ox+x,oy+y,oz+z);
            if(!mc.level.hasChunkAt(pos)) continue;
            var state=mc.level.getBlockState(pos);
            var fluid=state.getFluidState();
            if(!fluid.isEmpty()) {
                fluids.begin(pos,ox,oy,oz);
                fluidRenderer.tesselate(mc.level,pos,fluids,state,fluid);
                fluids.finish(); fluidCells++;
                if(fluid.is(net.minecraft.tags.FluidTags.WATER)) waterCells++;
                if(fluid.is(net.minecraft.tags.FluidTags.LAVA)) lavaCells++;
            }
            if(state.getRenderShape()!=RenderShape.MODEL) continue;
            var model=mc.getModelManager().getBlockStateModelSet().get(state);
            renderer.tesselateBlock(this,x,y,z,mc.level,pos,state,model,state.getSeed(pos));
        }
        if(System.nanoTime()-started>1_000_000_000L) { cursor=-1; LOG.warn("event=SCENE_BUILD_DROPPED reason=age"); return; }
        if(cursor==SIDE*SIDE*SIDE) {
            // Replace dynamic geometry on publication retries; never append twice.
            mesh.position(HEADER+count*24);
            if(staticTranslucent<0) staticTranslucent=translucent.vertices();
            translucent.truncate(staticTranslucent);
            int drops=DropCapture.append(mesh,translucent,LIMIT,ox,oy,oz,SIDE,spriteRects,sequence);
            var eye=mc.player.getEyePosition(mc.getDeltaTracker().getGameTimeDeltaPartialTick(false)).subtract(ox,oy,oz);
            int transparent=translucent.append(mesh,eye,sequence);
            int cracks=CrackCapture.append(mesh,LIMIT,ox,oy,oz,SIDE,atlasWidth,crackY,atlasHeight,sequence);
            int total=count+drops+transparent+cracks;
            mesh.putInt(44,total).putInt(48,total*24).putInt(96,cracks).putInt(100,drops).putInt(108,transparent);
            mesh.putInt(120,fluidOpaque).putInt(124,fluidTransparent);
            var data=mesh.duplicate().position(0).limit(HEADER+total*24);
            int result=link.scene(MemorySegment.ofBuffer(data),data.remaining());
            if(result<0) throw new IllegalStateException("Mesh publication failed");
            LOG.info("event=SCENE_FLUID snapshot={} cells={} water_cells={} lava_cells={} opaque_vertices={} translucent_vertices={}",sequence,fluidCells,waterCells,lavaCells,fluidOpaque,fluidTransparent);
            LOG.info("event=SCENE_CAPTURE snapshot={} vertices={} crack_vertices={} translucent_quads_skipped={} build_us={} sent={}",sequence,total,cracks,skipped,(System.nanoTime()-started)/1000,result);
            LOG.info("event=SCENE_BOUNDS snapshot={} min_x={} min_y={} min_z={} max_x_exclusive={} max_y_exclusive={} max_z_exclusive={} player_x={} player_y={} player_z={} aim={}",
                sequence,ox,oy,oz,ox+SIDE,oy+SIDE,oz+SIDE,mc.player.getX(),mc.player.getY(),mc.player.getZ(),
                mc.hitResult instanceof net.minecraft.world.phys.BlockHitResult hit && hit.getType()==net.minecraft.world.phys.HitResult.Type.BLOCK ? hit.getBlockPos().toShortString() : "no_block");
            if(result>0) cursor=-1;
        }
    }
    private void fluidQuad(float[] v,int[] colors,int[] lights,boolean transparent) {
        if(count+translucent.vertices()+6>LIMIT) throw new IllegalStateException("Fluid mesh exceeds combined vertex limit");
        var target=transparent?translucent.buffer():mesh;
        for(int k:new int[]{0,1,2,0,2,3}) {
            int at=k*5,c=colors[k],light=lights[k];
            float brightness=0.2f+0.8f*Math.max((light>>4)&15,(light>>20)&15)/15.0f;
            target.putFloat(v[at]).putFloat(v[at+1]).putFloat(v[at+2]);
            target.putFloat(v[at+3]*blockWidth/atlasWidth).putFloat(v[at+4]*blockHeight/atlasHeight);
            target.put((byte)(((c>>16)&255)*brightness)).put((byte)(((c>>8)&255)*brightness)).put((byte)((c&255)*brightness)).put((byte)(c>>>24));
        }
        if(transparent) {
            float x=0,y=0,z=0;
            for(int k=0;k<4;k++) { x+=v[k*5]/4; y+=v[k*5+1]/4; z+=v[k*5+2]/4; }
            translucent.finishQuad(x,y,z); fluidTransparent+=6;
        } else { count+=6; fluidOpaque+=6; }
    }
    @Override public void put(float x,float y,float z,BakedQuad quad,QuadInstance instance) {
        boolean transparent=quad.materialInfo().layer().translucent();
        if(count+translucent.vertices()+6>LIMIT) throw new IllegalStateException("Nearby mesh exceeds vertex limit; incomplete mesh not published");
        var target=transparent?translucent.buffer():mesh;
        for(int k:new int[]{0,1,2,0,2,3}) {
            var p=quad.position(k); long uv=quad.packedUV(k); int c=instance.getColor(k);
            int light=instance.getLightCoordsWithEmission(k,quad.materialInfo().lightEmission());
            float brightness=0.2f+0.8f*Math.max((light>>4)&15,(light>>20)&15)/15.0f;
            target.putFloat(p.x()+x).putFloat(p.y()+y).putFloat(p.z()+z).putFloat(UVPair.unpackU(uv)*blockWidth/atlasWidth).putFloat(UVPair.unpackV(uv)*blockHeight/atlasHeight);
            target.put((byte)(((c>>16)&255)*brightness)).put((byte)(((c>>8)&255)*brightness)).put((byte)((c&255)*brightness)).put((byte)(c>>>24));
            if(!transparent) count++;
        }
        if(transparent) {
            float cx=x,cy=y,cz=z;
            for(int k=0;k<4;k++) { var p=quad.position(k); cx+=p.x()/4; cy+=p.y()/4; cz+=p.z()/4; }
            translucent.finishQuad(cx,cy,cz);
        }
    }
}
