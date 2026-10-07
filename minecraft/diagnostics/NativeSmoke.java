import dev.passthrough.NativeLink;
import java.nio.file.Path;

/** Synthetic 60-FPS samples: checks Java -> native DLL -> another Windows process. */
public final class NativeSmoke {
    public static void main(String[] args) throws Throwable {
        var link = new NativeLink(Path.of(args[0]));
        if (link.initialize() != 1) throw new AssertionError("native initialization failed");
        for (long frame=1; frame<=180; frame++) {
            // Three render samples per 20-Hz tick, including intermediate positions.
            long tick=(frame-1)/3;
            double partial=((frame-1)%3)/3.0;
            double x=12+tick*0.1+partial*0.1;
            double y=64+partial*0.125;
            double z=-5-tick*0.2-partial*0.2;
            if (link.send(1,frame,partial,x,y,z,true) != 1) throw new AssertionError("send failed at frame "+frame);
            Thread.sleep(16);
        }
        System.out.println("PASS: Java native smoke publisher sent 180 synthetic render samples.");
    }
}
