package dev.passthrough;

import java.util.Comparator;
import net.minecraft.client.Minecraft;
import net.minecraft.client.gui.components.Button;
import net.minecraft.client.gui.screens.Screen;
import net.minecraft.client.gui.screens.AlertScreen;
import net.minecraft.client.gui.screens.ConfirmScreen;
import net.minecraft.network.chat.Component;
import net.minecraft.world.level.storage.LevelSummary;
import org.slf4j.LoggerFactory;

/** Opens Minecraft saves through its normal loader; never copies player data. */
public final class BridgeSaves {
    private BridgeSaves() {}
    public static void confirmNew(Screen parent) {
        var mc=Minecraft.getInstance();
        mc.gui.setScreen(new ConfirmScreen(yes->{
            if(yes) BridgeWorld.create(parent); else mc.gui.setScreen(parent);
        },Component.literal("Create a separate bridge world?"),
            Component.literal("This starts a new inventory and world. To keep your existing inventory and builds, cancel and use Resume bridge or select your existing save."),
            Component.literal("Create new"),Component.literal("Cancel")));
    }
    public static void resume(Screen parent,Button button) {
        var mc=Minecraft.getInstance(); button.active=false;
        try {
            var storage=mc.getLevelSource();
            storage.loadLevelSummaries(storage.findLevelCandidates()).whenComplete((worlds,error)->mc.execute(()->{
                button.active=true;
                if(mc.gui.screen()!=parent) return;
                if(error!=null) { failure(parent,error); return; }
                var latest=worlds.stream().filter(s->BridgeWorld.isBridgeName(s.getLevelName()))
                    .max(Comparator.comparingLong(LevelSummary::getLastPlayed).thenComparing(LevelSummary::getLevelId));
                if(latest.isEmpty()) {
                    alert(parent,"No saved bridge found","Use New Skyrim bridge once to start. After that, reopen that same save. Renamed saves can still be found in the ordinary world list."); return;
                }
                var save=latest.get();
                if(save.isLocked() || save.isDisabled()) {
                    alert(parent,"Bridge save unavailable",save.getLevelName()+" cannot be opened now. Check its entry in the world list; no new world has been created."); return;
                }
                mc.gui.setScreen(new ConfirmScreen(yes->{
                    if(!yes) { mc.gui.setScreen(parent); return; }
                    LoggerFactory.getLogger("PassthroughPractice").info("event=SCENE_BRIDGE_RESUME save={} name={} last_played={}",save.getLevelId(),save.getLevelName(),save.getLastPlayed());
                    mc.createWorldOpenFlows().openWorld(save.getLevelId(),()->mc.gui.setScreen(parent));
                },Component.literal("Resume this bridge?"),
                    Component.literal(save.getLevelName()+"\nSave: "+save.getLevelId()+"\nThis is your most recently played bridge. Its Minecraft inventory and builds will be loaded. Cancel to select a different save from the list."),
                    Component.literal("Resume"),Component.literal("Cancel")));
            }));
        } catch(Exception error) { button.active=true; failure(parent,error); }
    }
    private static void failure(Screen parent,Throwable error) {
        LoggerFactory.getLogger("PassthroughPractice").error("event=SCENE_BRIDGE_RESUME_FAILED",error);
        alert(parent,"Could not read bridge saves","Return to the world list and check the save. No new world was created.");
    }
    private static void alert(Screen parent,String title,String message) {
        var mc=Minecraft.getInstance();
        mc.gui.setScreen(new AlertScreen(()->mc.gui.setScreen(parent),Component.literal(title),Component.literal(message)));
    }
}

