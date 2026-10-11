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
    private final MethodHandle overlayReady;
    private final MethodHandle overlay;
    private final MethodHandle scene;
    private final MethodHandle menu;
    private final MethodHandle switches;
    private final java.lang.foreign.MemorySegment menuBuffer=Arena.global().allocate(32,8);
    public static final int WORLD_HEADER=144;
    public static final int WORLD_SIZE = WORLD_HEADER + 8192 * 48 + 128 * 32 + 8 + 48 + 528;
    /** u32: what Skyrim's crosshair is on (0 nothing, 1 other, 2 container, 3 living, 4 body, 5 door, 6 enemy). */
    public static final int TARGET_KIND_AT = WORLD_SIZE - 48 - 528 - 8;
    private final java.lang.foreign.MemorySegment worldBuffer = Arena.global().allocate(WORLD_SIZE, 8);

    public NativeLink(Path dll) {
        var symbols = SymbolLookup.libraryLookup(dll.toAbsolutePath(), Arena.global());
        var linker = Linker.nativeLinker();
        init = linker.downcallHandle(symbols.find("pt_mc_init").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.JAVA_INT));
        frame = linker.downcallHandle(symbols.find("pt_mc_frame15").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.JAVA_LONG, ValueLayout.JAVA_LONG,
                ValueLayout.JAVA_DOUBLE, ValueLayout.JAVA_DOUBLE, ValueLayout.JAVA_DOUBLE,
                ValueLayout.JAVA_DOUBLE, ValueLayout.JAVA_DOUBLE, ValueLayout.JAVA_DOUBLE, ValueLayout.JAVA_INT, ValueLayout.JAVA_LONG, ValueLayout.JAVA_LONG, ValueLayout.JAVA_LONG, ValueLayout.JAVA_LONG, ValueLayout.JAVA_LONG, ValueLayout.JAVA_LONG, ValueLayout.JAVA_INT, ValueLayout.JAVA_LONG, ValueLayout.JAVA_LONG, ValueLayout.JAVA_LONG, ValueLayout.JAVA_LONG, ValueLayout.JAVA_INT, ValueLayout.JAVA_LONG, ValueLayout.JAVA_INT, ValueLayout.JAVA_FLOAT, ValueLayout.JAVA_LONG, ValueLayout.JAVA_FLOAT, ValueLayout.JAVA_FLOAT, ValueLayout.JAVA_LONG, ValueLayout.JAVA_INT, ValueLayout.JAVA_INT));
        menu=linker.downcallHandle(symbols.find("pt_mc_menu").orElseThrow(),FunctionDescriptor.of(ValueLayout.JAVA_INT,ValueLayout.ADDRESS));
        switches=linker.downcallHandle(symbols.find("pt_mc_switches").orElseThrow(),FunctionDescriptor.of(ValueLayout.JAVA_LONG));
        world = linker.downcallHandle(symbols.find("pt_mc_world").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.JAVA_INT));
        scene=linker.downcallHandle(symbols.find("pt_mc_scene").orElseThrow(),FunctionDescriptor.of(ValueLayout.JAVA_INT,ValueLayout.ADDRESS,ValueLayout.JAVA_INT));
        overlayReady=linker.downcallHandle(symbols.find("pt_mc_overlay_ready").orElseThrow(),FunctionDescriptor.of(ValueLayout.JAVA_LONG));
        overlay=linker.downcallHandle(symbols.find("pt_mc_overlay").orElseThrow(),FunctionDescriptor.of(ValueLayout.JAVA_INT,
            ValueLayout.JAVA_LONG,ValueLayout.JAVA_LONG,ValueLayout.JAVA_LONG,ValueLayout.JAVA_INT,ValueLayout.JAVA_INT,ValueLayout.ADDRESS,ValueLayout.JAVA_INT));
    }
    public int initialize() throws Throwable { return (int)init.invokeExact(); }
    /** Steps switched off in switches.txt, as bits; see Switches. Needs no connection. */
    public long switches() throws Throwable { return (long)switches.invokeExact(); }
    public ByteBuffer menu() throws Throwable {
        int result=(int)menu.invokeExact(menuBuffer);
        if(result<0) throw new IllegalStateException("Native menu status failed");
        return result==0 ? null : menuBuffer.asByteBuffer().order(ByteOrder.LITTLE_ENDIAN);
    }
    public int scene(java.lang.foreign.MemorySegment bytes,int size) throws Throwable { return (int)scene.invokeExact(bytes,size); }
    public long overlayReady() throws Throwable { return (long)overlayReady.invokeExact(); }
    public int overlay(long world,long frame,long captured,int width,int height,java.lang.foreign.MemorySegment pixels,int size) throws Throwable {
        return (int)overlay.invokeExact(world,frame,captured,width,height,pixels,size);
    }
    public ByteBuffer receiveWorld() throws Throwable {
        int result = (int)world.invokeExact(worldBuffer, WORLD_SIZE);
        if (result < 0) throw new IllegalStateException("Native world receive failed");
        return result == 0 ? null : worldBuffer.asByteBuffer().order(ByteOrder.LITTLE_ENDIAN);
    }
    private static int controls(boolean active) {
        var mc=net.minecraft.client.Minecraft.getInstance();
        if(!active || mc.player==null || !mc.isWindowActive() || mc.gui.screen()!=null || SkyrimMenus.active()) return 0;
        boolean move=mc.options.keyUp.isDown() || mc.options.keyDown.isDown()
            || mc.options.keyLeft.isDown() || mc.options.keyRight.isDown();
        return BridgeRules.controlFlags(true,move,mc.player.getAbilities().mayfly);
    }
    public int send(long world, long number, double partial, double x, double y, double z, boolean active) throws Throwable {
        return send(world,number,partial,x,y,z,0.0,0.0,active);
    }
    public int send(long world, long number, double partial, double x, double y, double z, double yaw, double pitch, boolean active) throws Throwable {
        var id=active ? BridgePairing.expected(net.minecraft.client.Minecraft.getInstance().getSingleplayerServer()) : new long[2];
        var mcId=active ? BridgePairing.worldId(net.minecraft.client.Minecraft.getInstance().getSingleplayerServer()) : new long[2];
        var command=SkyrimMenus.frame(active);
        var damage=BridgeDamage.status(active);
        var attack=SkyrimAttack.frame(active);
        var cursor=SkyrimMenus.cursor(active);
        var death=BridgeDeath.frame(active);
        return (int)frame.invokeExact(world, number, partial, x, y, z, yaw, pitch, active ? 1 : 0,id[0],id[1],mcId[0],mcId[1],SkyrimInteraction.frame(active),command.sequence(),command.action(),command.context(),BridgeRecovery.acknowledgement(active),damage.epoch(),damage.ack(),damage.ready(),attack.sequence(),attack.handle(),attack.damage(),cursor.context(),cursor.x(),cursor.y(),death.sequence(),death.ageMs(),controls(active));
    }
}
