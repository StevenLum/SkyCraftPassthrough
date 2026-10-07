package dev.passthrough;

import java.lang.foreign.Arena;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.Linker;
import java.lang.foreign.SymbolLookup;
import java.lang.foreign.ValueLayout;
import java.lang.invoke.MethodHandle;
import java.nio.file.Path;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;

/** The same Rust DLL supplies the SKSE entry point and the Minecraft bridge. */
public final class NativeLink {
    private final MethodHandle init;
    private final MethodHandle frame;
    private final MethodHandle world;
    public static final int WORLD_SIZE = 128 + 8192 * 48 + 128 * 32;
    private final java.lang.foreign.MemorySegment worldBuffer = Arena.global().allocate(WORLD_SIZE, 8);

    public NativeLink(Path dll) {
        var symbols = SymbolLookup.libraryLookup(dll.toAbsolutePath(), Arena.global());
        var linker = Linker.nativeLinker();
        init = linker.downcallHandle(symbols.find("pt_mc_init").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.JAVA_INT));
        frame = linker.downcallHandle(symbols.find("pt_mc_frame4").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.JAVA_LONG, ValueLayout.JAVA_LONG,
                ValueLayout.JAVA_DOUBLE, ValueLayout.JAVA_DOUBLE, ValueLayout.JAVA_DOUBLE,
                ValueLayout.JAVA_DOUBLE, ValueLayout.JAVA_DOUBLE, ValueLayout.JAVA_DOUBLE, ValueLayout.JAVA_INT));
        world = linker.downcallHandle(symbols.find("pt_mc_world").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.JAVA_INT));
    }
    public int initialize() throws Throwable { return (int)init.invokeExact(); }
    public ByteBuffer receiveWorld() throws Throwable {
        int result = (int)world.invokeExact(worldBuffer, WORLD_SIZE);
        if (result < 0) throw new IllegalStateException("Native world receive failed");
        return result == 0 ? null : worldBuffer.asByteBuffer().order(ByteOrder.LITTLE_ENDIAN);
    }
    public int send(long world, long number, double partial, double x, double y, double z, boolean active) throws Throwable {
        return send(world,number,partial,x,y,z,0.0,0.0,active);
    }
    public int send(long world, long number, double partial, double x, double y, double z, double yaw, double pitch, boolean active) throws Throwable {
        return (int)frame.invokeExact(world, number, partial, x, y, z, yaw, pitch, active ? 1 : 0);
    }
}
