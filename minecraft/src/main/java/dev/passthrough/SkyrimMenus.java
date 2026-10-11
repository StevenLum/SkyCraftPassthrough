package dev.passthrough;

import com.mojang.blaze3d.platform.InputConstants;
import net.minecraft.client.KeyMapping;
import net.minecraft.client.Minecraft;
import org.slf4j.LoggerFactory;

/** Menu status expires; each command belongs to one particular Skyrim menu context. */
public final class SkyrimMenus {
    private static KeyMapping[] keys;
    private static final boolean[] held=new boolean[6];
    private static volatile long freshUntil;
    private static long context,sequence,expires;
    private static Command pending=new Command(0,0,0);
    public record Command(long sequence,int action,long context) {}
    /** Running mouse total for one menu context, in window widths/heights; context 0 means none. */
    public record Cursor(long context,float x,float y) {}
    private static double cursorX,cursorY;
    private static long cursorContext,cursorLogged;
    /** Size the pointer travel is measured against, re-read at most once a second. */
    private static int spanX,spanY;
    private static long spanChecked;
    /**
     * The monitor the window is on, not the window: one mouse unit then moves the Skyrim
     * pointer one monitor pixel whatever size the Minecraft window is. Falls back to the
     * window when the monitor cannot be read.
     */
    private static void span(com.mojang.blaze3d.platform.Window window,int width,int height,long now) {
        if(spanX>0 && now-spanChecked<1_000_000_000L) return;
        spanChecked=now;spanX=width;spanY=height;
        // Switched off in switches.txt: measure against the window, as before 0.10.24.
        if(!Switches.on(Switches.POINTER_MONITOR)) return;
        var monitor=window.findBestMonitor();
        var mode=monitor==null ? null : monitor.currentMode();
        if(mode!=null && BridgeRules.measureAgainstMonitor(width,height,mode.getWidth(),mode.getHeight())) {spanX=mode.getWidth();spanY=mode.getHeight();}
    }
    private SkyrimMenus() {}
    public static KeyMapping[] bindings() {
        if(keys==null) {
            String[] names={"up","down","left","right","accept","cancel"};
            int[] defaults={InputConstants.KEY_UP,InputConstants.KEY_DOWN,InputConstants.KEY_LEFT,InputConstants.KEY_RIGHT,InputConstants.KEY_RETURN,InputConstants.KEY_BACKSPACE};
            keys=new KeyMapping[6];
            for(int i=0;i<6;i++) keys[i]=new KeyMapping("key.passthrough.menu_"+names[i],defaults[i],KeyMapping.Category.MISC);
        }
        return keys;
    }
    public static boolean active() {return System.nanoTime()<freshUntil;}
    public static void refresh(NativeLink link,boolean eligible) throws Throwable {
        var status=link.menu();
        long next=eligible && status!=null && status.getInt(0)==1 && status.getLong(16)==PassthroughClient.worldId() ? status.getLong(8) : 0;
        if(next!=context) {
            context=next;pending=new Command(0,0,0);
            cursorX=0;cursorY=0;cursorContext=0;
            for(var key:bindings()) while(key.consumeClick()) {}
            LoggerFactory.getLogger("PassthroughPractice").info("event=SCENE_MENU_STATUS active={} context={}",next!=0,next);
        }
        freshUntil=next==0 ? 0 : System.nanoTime()+250_000_000L;
    }
    public static Command frame(boolean enabled) {
        var mc=Minecraft.getInstance();
        boolean focused=enabled && mc.isWindowActive() && mc.gui.screen()==null;
        boolean eligible=focused && active();
        long now=System.nanoTime();
        for(int i=0;i<bindings().length;i++) {
            var key=keys[i];boolean clicked=false;
            while(key.consumeClick()) clicked=true;
            boolean press=clicked && !held[i];held[i]=key.isDown();
            if(press && eligible) {
                pending=new Command(++sequence,i+1,context);expires=now+200_000_000L;
                LoggerFactory.getLogger("PassthroughPractice").info("event=SCENE_MENU_SEND sequence={} action={} context={} world={}",sequence,i+1,context,PassthroughClient.worldId());
                if(i<4) arm(i,now);
            }
        }
        directions(eligible,now);
        extras(eligible);
        // Quicksave/quickload are gameplay commands: sent with context 0, only while no Skyrim menu is open.
        boolean gameplay=focused && !active();
        int save=SkyrimSaves.frame(gameplay);
        if(save!=0) {
            pending=new Command(++sequence,save,0);expires=now+200_000_000L;
            LoggerFactory.getLogger("PassthroughPractice").info("event=SCENE_MENU_SEND sequence={} action={} context=0 world={} source={}",sequence,save,PassthroughClient.worldId(),save==SkyrimSaves.QUICKSAVE?"quicksave":"quickload");
        }
        // Keys that open Skyrim's own menus are gameplay commands too; a save in the same frame wins.
        int open=SkyrimShortcuts.frame(gameplay,eligible);
        if(open!=0 && save==0) {
            pending=new Command(++sequence,open,0);expires=now+200_000_000L;
            LoggerFactory.getLogger("PassthroughPractice").info("event=SCENE_MENU_SEND sequence={} action={} context=0 world={} source={}",sequence,open,PassthroughClient.worldId(),SkyrimShortcuts.name(open));
        }
        boolean valid=pending.context()!=0 ? eligible : pending.action()>=SkyrimSaves.QUICKSAVE && gameplay;
        if(!valid || now>=expires) pending=new Command(0,0,0);
        return pending;
    }
    /** Per direction (up, down, left, right): the menu it was pressed in, since when, and the last send. */
    private static final long[] armedContext=new long[4],downSince=new long[4],lastSent=new long[4];
    private static final boolean[] moveHeld=new boolean[4];
    private static void arm(int direction,long now) {armedContext[direction]=context;downSince[direction]=now;lastSent[direction]=now;}
    /**
     * W, A, S and D (Minecraft's movement keys) move through a Skyrim menu as the arrow keys
     * do, and a direction that stays held repeats so long lists can be scrolled. Only a
     * direction pressed while this menu was open repeats: walking into a conversation with W
     * held does not scroll it. Switched off in switches.txt with menu_wasd=off.
     */
    private static void directions(boolean eligible,long now) {
        var mc=Minecraft.getInstance();
        KeyMapping[] move={mc.options.keyUp,mc.options.keyDown,mc.options.keyLeft,mc.options.keyRight};
        boolean on=Switches.on(Switches.MENU_WASD);
        for(int i=0;i<4;i++) {
            // Movement reads "is down", never these presses, so emptying them changes nothing
            // for walking and keeps an old press from arriving in a later menu.
            boolean clicked=false;
            while(move[i].consumeClick()) clicked=true;
            boolean press=clicked && !moveHeld[i];moveHeld[i]=move[i].isDown();
            if(!on) {armedContext[i]=0;continue;}
            if(press && eligible && request(i+1,"move_key")) arm(i,now);
            boolean down=keys[i].isDown() || move[i].isDown();
            if(!down || !eligible || armedContext[i]!=context) {
                if(!down) armedContext[i]=0;
                continue;
            }
            if(BridgeRules.repeatDue((now-downSince[i])/1_000_000L,(now-lastSent[i])/1_000_000L) && request(i+1,"repeat")) lastSent[i]=now;
        }
    }
    /** Mouse motion that would turn the player moves Skyrim's menu pointer instead; true means captured. */
    public static boolean mouse(double dx,double dy) {
        var mc=Minecraft.getInstance();
        if(!active() || mc.gui.screen()!=null) return false;
        var window=mc.getWindow();
        int width=window.getScreenWidth(),height=window.getScreenHeight();
        if(width>0 && height>0 && Double.isFinite(dx) && Double.isFinite(dy) && (dx!=0 || dy!=0)) {
            long now=System.nanoTime();
            span(window,width,height,now);
            // Minecraft's own Mouse Sensitivity setting scales the pointer: 100% is one monitor
            // pixel per mouse unit. With pointer_monitor switched off the old mapping is exact.
            double factor=Switches.on(Switches.POINTER_MONITOR) ? BridgeRules.pointerFactor(mc.options.sensitivity().get()) : 1.0;
            cursorX+=dx/spanX*factor;cursorY+=dy/spanY*factor;cursorContext=context;
            if(now-cursorLogged>=250_000_000L) {
                cursorLogged=now;
                LoggerFactory.getLogger("PassthroughPractice").info("event=SCENE_CURSOR_SEND context={} total_x={} total_y={} window_w={} window_h={} span_w={} span_h={} factor={} world={}",context,cursorX,cursorY,width,height,spanX,spanY,factor,PassthroughClient.worldId());
            }
        }
        return true;
    }
    public static Cursor cursor(boolean enabled) {
        return enabled && active() && cursorContext!=0 && cursorContext==context
            ? new Cursor(context,(float)cursorX,(float)cursorY) : new Cursor(0,0,0);
    }
    /** Command numbers beyond the six navigation keys; the Skyrim side holds the matching table. */
    public static final int CONFIRM=5,BACK=6,TAKE_ALL=9,POINTER_CLICK=10;
    private static KeyMapping pointerClick;
    private static boolean pointerHeld,backHeld,confirmHeld;
    private static long polled;
    /**
     * A held-button latch only counts while the per-frame poll that releases it is running.
     * If the bridge sender ever stops, the latch lapses instead of swallowing clicks for good.
     */
    private static boolean latched(boolean flag) {return flag && System.nanoTime()-polled<500_000_000L;}
    /** Optional Skyrim left mouse action, including native use in item menus. */
    public static KeyMapping pointerClickBinding() {
        if(pointerClick==null) pointerClick=new KeyMapping("key.passthrough.menu_click",InputConstants.Type.MOUSE,InputConstants.MOUSE_BUTTON_MIDDLE,KeyMapping.Category.MISC);
        return pointerClick;
    }
    /** One command for the open Skyrim menu from the mouse or an extra key; false when no menu takes it. */
    public static boolean request(int action,String source) {
        var mc=Minecraft.getInstance();
        if(!active() || !mc.isWindowActive() || mc.gui.screen()!=null) return false;
        pending=new Command(++sequence,action,context);expires=System.nanoTime()+200_000_000L;
        LoggerFactory.getLogger("PassthroughPractice").info("event=SCENE_MENU_SEND sequence={} action={} context={} world={} source={}",sequence,action,context,PassthroughClient.worldId(),source);
        return true;
    }
    /**
     * A left click sends Skyrim its native menu action: item use in inventory/favourites,
     * pointer click elsewhere. True skips vanilla attack. Enter remains confirm.
     */
    public static boolean click() {
        if(!request(POINTER_CLICK,"mouse")) return false;
        confirmHeld=true;
        return true;
    }
    /** True while the button press that confirmed a Skyrim choice is still held down. */
    public static boolean confirmHeld() {return latched(confirmHeld);}
    /**
     * A right click goes back once per press; true means the held Minecraft item is not used.
     * The press stays swallowed until released, so closing a menu cannot also place a block.
     */
    public static boolean back() {
        var mc=Minecraft.getInstance();
        if(latched(backHeld)) return true;
        if(!active() || mc.gui.screen()!=null) return false;
        backHeld=true;request(BACK,"right_click");
        return true;
    }
    /** The wheel moves the Skyrim selection; true means the Minecraft hotbar is left alone. */
    public static boolean scroll(double amount) {
        var mc=Minecraft.getInstance();
        if(!active() || mc.gui.screen()!=null) return false;
        if(amount!=0) request(amount>0 ? 1 : 2,"wheel");
        return true;
    }
    /** Per frame: release the right-click latch and read the optional pointer-click key. */
    private static void extras(boolean eligible) {
        var mc=Minecraft.getInstance();
        polled=System.nanoTime();
        if(!mc.options.keyUse.isDown()) backHeld=false;
        if(!mc.options.keyAttack.isDown()) confirmHeld=false;
        var key=pointerClickBinding();boolean clicked=false;
        while(key.consumeClick()) clicked=true;
        boolean press=clicked && !pointerHeld;pointerHeld=key.isDown();
        if(press && eligible) request(POINTER_CLICK,"pointer_click");
    }
    /**
     * Minecraft gives a press to every action bound to that key. While a Skyrim menu is open
     * and the pointer-click key shares a key with "pick block", the pick is dropped.
     */
    public static void beforeKeybinds() {
        var mc=Minecraft.getInstance();
        if(!active() || mc.player==null) return;
        var pick=mc.options.keyPickItem;
        if(pointerClickBinding().same(pick)) while(pick.consumeClick()) {}
    }
}
