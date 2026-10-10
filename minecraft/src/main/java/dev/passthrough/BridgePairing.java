package dev.passthrough;

import java.nio.ByteBuffer;
import java.nio.file.*;
import java.nio.charset.StandardCharsets;
import java.util.*;
import net.minecraft.client.Minecraft;
import net.minecraft.client.server.IntegratedServer;
import net.minecraft.client.gui.screens.ConfirmScreen;
import net.minecraft.network.chat.Component;
import net.minecraft.world.level.storage.LevelResource;
import org.slf4j.LoggerFactory;

/** Persistent lineage pairing and messages for native admission/alignment. */
public final class BridgePairing {
    private static final Map<IntegratedServer,State> states=new WeakHashMap<>();
    private static final String ZERO="0".repeat(32);
    private static final class State {
        String paired,incoming,lastStatus,asked;
        boolean invalid,prompt;
        int originBlock;
    }
    private BridgePairing() {}
    private static Path path(IntegratedServer server) {
        return server.getWorldPath(LevelResource.ROOT).normalize().resolve("passthrough-pairing.txt");
    }
    private static State read(IntegratedServer server) {
        var state=new State(); var path=path(server);
        try {
            if(Files.exists(path)) {
                if(Files.size(path)>128) throw new IllegalArgumentException("Pairing file too large");
                String text=Files.readString(path,StandardCharsets.US_ASCII);
                if(!text.matches("PTPAIR1\n[0-9A-F]{32}\n")) throw new IllegalArgumentException("Invalid pairing record");
                state.paired=text.substring(8,40);
                if(state.paired.equals(ZERO)) throw new IllegalArgumentException("Empty pairing ID");
            }
        } catch(Exception error) {
            state.invalid=true;
            LoggerFactory.getLogger("PassthroughPractice").error("event=SCENE_PAIRING_READ_FAILED",error);
        }
        return state;
    }
    public static long[] worldId(IntegratedServer server) {
        if(server==null) return new long[2];
        String folder=path(server).getParent().getFileName().toString();
        try {
            if(!folder.startsWith("skyrim-bridge-")) return new long[2];
            String text=folder.substring("skyrim-bridge-".length());
            var id=UUID.fromString(text);
            if(!id.toString().equalsIgnoreCase(text)) return new long[2];
            return new long[]{id.getMostSignificantBits(),id.getLeastSignificantBits()};
        } catch(IllegalArgumentException e) { return new long[2]; }
    }
    public static long[] expected(IntegratedServer server) {
        if(server==null || !BridgeWorld.marked(server)) return new long[2];
        var state=states.computeIfAbsent(server,BridgePairing::read);
        if(state.invalid || state.paired==null) return new long[2];
        var b=ByteBuffer.wrap(HexFormat.of().parseHex(state.paired)).order(java.nio.ByteOrder.LITTLE_ENDIAN);
        return new long[]{b.getLong(),b.getLong()};
    }
    public static void observe(ByteBuffer buffer,IntegratedServer server) {
        if(!BridgeWorld.marked(server)) return;
        var mc=Minecraft.getInstance();
        if(mc.getSingleplayerServer()!=server) return;
        var state=states.computeIfAbsent(server,BridgePairing::read);
        byte[] bytes=new byte[16];
        buffer.get(NativeLink.WORLD_SIZE-16,bytes);
        String id=HexFormat.of().withUpperCase().formatHex(bytes);
        state.incoming=id;
        int reason=buffer.getInt(124);
        if(reason!=state.originBlock) {
            state.originBlock=reason;
            String message=switch(reason) {
                case 0 -> "Saved origin is ready.";
                case 1 -> "The Minecraft save folder no longer has its bridge world ID.";
                case 2 -> "The saved origin is invalid or unavailable. Load a valid Skyrim save.";
                case 3 -> "This origin belongs to a different Minecraft world. Open its matching world.";
                case 4 -> "This is a different Skyrim location. Return to the saved practice location.";
                case 5 -> "Moving to the matching Minecraft position for this Skyrim place. If this remains held, stop and check the bridge logs.";
                case 7 -> "You died. Waiting for Skyrim to reload its last save.";
                default -> "Origin state is unavailable. See the bridge logs.";
            };
            LoggerFactory.getLogger("PassthroughPractice").info("event=SCENE_ORIGIN_STATUS reason={} message={}",reason,message);
            if(mc.player!=null) mc.player.sendSystemMessage(Component.literal("Skyrim bridge: "+message));
        }
        String status=state.invalid?"invalid_metadata":id.equals(ZERO)?"missing_skyrim_identity":
            state.paired==null?"unpaired":state.paired.equals(id)?"matched":"mismatch";
        String key=status+":"+id;
        if(!key.equals(state.lastStatus)) {
            state.lastStatus=key;
            LoggerFactory.getLogger("PassthroughPractice").info(
                "event=SCENE_PAIRING state={} save={} expected={} received={}",status,
                path(server).getParent().getFileName(),state.paired,id);
            if(mc.player!=null && (status.equals("mismatch") || status.equals("invalid_metadata")))
                mc.player.sendSystemMessage(Component.literal("Skyrim bridge pairing warning: "+status+
                    ". Stop bridge play and load the matching Skyrim/Minecraft saves. Bridge control is blocked."));
            if(mc.player!=null && status.equals("missing_skyrim_identity"))
                mc.player.sendSystemMessage(Component.literal("Save Skyrim through SKSE before pairing this bridge."));
        }
        if(!status.equals("unpaired") || state.prompt || id.equals(state.asked) || mc.gui.screen()!=null) return;
        state.prompt=true; state.asked=id;
        mc.gui.setScreen(new ConfirmScreen(yes->{
            state.prompt=false;
            if(yes && mc.getSingleplayerServer()==server && id.equals(state.incoming) && !server.isPublished()) {
                try {
                    // CREATE_NEW never overwrites an existing binding, including concurrent creation.
                    Files.writeString(path(server),"PTPAIR1\n"+id+"\n",StandardCharsets.US_ASCII,
                        StandardOpenOption.CREATE_NEW,StandardOpenOption.WRITE);
                    state.paired=id; state.lastStatus=null;
                    LoggerFactory.getLogger("PassthroughPractice").info(
                        "event=SCENE_PAIRING state=paired save={} id={}",path(server).getParent().getFileName(),id);
                } catch(Exception error) {
                    state.invalid=true;
                    LoggerFactory.getLogger("PassthroughPractice").error("event=SCENE_PAIRING_WRITE_FAILED",error);
                    if(mc.player!=null) mc.player.sendSystemMessage(Component.literal("Bridge pairing could not be saved. Reopen this world to retry; see logs."));
                }
            } else {
                LoggerFactory.getLogger("PassthroughPractice").info("event=SCENE_PAIRING state=cancelled_or_changed");
            }
            mc.gui.setScreen(null);
        },Component.literal("Pair this bridge with Skyrim?"),
            Component.literal("World: "+path(server).getParent().getFileName()+"\nSkyrim ID: "+id+
                "\nRemember this pairing for future checks? Bridge control requires matching IDs. Save both games at the same practice location to preserve alignment."),
            Component.literal("Pair"),Component.literal("Not now")));
    }
}
