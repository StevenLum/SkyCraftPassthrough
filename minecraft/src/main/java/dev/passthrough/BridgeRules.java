package dev.passthrough;

/**
 * The bridge's own arithmetic and yes/no rules, with nothing from Minecraft in them, so they
 * can be checked without the game: minecraft/diagnostics/RulesCheck.java runs worked examples
 * against this file before every build (tools/test-java-rules.ps1). The classes that talk to
 * Minecraft call these functions instead of repeating the numbers.
 */
public final class BridgeRules {
    private BridgeRules() {}
    /** E is an exit request while deliberately seated, even with no crosshair target. */
    public static boolean skyrimE(boolean actsOnTarget,boolean seated) {return seated || actsOnTarget;}
    /** Seating holds before geometry is consulted; recovery/menu still have precedence. */
    public static String motionReason(boolean recovery,boolean menu,boolean seated,String coverage) {
        return recovery ? "save_position_recovery" : menu ? "skyrim_menu" : seated ? "skyrim_seated" : coverage;
    }
    /** Flight permission keeps tall coverage active after flight is toggled off in creative. */
    public static int controlFlags(boolean eligible,boolean movementKey,boolean mayFly) {
        return eligible ? (movementKey ? 1 : 0) | (mayFly ? 2 : 0) : 0;
    }

    // ---- Attacks on Skyrim enemies ----

    /** Skyrim damage per point of Minecraft attack damage; lines sword tiers up with Skyrim's own swords. */
    public static final double DAMAGE_SCALE=1.75;
    /** Minecraft's own bonus for a critical hit. */
    public static final double CRITICAL_SCALE=1.5;
    /** A critical hit needs the attack recharged beyond this. */
    public static final double CRITICAL_CHARGE=0.9;
    /** Minecraft's Sharpness: 1 at level one and half a point more for each further level. */
    public static double sharpnessBonus(int level) {return level<=0 ? 0.0 : 0.5*level+0.5;}
    /** Strength adds 3 attack damage per level and Weakness takes 4. Levels count from 1; 0 means none. */
    public static double effectBonus(int strengthLevel,int weaknessLevel) {
        return 3.0*Math.max(0,strengthLevel)-4.0*Math.max(0,weaknessLevel);
    }
    /** The weapon's damage once effects are counted; never below zero. */
    public static double weaponDamage(double itemDamage,double effectBonus) {return Math.max(0.0,itemDamage+effectBonus);}
    /** How far the attack has recharged, 0 to 1. A negative tick count means "first attack": full. */
    public static double charge(long elapsedTicks,double attackSpeed) {
        return elapsedTicks<0 ? 1.0 : Math.min(1.0,(elapsedTicks+0.5)*attackSpeed/20.0);
    }
    /** Minecraft's own curve from recharge to damage: a fifth at once, the rest with the square. */
    public static double rechargeMultiplier(double charge) {return 0.2+0.8*charge*charge;}
    /** Skyrim damage of a fully recharged, non-critical hit. */
    public static double fullDamage(double weaponDamage,double sharpnessBonus) {return (weaponDamage+sharpnessBonus)*DAMAGE_SCALE;}
    /**
     * Skyrim damage asked for one hit. As in Minecraft's own attack, recharge and critical act
     * on the weapon's damage; the enchantment bonus follows the recharge in a straight line and
     * is never critical.
     */
    public static double attackDamage(double weaponDamage,double sharpnessBonus,double charge,boolean critical) {
        return (weaponDamage*rechargeMultiplier(charge)*(critical ? CRITICAL_SCALE : 1.0)+sharpnessBonus*charge)*DAMAGE_SCALE;
    }

    // ---- Shots with a bow ----

    /** A bow drawn less than this far does not shoot at all, as in Minecraft. */
    public static final double MINIMUM_DRAW=0.1;
    /**
     * Skyrim damage of one arrow. Minecraft's arrow: three times the draw (0 to 1) is its
     * speed, and it deals speed times 2, or times 2 plus half a point per Power level plus a
     * half. A fully drawn bow shoots a critical arrow, worth a quarter more here (Minecraft
     * adds a random amount of up to about half). Then the same scale as a sword.
     */
    public static double arrowDamage(double draw,int powerLevel) {
        if(!(draw>=MINIMUM_DRAW)) return 0.0;
        double pull=Math.min(1.0,draw);
        double perSpeed=2.0+(powerLevel>0 ? 0.5*powerLevel+0.5 : 0.0);
        return 3.0*pull*perSpeed*(pull>=1.0 ? 1.25 : 1.0)*DAMAGE_SCALE;
    }

    // ---- The inventory key (E) ----

    /**
     * Whether Skyrim's own E acts on what its crosshair is on. Kinds: 0 nothing, 1 object,
     * 2 container, 3 living character, 4 body, 5 door, 6 enemy. With precedence switched off
     * only containers and bodies count, as in 0.10.18. An unknown kind never counts.
     */
    public static boolean skyrimActsOn(int kind,boolean precedence) {
        return precedence ? kind>=1 && kind<=5 : kind==2 || kind==4;
    }

    // ---- Skyrim hits on the player, and health Skyrim restores ----

    /**
     * One event from Skyrim's list: a share of full health above zero, and Skyrim's own amount,
     * which is positive for health taken and negative for health restored. Zero is neither.
     */
    public static boolean validHealthEvent(float fraction,float raw) {
        return Float.isFinite(fraction) && fraction>0 && Float.isFinite(raw) && raw!=0;
    }
    public static boolean isRestore(float raw) {return raw<0;}

    /** Minecraft takes any hit once this many or fewer of its 20 hit-pause ticks remain. */
    public static final int HIT_PAUSE_OVER=10;
    public static boolean hitPauseOver(int pauseTicks) {return pauseTicks<=HIT_PAUSE_OVER;}
    /** A pool that could not be delivered for this long is dropped, never delivered late. */
    public static final long POOL_MAX_AGE_NANOS=2_000_000_000L;
    public static boolean poolExpired(long ageNanos) {return ageNanos>POOL_MAX_AGE_NANOS;}

    // ---- Arriving after a door, a load or fast travel ----

    public static final int REFUSED=0,NO_BUILDING=1,NORMAL=2;
    /**
     * How far outside Minecraft's building range the player may arrive. The ground comes from
     * Skyrim's collision, not from blocks, so a player can stand above the ceiling. Below the
     * floor Minecraft's own "fell out of the world" damage starts at 64 blocks; a bridge world
     * exempts the player from it (BridgeVoidMixin), and only when that is switched off must
     * arrival stop short of it.
     */
    public static final int ABOVE_CEILING=2000,BELOW_FLOOR=2000,BELOW_FLOOR_WITH_VOID_DAMAGE=56;
    /**
     * `minY` and `maxY` are the lowest and highest block heights of the Minecraft world;
     * `voidGuard` says whether the player is exempt from falling out of the world.
     */
    public static int arrival(double y,int minY,int maxY,boolean voidGuard) {
        int below=voidGuard ? BELOW_FLOOR : BELOW_FLOOR_WITH_VOID_DAMAGE;
        if(!Double.isFinite(y) || y<minY-below || y>maxY+ABOVE_CEILING) return REFUSED;
        return y>=minY && y+2<=maxY ? NORMAL : NO_BUILDING;
    }

    // ---- Skyrim menus: a held direction repeats ----

    /** A held direction starts repeating after this long, then repeats this often. */
    public static final long REPEAT_DELAY_MS=400,REPEAT_EVERY_MS=120;
    public static boolean repeatDue(long heldMs,long sinceLastMs) {
        return heldMs>=REPEAT_DELAY_MS && sinceLastMs>=REPEAT_EVERY_MS;
    }

    // ---- Skyrim's menu pointer ----

    /**
     * Pointer speed from Minecraft's Mouse Sensitivity setting, which runs from 0 to 1 with
     * 0.5 shown as 100%: twice the setting, so 100% leaves the speed alone and 200% doubles
     * it. Never below a quarter, so the pointer cannot be set to stand still.
     */
    public static double pointerFactor(double sensitivity) {
        return Double.isFinite(sensitivity) ? Math.max(0.25,Math.min(2.0,sensitivity*2.0)) : 1.0;
    }
    /** The monitor is used for pointer travel only when it is known and not smaller than the window. */
    public static boolean measureAgainstMonitor(int windowWidth,int windowHeight,int monitorWidth,int monitorHeight) {
        return windowWidth>0 && windowHeight>0 && monitorWidth>=windowWidth && monitorHeight>=windowHeight;
    }
}
