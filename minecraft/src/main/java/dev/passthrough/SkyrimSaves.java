package dev.passthrough;

import com.mojang.blaze3d.platform.InputConstants;
import net.minecraft.client.KeyMapping;
import net.minecraft.client.Minecraft;

/** Remappable keys asking Skyrim for its own quicksave and quickload. */
public final class SkyrimSaves {
    /** Command numbers shared with the Skyrim menu commands (1..6 are menu navigation). */
    public static final int QUICKSAVE=7,QUICKLOAD=8;
    private static KeyMapping[] keys;
    private static final boolean[] held=new boolean[2];
    private SkyrimSaves() {}
    public static KeyMapping[] bindings() {
        if(keys==null) keys=new KeyMapping[] {
            new KeyMapping("key.passthrough.quicksave",InputConstants.KEY_F5,KeyMapping.Category.MISC),
            new KeyMapping("key.passthrough.quickload",InputConstants.KEY_F9,KeyMapping.Category.MISC)};
        return keys;
    }
    /** One command per fresh press while allowed; presses at other times are discarded. */
    public static int frame(boolean allowed) {
        int command=0;
        for(int i=0;i<bindings().length;i++) {
            var key=keys[i];boolean clicked=false;
            while(key.consumeClick()) clicked=true;
            boolean press=clicked && !held[i];held[i]=key.isDown();
            if(press && allowed && command==0) command=QUICKSAVE+i;
        }
        return command;
    }
    /**
     * F5 is also Minecraft's perspective toggle. In a bridge world, when both actions share a
     * key, the perspective toggle is dropped so one press does not also change the camera.
     */
    public static void beforeKeybinds() {
        var mc=Minecraft.getInstance();
        if(mc.player==null || mc.level==null || !BridgeWorld.eligible(mc.level)) return;
        var perspective=mc.options.keyTogglePerspective;
        for(var key:bindings()) if(key.same(perspective)) while(perspective.consumeClick()) {}
    }
}
