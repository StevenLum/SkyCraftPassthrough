package dev.passthrough;

import java.nio.ByteBuffer;
import java.util.Map;
import net.minecraft.client.Minecraft;
import net.minecraft.client.renderer.item.ItemStackRenderState;
import net.minecraft.client.renderer.texture.TextureAtlasSprite;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.util.RandomSource;
import net.minecraft.world.entity.item.ItemEntity;
import net.minecraft.world.item.ItemDisplayContext;
import org.slf4j.LoggerFactory;

/** Full block drops with a particle-sprite fallback for other items. */
final class DropCapture {
    private static final org.slf4j.Logger LOG=LoggerFactory.getLogger("PassthroughPractice");
    private static final ItemStackRenderState STATE=new ItemStackRenderState();
    private static final RandomSource RANDOM=RandomSource.create(0);
    private static final int[] TRIANGLES={0,1,2,0,2,3};
    private DropCapture() {}

    static int append(ByteBuffer mesh,TranslucentMesh translucent,int limit,int ox,int oy,int oz,int side,
                      Map<TextureAtlasSprite,float[]> sprites,long snapshot) {
        Minecraft mc=Minecraft.getInstance();
        float partial=mc.getDeltaTracker().getGameTimeDeltaPartialTick(false);
        int items=0,skipped=0,scanned=0,attempted=0,errors=0,vertices=0,models=0;
        int initialPosition=mesh.position(),initialTranslucent=translucent.vertices();
        boolean budget=false;
        long deadline=System.nanoTime()+3_000_000L;
        for(var entity:mc.level.entitiesForRendering()) {
            if(scanned>=4096 || attempted>=64 || System.nanoTime()>deadline) { budget=true; break; }
            scanned++;
            if(!(entity instanceof ItemEntity item) || item.isRemoved() || item.getItem().isEmpty()) continue;
            var p=item.getPosition(partial);
            if(p.x<ox || p.x>=ox+side || p.y<oy || p.y>=oy+side || p.z<oz || p.z>=oz+side) continue;
            attempted++;
            if(mesh.position()+(translucent.vertices()+6)*24>128+limit*24) { skipped++; continue; }
            int start=mesh.position();
            int transStart=translucent.vertices();
            try {
                float age=item.getAge()+partial;
                float spin=ItemEntity.getSpin(age,item.bobOffs);
                float bob=(float)Math.sin(age/10.0f+item.bobOffs)*0.1f+0.1f;
                float x=(float)(p.x-ox),y=(float)(p.y-oy)+bob+0.25f,z=(float)(p.z-oz);
                int added=BlockDropCapture.append(mesh,translucent,limit,item.getItem(),sprites,x,y-0.23f,z,spin);
                boolean model=added>0;
                if(!model) {
                STATE.clear();
                mc.getItemModelResolver().updateForNonLiving(STATE,item.getItem(),ItemDisplayContext.GROUND,item);
                RANDOM.setSeed(0);
                var material=STATE.pickParticleMaterial(RANDOM);
                float[] uv=material==null?null:sprites.get(material.sprite());
                if(uv==null) { skipped++; continue; }
                float dx=(float)Math.cos(spin)*0.25f,dz=(float)Math.sin(spin)*0.25f;
                float[][] corners={{x-dx,y+0.25f,z-dz},{x+dx,y+0.25f,z+dz},
                    {x+dx,y-0.25f,z+dz},{x-dx,y-0.25f,z-dz}};
                for(int k:TRIANGLES) {
                    var v=corners[k];
                    mesh.putFloat(v[0]).putFloat(v[1]).putFloat(v[2]);
                    mesh.putFloat(k==0 || k==3?uv[0]:uv[2]).putFloat(k<2?uv[1]:uv[3]);
                    mesh.putInt(-1);
                }
                added=6;
                }
                items++; vertices+=added; if(model) models++;
                LOG.info("event=SCENE_ITEM snapshot={} entity={} item={} stack={} partial={} x={} y={} z={} bob={} spin_rad={} geometry={} vertices={} translucent_vertices={}",
                    snapshot,item.getId(),BuiltInRegistries.ITEM.getKey(item.getItem().getItem()),item.getItem().getCount(),partial,p.x,p.y,p.z,bob,spin,model?"block_model":"icon",added,translucent.vertices()-transStart);
            } catch(RuntimeException error) {
                mesh.position(start); translucent.truncate(transStart); errors++;
                LOG.warn("event=SCENE_ITEM_ERROR snapshot={} entity={} type={}",snapshot,item.getId(),error.getClass().getSimpleName());
            }
        }
        int transparent=translucent.vertices()-initialTranslucent;
        mesh.putInt(104,items).putInt(112,models).putInt(116,transparent);
        LOG.info("event=SCENE_ITEMS snapshot={} items={} vertices={} models={} icons={} skipped={} errors={} scanned={} budget_limited={}",snapshot,items,vertices,models,items-models,skipped,errors,scanned,budget);
        return (mesh.position()-initialPosition)/24;
    }
}
