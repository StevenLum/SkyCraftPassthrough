package dev.passthrough;

import java.util.*;
import net.minecraft.client.Minecraft;
import net.minecraft.client.gui.screens.Screen;
import net.minecraft.client.server.IntegratedServer;
import net.minecraft.core.HolderSet;
import net.minecraft.core.registries.Registries;
import net.minecraft.network.chat.Component;
import net.minecraft.world.Difficulty;
import net.minecraft.world.entity.player.Player;
import net.minecraft.world.level.*;
import net.minecraft.world.level.biome.Biomes;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.levelgen.*;
import net.minecraft.world.level.levelgen.flat.*;
import net.minecraft.world.level.levelgen.presets.WorldPresets;
import net.minecraft.world.phys.*;
import org.slf4j.LoggerFactory;

/** Only new marked all-air saves can control Skyrim. Never converts existing worlds. */
public final class BridgeWorld {
    private static final String MARKER="Skyrim Bridge [v1] ";
    private static final Map<IntegratedServer,Boolean> saves=new WeakHashMap<>();
    private static final Map<Player,Boolean> armed=new WeakHashMap<>();
    private static final Map<Player,String> reasons=new WeakHashMap<>();
    private static final Map<Player,Long> heldSince=new WeakHashMap<>(),reportAt=new WeakHashMap<>();
    private static Object notifiedLevel;
    private BridgeWorld() {}
    public static boolean isBridgeName(String name) { return name.startsWith(MARKER); }
    public static void create(Screen parent) {
        var mc=Minecraft.getInstance(); String id="skyrim-bridge-"+UUID.randomUUID();
        var settings=new LevelSettings(MARKER+java.time.LocalDateTime.now().withNano(0),GameType.SURVIVAL,
            new LevelSettings.DifficultySettings(Difficulty.NORMAL,false,false),true,WorldDataConfiguration.DEFAULT);
        mc.createWorldOpenFlows().createFreshLevel(id,settings,new WorldOptions(new Random().nextLong(),false,false),lookup->{
            var flat=new FlatLevelGeneratorSettings(Optional.of(HolderSet.direct()),
                lookup.lookupOrThrow(Registries.BIOME).getOrThrow(Biomes.THE_VOID),List.of());
            flat.getLayersInfo().add(new FlatLayerInfo(128,Blocks.AIR)); flat.updateLayers();
            return WorldPresets.createNormalWorldDimensions(lookup).replaceOverworldGenerator(lookup,new FlatLevelSource(flat));
        },parent);
    }
    public static boolean signature(FlatLevelSource flat) {
        var s=flat.settings();
        var flags=(dev.passthrough.mixin.FlatSettingsAccess)(Object)s;
        return s.getBiome().is(Biomes.THE_VOID) && s.getLayers().size()==128
            && s.getLayers().stream().allMatch(b->b==null || b.isAir())
            && s.structureOverrides().isPresent() && s.structureOverrides().get().size()==0
            && !flags.passthrough$decoration() && !flags.passthrough$lakes();
    }
    public static synchronized boolean marked(IntegratedServer server) {
        if(server==null || server.isPublished() || server.overworld()==null) return false;
        return saves.computeIfAbsent(server,s->{
            var level=s.overworld();
            boolean valid=level!=null && s.getWorldData().getLevelName().startsWith(MARKER)
                && level.getChunkSource().getGenerator() instanceof FlatLevelSource flat && signature(flat);
            LoggerFactory.getLogger("PassthroughPractice").info("event=SCENE_BRIDGE_WORLD name={} eligible={} generator=marked_air128_void",s.getWorldData().getLevelName(),valid);
            return valid;
        });
    }
    public static boolean eligible(Level level) {
        var mc=Minecraft.getInstance(); var server=mc.getSingleplayerServer();
        return level!=null && level.dimension().equals(Level.OVERWORLD) && marked(server)
            && (level.isClientSide()?level==mc.level:level.getServer()==server);
    }
    public static void notice(Minecraft mc) {
        if(mc.level==notifiedLevel || mc.player==null) return;
        notifiedLevel=mc.level;
        if(!eligible(mc.level)) mc.player.sendSystemMessage(Component.literal("Skyrim bridge inactive: resume an existing bridge in Singleplayer, or create one for first setup."));
        else mc.player.sendSystemMessage(Component.literal("Skyrim bridge: waiting for ground and fresh collision. Next session, reopen this save to keep your inventory; New bridge starts a separate world."));
    }
    public static synchronized boolean hold(Player player,Vec3 movement) {
        if(!eligible(player.level())) return false;
        var box=player.getBoundingBox();
        var requested=box.expandTowards(movement).inflate(.5);
        String reason=BridgeRecovery.holding() ? "save_position_recovery" : SkyrimMenus.active() ? "skyrim_menu" : SkyCollision.coverageReason(player.level(),requested);
        if(reason.equals("ready") && !armed.containsKey(player)) {
            var below=new AABB(box.minX,box.minY-1,box.minZ,box.maxX,box.minY+.05,box.maxZ);
            boolean ground=SkyCollision.query(player.level(),below).stream().anyMatch(s->s.terrain());
            if(!ground) reason="waiting_for_initial_ground";
            else armed.put(player,true);
        }
        long now=System.nanoTime(); boolean ready=reason.equals("ready");
        boolean changed=!reason.equals(reasons.put(player,reason));
        if(!ready) heldSince.putIfAbsent(player,now);
        long duration=(now-heldSince.getOrDefault(player,now))/1_000_000L;
        if(changed || (!ready && now-reportAt.getOrDefault(player,0L)>1_000_000_000L)) {
            reportAt.put(player,now);
            LoggerFactory.getLogger("PassthroughPractice").info("event=SCENE_BRIDGE_MOTION side={} state={} hold_ms={} position={} movement={} requested={} coverage={}",
                player.level().isClientSide()?"client":"server",reason,duration,player.position(),movement,requested,SkyCollision.coverageDetails());
            if(changed && player.level().isClientSide()) player.sendOverlayMessage(Component.literal(ready?"Skyrim bridge ready":"Skyrim bridge holding: "+reason));
        }
        if(ready) { heldSince.remove(player); return false; }
        player.setDeltaMovement(Vec3.ZERO); player.resetFallDistance(); return true;
    }
}

