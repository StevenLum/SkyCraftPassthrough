package dev.passthrough;

import com.mojang.blaze3d.vertex.VertexConsumer;
import net.minecraft.client.renderer.block.FluidRenderer;
import net.minecraft.client.renderer.chunk.ChunkSectionLayer;
import net.minecraft.core.BlockPos;

/** Adapts FluidRenderer's complete vertices; never extracts or saves game assets. */
final class FluidCapture implements FluidRenderer.Output,VertexConsumer {
    interface Sink { void quad(float[] xyzuv,int[] colors,int[] lights,boolean translucent); }
    private final Sink sink;
    private final float[] xyzuv=new float[20];
    private final int[] colors=new int[4],lights=new int[4];
    private int count,dx,dy,dz;
    private boolean translucent;
    FluidCapture(Sink sink) { this.sink=sink; }
    void begin(BlockPos pos,int ox,int oy,int oz) {
        if(count!=0) throw new IllegalStateException("Incomplete fluid quad");
        // FluidRenderer emits positions within a 16-block section, including at negative coordinates.
        dx=(pos.getX() & ~15)-ox; dy=(pos.getY() & ~15)-oy; dz=(pos.getZ() & ~15)-oz;
    }
    void finish() { if(count!=0) throw new IllegalStateException("Incomplete fluid quad"); }
    @Override public VertexConsumer getBuilder(ChunkSectionLayer layer) {
        finish(); translucent=layer.translucent(); return this;
    }
    @Override public void addVertex(float x,float y,float z,int color,float u,float v,int overlay,int light,float nx,float ny,float nz) {
        int at=count*5;
        xyzuv[at]=x+dx; xyzuv[at+1]=y+dy; xyzuv[at+2]=z+dz; xyzuv[at+3]=u; xyzuv[at+4]=v;
        colors[count]=color; lights[count]=light;
        if(++count==4) { count=0; sink.quad(xyzuv,colors,lights,translucent); }
    }
    private UnsupportedOperationException unsupported() { return new UnsupportedOperationException("Fluid renderer used unsupported incremental vertices"); }
    @Override public VertexConsumer addVertex(float x,float y,float z) { throw unsupported(); }
    @Override public VertexConsumer setColor(int r,int g,int b,int a) { throw unsupported(); }
    @Override public VertexConsumer setColor(int argb) { throw unsupported(); }
    @Override public VertexConsumer setUv(float u,float v) { throw unsupported(); }
    @Override public VertexConsumer setUv1(int u,int v) { throw unsupported(); }
    @Override public VertexConsumer setUv2(int u,int v) { throw unsupported(); }
    @Override public VertexConsumer setUv3(float u,float v) { throw unsupported(); }
    @Override public VertexConsumer setNormal(float x,float y,float z) { throw unsupported(); }
    @Override public VertexConsumer setLineWidth(float width) { throw unsupported(); }
}
