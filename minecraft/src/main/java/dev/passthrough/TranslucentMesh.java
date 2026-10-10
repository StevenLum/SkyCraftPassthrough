package dev.passthrough;

import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.util.ArrayList;
import java.util.Comparator;
import net.minecraft.world.phys.Vec3;
import org.slf4j.LoggerFactory;

/** Keeps complete quads together and sorts them far-to-near at publication. */
final class TranslucentMesh {
    private record Quad(int offset,float x,float y,float z) {
        double distance(Vec3 eye) {
            double dx=x-eye.x,dy=y-eye.y,dz=z-eye.z;
            return dx*dx+dy*dy+dz*dz;
        }
    }
    private final ByteBuffer data;
    private final ArrayList<Quad> quads=new ArrayList<>();
    TranslucentMesh(int maxVertices) { data=ByteBuffer.allocateDirect(maxVertices*24).order(ByteOrder.LITTLE_ENDIAN); }
    void clear() { data.clear(); quads.clear(); }
    void truncate(int vertices) {
        if(vertices<0 || vertices%6!=0 || vertices>vertices()) throw new IllegalArgumentException("Invalid translucent rollback");
        quads.subList(vertices/6,quads.size()).clear(); data.position(vertices*24);
    }
    int vertices() { return quads.size()*6; }
    ByteBuffer buffer() { return data; }
    void finishQuad(float x,float y,float z) { quads.add(new Quad(data.position()-6*24,x,y,z)); }
    int append(ByteBuffer mesh,Vec3 eye,long snapshot) {
        long start=System.nanoTime();
        var sorted=new ArrayList<>(quads);
        sorted.sort(Comparator.comparingDouble((Quad q)->q.distance(eye)).reversed());
        var source=data.duplicate();
        for(var quad:sorted) {
            source.clear().position(quad.offset()).limit(quad.offset()+6*24);
            mesh.put(source);
        }
        LoggerFactory.getLogger("PassthroughPractice").info(
            "event=SCENE_TRANSLUCENT snapshot={} quads={} vertices={} sort_copy_us={} eye_relative_x={} eye_relative_y={} eye_relative_z={}",
            snapshot,quads.size(),vertices(),(System.nanoTime()-start)/1000,eye.x,eye.y,eye.z);
        return vertices();
    }
}
