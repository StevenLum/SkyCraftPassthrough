package dev.passthrough;

/**
 * Steps the user switched off in switches.txt (project folder, one "name=off" per line).
 * The bridge library reads the file once and hands over the set; the numbers are the bit
 * order of switches::NAMES in the Rust source and must never be reordered.
 */
public final class Switches {
    public static final int E_PRECEDENCE=0,HOSTILITY=1,MENU_KEYS=2,DAMAGE_POOL=3,PAUSE_SHIELD=4,
        POINTER_MONITOR=5,HEALING=6,MENU_WASD=7,VOID_GUARD=8,SERVER_TRUST=9,POSE_FREE=10,SLOW_SCAN=11,
        RANGED=12,SHOUT=13,SEAT_GUARD=14,STAND_UP=15,FAST_SCAN=16,SITTING=17,SAFE_ARRIVAL=18,FURNITURE_GROUND=19,NATIVE_HANDS=20,ITEM_CLICK=21;
    private static volatile long off;
    private Switches() {}
    static void set(long mask) {off=mask;}
    /** True unless the user switched this step off. Everything is on until the library is loaded. */
    public static boolean on(int which) {return (off&(1L<<which))==0;}
}
