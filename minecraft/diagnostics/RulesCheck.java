import static dev.passthrough.BridgeRules.*;

/**
 * Worked examples for dev.passthrough.BridgeRules, run without Minecraft before every build
 * (tools\test-java-rules.ps1). Any failed example stops the build.
 */
public final class RulesCheck {
    private static int checked,failed;
    private static void same(String what,double actual,double expected) {
        checked++;
        if(Math.abs(actual-expected)>1e-9) {failed++;System.out.println("FAIL "+what+": expected "+expected+", got "+actual);}
    }
    private static void yes(String what,boolean actual) {same(what,actual ? 1 : 0,1);}
    private static void no(String what,boolean actual) {same(what,actual ? 1 : 0,0);}

    public static void main(String[] args) {
        no("empty air opens Minecraft inventory",skyrimE(false,false));
        yes("door E activates",skyrimE(true,false));
        yes("seated E stands without crosshair",skyrimE(false,true));
        yes("seated E stands rather than uses target",skyrimE(true,true));
        yes("seat held with empty geometry",motionReason(false,false,true,"ready").equals("skyrim_seated"));
        yes("seat held outside coverage",motionReason(false,false,true,"outside_bounds").equals("skyrim_seated"));
        yes("menu precedence while seated",motionReason(false,true,true,"ready").equals("skyrim_menu"));
        yes("recovery precedence while seated",motionReason(true,true,true,"ready").equals("save_position_recovery"));
        yes("ordinary coverage hold retained",motionReason(false,false,false,"outside_bounds").equals("outside_bounds"));
        same("no controls when inactive",controlFlags(false,true,true),0);
        same("ordinary idle",controlFlags(true,false,false),0);
        same("WASD",controlFlags(true,true,false),1);
        same("creative flight or flight-off falling",controlFlags(true,false,true),2);
        same("WASD in creative",controlFlags(true,true,true),3);
        // Sharpness, from Minecraft's own data: 1 at level I, +0.5 per level.
        same("no sharpness",sharpnessBonus(0),0);
        same("sharpness I",sharpnessBonus(1),1.0);
        same("sharpness III",sharpnessBonus(3),2.0);
        same("sharpness V",sharpnessBonus(5),3.0);
        // Strength +3 per level, Weakness -4 per level.
        same("no effects",effectBonus(0,0),0);
        same("strength II",effectBonus(2,0),6.0);
        same("weakness I",effectBonus(0,1),-4.0);
        same("strength I and weakness I",effectBonus(1,1),-1.0);
        // A fist (1) under Weakness cannot go below zero.
        same("weak fist",weaponDamage(1.0,effectBonus(0,1)),0);
        same("iron sword with strength I",weaponDamage(6.0,effectBonus(1,0)),9.0);
        // Recharge: first attack full; a sword (speed 1.6) needs 12.5 ticks.
        same("first attack",charge(-1,1.6),1.0);
        same("sword at once",charge(0,1.6),0.04);
        same("sword after 6 ticks",charge(6,1.6),0.52);
        same("sword after 13 ticks",charge(13,1.6),1.0);
        same("no recharge gives a fifth",rechargeMultiplier(0),0.2);
        same("full recharge",rechargeMultiplier(1),1.0);
        same("half recharge",rechargeMultiplier(0.5),0.4);
        // Whole hits. Iron sword 6, Sharpness III 2: (6+2) x 1.75 = 14 at full charge.
        same("iron sword sharpness III, full",attackDamage(6,2,1,false),14.0);
        same("the same as full damage",fullDamage(6,2),14.0);
        // Critical is on the weapon part only: (6 x 1.5 + 2) x 1.75 = 19.25.
        same("iron sword sharpness III, critical",attackDamage(6,2,1,true),19.25);
        // Half charged: (6 x 0.4 + 2 x 0.5) x 1.75 = 5.95.
        same("iron sword sharpness III, half charge",attackDamage(6,2,0.5,false),5.95);
        // The accepted 0.10.5 numbers must not move: wooden sword 4 -> 7, fist 1 -> 1.75.
        same("wooden sword",attackDamage(4,0,1,false),7.0);
        same("fist",attackDamage(1,0,1,false),1.75);
        same("wooden sword critical",attackDamage(4,0,1,true),10.5);

        // Arrows. Full draw: speed 3 times 2 is 6, a quarter more for the critical arrow,
        // times 1.75 is 13.125.
        same("full draw",arrowDamage(1.0,0),13.125);
        // Half draw: speed 1.5 times 2 is 3, times 1.75 is 5.25; no critical.
        same("half draw",arrowDamage(0.5,0),5.25);
        same("barely drawn",arrowDamage(0.1,0),1.05);
        same("not drawn enough",arrowDamage(0.09,0),0);
        same("not a number",arrowDamage(Double.NaN,0),0);
        same("over-long draw counts as full",arrowDamage(1.7,0),13.125);
        // Power I adds 1 per speed, Power V adds 3: 3 x (2+3) x 1.25 x 1.75 = 32.8125.
        same("power I, full draw",arrowDamage(1.0,1),19.6875);
        same("power V, full draw",arrowDamage(1.0,5),32.8125);
        same("power V, half draw",arrowDamage(0.5,5),13.125);

        // The E key. Kinds: 0 nothing, 1 object, 2 container, 3 living, 4 body, 5 door, 6 enemy.
        no("E on nothing",skyrimActsOn(0,true));
        for(int kind=1;kind<=5;kind++) yes("E on kind "+kind,skyrimActsOn(kind,true));
        no("E on an enemy",skyrimActsOn(6,true));
        no("E on an unknown kind",skyrimActsOn(7,true));
        no("E on a negative kind",skyrimActsOn(-1,true));
        // Switched off: the 0.10.18 rule, containers and bodies only.
        for(int kind=0;kind<=7;kind++) same("0.10.18 rule, kind "+kind,skyrimActsOn(kind,false) ? 1 : 0,kind==2 || kind==4 ? 1 : 0);

        // Minecraft's hit pause: 20 ticks after a hit, any hit is taken from 10 down.
        no("just hit",hitPauseOver(20));
        no("11 ticks left",hitPauseOver(11));
        yes("10 ticks left",hitPauseOver(10));
        yes("no pause",hitPauseOver(0));
        no("pool one second old",poolExpired(1_000_000_000L));
        yes("pool three seconds old",poolExpired(3_000_000_000L));

        // Events from Skyrim: a hit has a positive Skyrim amount, restored health a negative one.
        yes("a hit",validHealthEvent(0.25f,25f));
        yes("a potion",validHealthEvent(0.5f,-50f));
        no("no amount",validHealthEvent(0.5f,0f));
        no("no share of health",validHealthEvent(0f,-50f));
        no("a negative share",validHealthEvent(-0.5f,-50f));
        no("not a number",validHealthEvent(Float.NaN,25f));
        no("infinite amount",validHealthEvent(0.5f,Float.NEGATIVE_INFINITY));
        yes("negative amount restores",isRestore(-50f));
        no("positive amount is a hit",isRestore(25f));

        // Arrival heights in a normal world (blocks from -64 to 319).
        same("ordinary height",arrival(70,-64,319,true),NORMAL);
        same("on the floor",arrival(-64,-64,319,true),NORMAL);
        same("head at the ceiling",arrival(317,-64,319,true),NORMAL);
        same("head above the ceiling",arrival(318,-64,319,true),NO_BUILDING);
        same("mountain",arrival(700,-64,319,true),NO_BUILDING);
        same("sea level at home",arrival(-98.3,-64,319,true),NO_BUILDING);
        same("deep sea floor",arrival(-300,-64,319,true),NO_BUILDING);
        same("lowest allowed",arrival(-64-BELOW_FLOOR,-64,319,true),NO_BUILDING);
        same("just too deep",arrival(-64-BELOW_FLOOR-0.01,-64,319,true),REFUSED);
        same("highest allowed",arrival(319+ABOVE_CEILING,-64,319,true),NO_BUILDING);
        same("too high",arrival(319+ABOVE_CEILING+1,-64,319,true),REFUSED);
        same("not a number",arrival(Double.NaN,-64,319,true),REFUSED);
        same("infinite",arrival(Double.POSITIVE_INFINITY,-64,319,true),REFUSED);
        // With the exemption switched off, arrival stops short of Minecraft's damage at 64 below.
        yes("the short limit is above the damage",BELOW_FLOOR_WITH_VOID_DAMAGE<64);
        same("sea level, no exemption",arrival(-98.3,-64,319,false),NO_BUILDING);
        same("lowest allowed, no exemption",arrival(-64-BELOW_FLOOR_WITH_VOID_DAMAGE,-64,319,false),NO_BUILDING);
        same("deep sea floor, no exemption",arrival(-300,-64,319,false),REFUSED);

        // A held direction: nothing for 0.4 s, then one step every 0.12 s.
        no("just pressed",repeatDue(0,0));
        no("held 0.3 s",repeatDue(300,300));
        yes("held 0.4 s",repeatDue(400,400));
        no("0.05 s after a repeat",repeatDue(450,50));
        yes("0.12 s after a repeat",repeatDue(520,120));
        // One second of holding gives the press plus six repeats (at 0.4, 0.52, ... 1.0 s).
        int steps=0;long last=0;
        for(long t=0;t<=1000;t+=10) if(repeatDue(t,t-last)) {steps++;last=t;}
        same("repeats in one second",steps,6);

        // Pointer speed from Minecraft's sensitivity setting (0.5 is shown as 100%).
        same("sensitivity 100%",pointerFactor(0.5),1.0);
        same("sensitivity 200%",pointerFactor(1.0),2.0);
        same("sensitivity 50%",pointerFactor(0.25),0.5);
        same("sensitivity 0% still moves",pointerFactor(0.0),0.25);
        same("sensitivity out of range",pointerFactor(5.0),2.0);
        same("sensitivity not a number",pointerFactor(Double.NaN),1.0);

        // Pointer: the monitor only when known and not smaller than the window.
        yes("small window on a big monitor",measureAgainstMonitor(854,480,2560,1440));
        yes("full screen",measureAgainstMonitor(2560,1440,2560,1440));
        no("monitor unknown",measureAgainstMonitor(854,480,0,0));
        no("window wider than the monitor",measureAgainstMonitor(3000,480,2560,1440));
        no("no window",measureAgainstMonitor(0,0,2560,1440));

        if(failed>0) {System.out.println("FAILED: "+failed+" of "+checked+" Minecraft-side rule checks.");System.exit(1);}
        System.out.println("PASS: "+checked+" Minecraft-side rule checks.");
    }
}
