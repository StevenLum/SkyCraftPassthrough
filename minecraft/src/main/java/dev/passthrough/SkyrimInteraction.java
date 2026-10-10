package dev.passthrough;

import net.minecraft.client.KeyMapping;
import net.minecraft.client.Minecraft;
import com.mojang.blaze3d.platform.InputConstants;
import org.slf4j.LoggerFactory;

/** Single short-lived request per physical key press, consumed after rendering. */
public final class SkyrimInteraction {
    private static KeyMapping key;
    private static boolean held,inventoryHeld,inventoryPress;
    private static long sequence,pending,expires;
    private SkyrimInteraction() {}
    public static KeyMapping binding() {
        if(key==null) key=new KeyMapping("key.passthrough.interact",InputConstants.KEY_R,KeyMapping.Category.MISC);
        return key;
    }
    /**
     * The inventory key (E) acts like Skyrim's own E where that is clearly meant: it confirms
     * in an open Skyrim menu, and it opens the Skyrim container or body under the crosshair.
     * Everywhere else the press is left alone and Minecraft opens its inventory as usual.
     * Runs before Minecraft reads its key presses.
     */
    public static void beforeKeybinds() {
        var mc=Minecraft.getInstance();
        if(mc.player==null || mc.level==null) return;
        var inventory=mc.options.keyInventory;
        boolean wasDown=inventoryHeld;inventoryHeld=inventory.isDown();
        boolean menu=SkyrimMenus.active();
        boolean loot=!menu && mc.gui.screen()==null && BridgeWorld.eligible(mc.level)
            && SkyCollision.admitted() && SkyrimAttack.lootTarget();
        if(!menu && !loot) return;
        boolean clicked=false;
        while(inventory.consumeClick()) clicked=true;
        if(!clicked || wasDown) return;
        if(menu) SkyrimMenus.request(SkyrimMenus.CONFIRM,"inventory_key");
        else inventoryPress=true;
    }
    public static long frame(boolean active) {
        var mc=Minecraft.getInstance(); var binding=binding();
        boolean clicked=false;
        while(binding.consumeClick()) clicked=true;
        String source=inventoryPress ? "inventory_key" : "interact_key";
        boolean keyPress=clicked && !held,press=keyPress || inventoryPress;
        held=binding.isDown();inventoryPress=false;
        boolean eligible=active && !SkyrimMenus.active() && mc.isWindowActive() && mc.gui.screen()==null && SkyCollision.admitted();
        long now=System.nanoTime();
        if(!eligible) {
            pending=0;
            // In an open Skyrim menu the Interact key is Skyrim's own R: take all, drop and similar.
            // Only that key: a press that came from the inventory key must never become "take all".
            if(press && !(keyPress && active && SkyrimMenus.request(SkyrimMenus.TAKE_ALL,"interact_key")))
                LoggerFactory.getLogger("PassthroughPractice").info("event=SCENE_INTERACT_INPUT state=discarded reason=inactive_or_menu");
            return 0;
        }
        if(press) {
            pending=++sequence;expires=now+200_000_000L;
            LoggerFactory.getLogger("PassthroughPractice").info("event=SCENE_INTERACT_SEND sequence={} world={} action=activate source={}",pending,PassthroughClient.worldId(),source);
        }
        if(now>=expires) pending=0;
        return pending;
    }
}
