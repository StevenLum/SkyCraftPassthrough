package dev.passthrough;

import com.mojang.blaze3d.platform.InputConstants;
import net.minecraft.client.KeyMapping;

/**
 * Remappable keys for things Skyrim does itself during play: its menus (as its Tab, M, J, T,
 * I, P and Q keys do) and its shout or power (Z).
 */
public final class SkyrimShortcuts {
    /** Command numbers continue the shared list (1..6 navigation, 7..8 saves, 9..10 menu extras). */
    public static final int CHARACTER_MENU=11;
    private static final String[] NAMES={"character_menu","map","journal","wait","items","magic","favorites","shout"};
    /** Place of the shout key in the list; it has a switch of its own. */
    private static final int SHOUT=7;
    private static KeyMapping[] keys;
    private static final boolean[] held=new boolean[NAMES.length];
    private SkyrimShortcuts() {}
    public static KeyMapping[] bindings() {
        if(keys==null) {
            // Skyrim's own defaults where Minecraft leaves the key free; "wait" starts unbound
            // because T is Minecraft's chat key.
            // Favourites is Q in Skyrim, which drops the held item in Minecraft, so it starts on G.
            int[] defaults={InputConstants.KEY_TAB,InputConstants.KEY_M,InputConstants.KEY_J,InputConstants.UNKNOWN.getValue(),
                InputConstants.KEY_I,InputConstants.KEY_P,InputConstants.KEY_G,InputConstants.KEY_Z};
            keys=new KeyMapping[NAMES.length];
            for(int i=0;i<keys.length;i++) keys[i]=new KeyMapping("key.passthrough.open_"+NAMES[i],defaults[i],KeyMapping.Category.MISC);
        }
        return keys;
    }
    /**
     * P is also Minecraft's "social interactions", which only prints a notice in a
     * single-player world. In a bridge world, while it shares a key with one of these, that
     * notice is dropped before Minecraft reads the press.
     */
    public static void beforeKeybinds() {
        var mc=net.minecraft.client.Minecraft.getInstance();
        if(mc.player==null || mc.level==null || !BridgeWorld.eligible(mc.level)) return;
        var social=mc.options.keySocialInteractions;
        for(var key:bindings()) if(key.same(social)) while(social.consumeClick()) {}
    }
    /** Log name of a command returned by {@link #frame}. */
    public static String name(int command) {return NAMES[command-CHARACTER_MENU];}
    /**
     * One command per fresh press. During normal play the returned command opens that Skyrim
     * menu. While a Skyrim menu is open the character-menu key goes back instead, as Tab does
     * in Skyrim, and the other keys do nothing. Returns 0 when there is nothing to send.
     */
    public static int frame(boolean gameplay,boolean inMenu) {
        int command=0;
        // Switched off in switches.txt: the keys are still read, so no press is left over, but nothing is sent.
        boolean menus=Switches.on(Switches.MENU_KEYS),shout=Switches.on(Switches.SHOUT);
        for(int i=0;i<bindings().length;i++) {
            var key=keys[i];boolean clicked=false;
            while(key.consumeClick()) clicked=true;
            boolean press=clicked && !held[i];held[i]=key.isDown();
            if(!press || !(i==SHOUT ? shout : menus)) continue;
            if(gameplay && command==0) command=CHARACTER_MENU+i;
            else if(inMenu && i==0) SkyrimMenus.request(SkyrimMenus.BACK,"character_menu_key");
        }
        return command;
    }
}
