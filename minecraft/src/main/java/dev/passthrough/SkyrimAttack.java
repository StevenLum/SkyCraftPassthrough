package dev.passthrough;

import java.nio.ByteBuffer;
import net.minecraft.client.Minecraft;
import net.minecraft.world.phys.BlockHitResult;
import net.minecraft.world.phys.HitResult;
import org.slf4j.LoggerFactory;

/** Observe vanilla attacks without consuming clicks or cancelling block breaking. */
public final class SkyrimAttack {
    public record Command(long sequence,int handle,float damage) {}
    private static int target,targetKind;
    private static net.minecraft.client.player.LocalPlayer rechargePlayer;
    private static long rechargeWorld;
    private static int lastAttackTick;
    private static long freshUntil,sequence,expires,lastClick,pendingWorld,sender;
    private static Command pending=new Command(0,0,0);
    private SkyrimAttack() {}
    public static void clear() {target=0;targetKind=0;freshUntil=0;pending=new Command(0,0,0);}
    /**
     * True while Skyrim's crosshair is freshly reported on something Skyrim's own E acts on:
     * an object, container, character, body or door (kinds 1 to 5). Nothing (0) and an enemy
     * (6) are not, and neither is any kind this version does not know.
     */
    public static boolean activates() {
        // Switched off in switches.txt: containers and bodies only, as in 0.10.18.
        return System.nanoTime()<freshUntil && target!=0 && target!=-1
            && BridgeRules.skyrimActsOn(targetKind,Switches.on(Switches.E_PRECEDENCE));
    }
    public static void observe(ByteBuffer b) {
        if(b==null || b.getInt(120)!=1 || b.getLong(72)<2250) {clear();return;}
        long next=b.getLong(8);
        if(next!=sender) {clear();sender=next;}
        target=b.getInt(NativeLink.WORLD_SIZE-48-528+12);
        targetKind=b.getInt(NativeLink.TARGET_KIND_AT);
        freshUntil=System.nanoTime()+Math.min(250,b.getLong(72)-2250)*1_000_000L;
    }
    private static boolean eligible() {
        var mc=Minecraft.getInstance();
        return mc.player!=null && mc.level!=null && BridgeWorld.eligible(mc.level)
            && !mc.player.isDeadOrDying() && !mc.player.isSpectator() && !mc.isPaused()
            && mc.isWindowActive() && mc.gui.screen()==null && SkyCollision.admitted()
            && !SkyrimMenus.active() && !BridgeRecovery.holding();
    }
    public static void click() {
        var mc=Minecraft.getInstance();long now=System.nanoTime();
        if(!eligible() || now>=freshUntil || target==0 || target==-1 || now-lastClick<100_000_000L) return;
        if(mc.hitResult!=null && mc.hitResult.getType()==HitResult.Type.ENTITY) return;
        if(mc.hitResult instanceof BlockHitResult hit && hit.getType()==HitResult.Type.BLOCK
            && !mc.level.getBlockState(hit.getBlockPos()).isAir()) return;
        var damageAttribute=net.minecraft.world.entity.ai.attributes.Attributes.ATTACK_DAMAGE;
        var held=mc.player.getMainHandItem();
        double base=mc.player.getAttributeBaseValue(damageAttribute);
        double observed=mc.player.getAttributeValue(damageAttribute);
        var modifiers=held.getOrDefault(net.minecraft.core.component.DataComponents.ATTRIBUTE_MODIFIERS,
            net.minecraft.world.item.component.ItemAttributeModifiers.EMPTY);
        // Start from base, not the client total: apply the held item's modifiers once.
        double attribute=modifiers.compute(damageAttribute,base,net.minecraft.world.entity.EquipmentSlot.MAINHAND);
        var speedAttribute=net.minecraft.world.entity.ai.attributes.Attributes.ATTACK_SPEED;
        double speed=modifiers.compute(speedAttribute,mc.player.getAttributeBaseValue(speedAttribute),
            net.minecraft.world.entity.EquipmentSlot.MAINHAND);
        // What the item's own numbers leave out, with Minecraft's values: Sharpness adds
        // 0.5 per level plus 0.5; Strength adds 3 per level and Weakness takes 4 per level.
        // Smite, Bane of Arthropods and Impaling are tied to kinds of Minecraft creature.
        // The numbers themselves are in BridgeRules, where they are checked before every build.
        int sharpnessLevel=0;
        for(var entry:held.getEnchantments().entrySet())
            if(entry.getKey().is(net.minecraft.world.item.enchantment.Enchantments.SHARPNESS)) sharpnessLevel=entry.getIntValue();
        double sharpness=BridgeRules.sharpnessBonus(sharpnessLevel);
        var strength=mc.player.getEffect(net.minecraft.world.effect.MobEffects.STRENGTH);
        var weakness=mc.player.getEffect(net.minecraft.world.effect.MobEffects.WEAKNESS);
        double effects=BridgeRules.effectBonus(strength==null ? 0 : strength.getAmplifier()+1,weakness==null ? 0 : weakness.getAmplifier()+1);
        double strong=BridgeRules.weaponDamage(attribute,effects);
        double fullDamage=BridgeRules.fullDamage(strong,sharpness);
        if(!Double.isFinite(speed) || speed<=0 || !Double.isFinite(fullDamage)
            || fullDamage<=0 || fullDamage>1000) return;
        long currentWorld=PassthroughClient.worldId();
        boolean first=rechargePlayer!=mc.player || rechargeWorld!=currentWorld;
        long elapsedTicks=first ? -1 : Math.max(0L,(long)mc.player.tickCount-lastAttackTick);
        double charge=BridgeRules.charge(elapsedTicks,speed);
        double multiplier=BridgeRules.rechargeMultiplier(charge);
        // Minecraft's own critical-hit rule: a charged hit while falling, not on the ground,
        // not climbing, swimming, sprinting, riding or blinded.
        var player=mc.player;
        boolean critical=charge>BridgeRules.CRITICAL_CHARGE && player.fallDistance>0 && !player.onGround() && !player.onClimbable()
            && !player.isInWater() && !player.isSprinting() && !player.isPassenger()
            && !player.hasEffect(net.minecraft.world.effect.MobEffects.BLINDNESS);
        // As in Minecraft's own attack: recharge and critical act on the weapon's damage, and
        // the enchantment bonus follows the recharge in a straight line and is never critical.
        float requested=(float)BridgeRules.attackDamage(strong,sharpness,charge,critical);
        if(!Float.isFinite(requested) || requested<=0) return;
        // Consume charge only for a request; item switches and stale snapshots do not reset it.
        rechargePlayer=mc.player;rechargeWorld=currentWorld;lastAttackTick=mc.player.tickCount;
        lastClick=now;pendingWorld=PassthroughClient.worldId();expires=now+200_000_000L;
        pending=new Command(++sequence,target,requested);
        if(critical) player.playSound(net.minecraft.sounds.SoundEvents.PLAYER_ATTACK_CRIT,1.0F,1.0F);
        wear(held,sequence);
        LoggerFactory.getLogger("PassthroughPractice").info("event=SCENE_ATTACK_SEND sequence={} handle={} world={} mode=held_item_recharge item={} base={} client_attribute={} component_modifiers={} attribute={} sharpness={} effects={} speed={} elapsed_ticks={} charge={} multiplier={} scale={} critical={} full_damage={} requested={}",sequence,Integer.toUnsignedString(target),pendingWorld,net.minecraft.core.registries.BuiltInRegistries.ITEM.getKey(held.getItem()),base,observed,modifiers.modifiers().size(),attribute,sharpness,effects,speed,elapsedTicks,charge,multiplier,BridgeRules.DAMAGE_SCALE,critical,fullDamage,requested);
    }
    /** Handle value for a shot: there is no target yet, Skyrim works out what the shot hits. */
    public static final int RANGED_HANDLE=0xFFFFFFFE;
    /**
     * The player let go of a drawn bow. With an arrow available and the bow drawn at least a
     * tenth, as Minecraft itself requires, one shot is sent with Minecraft's arrow damage for
     * that draw; Skyrim finds what it hits along the view. Minecraft's own arrow still flies
     * and is used up as usual.
     */
    public static void shot(net.minecraft.world.item.BowItem bow,net.minecraft.world.item.ItemStack stack,
        net.minecraft.world.entity.LivingEntity shooter,int timeLeft) {
        var mc=Minecraft.getInstance();long now=System.nanoTime();
        if(shooter!=mc.player || !Switches.on(Switches.RANGED) || !eligible() || now-lastClick<100_000_000L) return;
        if(mc.player.getProjectile(stack).isEmpty()) return;
        float draw=net.minecraft.world.item.BowItem.getPowerForTime(bow.getUseDuration(stack,shooter)-timeLeft);
        int power=0;
        for(var entry:stack.getEnchantments().entrySet())
            if(entry.getKey().is(net.minecraft.world.item.enchantment.Enchantments.POWER)) power=entry.getIntValue();
        float requested=(float)BridgeRules.arrowDamage(draw,power);
        if(!Float.isFinite(requested) || requested<=0) return;
        lastClick=now;pendingWorld=PassthroughClient.worldId();expires=now+200_000_000L;
        pending=new Command(++sequence,RANGED_HANDLE,requested);
        LoggerFactory.getLogger("PassthroughPractice").info("event=SCENE_SHOT_SEND sequence={} world={} draw={} power_level={} scale={} requested={}",
            sequence,pendingWorld,draw,power,BridgeRules.DAMAGE_SCALE,requested);
    }
    /**
     * Minecraft's own wear for one weapon hit, applied on the integrated server so Unbreaking,
     * creative mode and breaking behave as usual. Charged when the attack is sent: Skyrim
     * sends no "hit landed" reply.
     */
    private static void wear(net.minecraft.world.item.ItemStack held,long number) {
        var mc=Minecraft.getInstance();
        var weapon=held.get(net.minecraft.core.component.DataComponents.WEAPON);
        var server=mc.getSingleplayerServer();
        if(weapon==null || weapon.itemDamagePerAttack()<=0 || server==null || mc.player==null) return;
        int amount=weapon.itemDamagePerAttack();
        var id=mc.player.getUUID();var item=held.getItem();
        server.execute(()->{
            var owner=server.getPlayerList().getPlayer(id);
            if(owner==null || owner.isDeadOrDying()) return;
            var stack=owner.getMainHandItem();
            // Only the same kind of item still in hand; a switched item is left alone.
            if(stack.isEmpty() || stack.getItem()!=item) return;
            int before=stack.getDamageValue();
            stack.hurtAndBreak(amount,owner,net.minecraft.world.entity.EquipmentSlot.MAINHAND);
            LoggerFactory.getLogger("PassthroughPractice").info("event=SCENE_ATTACK_WEAR sequence={} item={} amount={} damage_before={} damage_after={} broken={}",
                number,net.minecraft.core.registries.BuiltInRegistries.ITEM.getKey(item),amount,before,stack.isEmpty() ? -1 : stack.getDamageValue(),stack.isEmpty());
        });
    }
    public static Command frame(boolean active) {
        if(!active || !eligible() || pendingWorld!=PassthroughClient.worldId() || System.nanoTime()>=expires) pending=new Command(0,0,0);
        return pending;
    }
}
