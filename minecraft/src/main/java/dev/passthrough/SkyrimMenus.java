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
            }
        }
        extras(eligible);
        // Quicksave/quickload are gameplay commands: sent with context 0, only while no Skyrim menu is open.
        boolean gameplay=focused && !active();
        int save=SkyrimSaves.frame(gameplay);
        if(save!=0) {
            pending=new Command(++sequence,save,0);expires=now+200_000_000L;
            LoggerFactory.getLogger("PassthroughPractice").info("event=SCENE_MENU_SEND sequence={} action={} context=0 world={} source={}",sequence,save,PassthroughClient.worldId(),save==SkyrimSaves.QUICKSAVE?"quicksave":"quickload");
        }
        boolean valid=pending.context()!=0 ? eligible : pending.action()>=SkyrimSaves.QUICKSAVE && gameplay;
        if(!valid || now>=expires) pending=new Command(0,0,0);
        return pending;
    }
    /** Mouse motion that would turn the player moves Skyrim's menu pointer instead; true means captured. */
    public static boolean mouse(double dx,double dy) {
        var mc=Minecraft.getInstance();
        if(!active() || mc.gui.screen()!=null) return false;
        var window=mc.getWindow();
        int width=window.getScreenWidth(),height=window.getScreenHeight();
        if(width>0 && height>0 && Double.isFinite(dx) && Double.isFinite(dy) && (dx!=0 || dy!=0)) {
            cursorX+=dx/width;cursorY+=dy/height;cursorContext=context;
            long now=System.nanoTime();
            if(now-cursorLogged>=250_000_000L) {
                cursorLogged=now;
                LoggerFactory.getLogger("PassthroughPractice").info("event=SCENE_CURSOR_SEND context={} total_x={} total_y={} window_w={} window_h={} world={}",context,cursorX,cursorY,width,height,PassthroughClient.worldId());
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
    /** Optional true Skyrim mouse click at the pointer; left click stays the accepted confirm. */
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
     * A left click is a real Skyrim mouse click at the pointer; true means vanilla attack is
     * skipped. Enter stays the confirm key for the highlighted entry.
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
