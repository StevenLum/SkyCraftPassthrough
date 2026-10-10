package dev.passthrough;

import java.nio.ByteBuffer;
import net.minecraft.client.Minecraft;
import net.minecraft.world.phys.BlockHitResult;
import net.minecraft.world.phys.HitResult;
import org.slf4j.LoggerFactory;

/** Observe vanilla attacks without consuming clicks or cancelling block breaking. */
public final class SkyrimAttack {
    public record Command(long sequence,int handle,float damage) {}
    /** Skyrim damage per point of Minecraft attack damage; lines sword tiers up with Skyrim's own swords. */
    private static final double DAMAGE_SCALE=1.75;
    /** Minecraft's own bonus for a critical hit. */
    private static final double CRITICAL_SCALE=1.5;
    private static int target,targetKind;
    private static net.minecraft.client.player.LocalPlayer rechargePlayer;
    private static long rechargeWorld;
    private static int lastAttackTick;
    private static long freshUntil,sequence,expires,lastClick,pendingWorld,sender;
    private static Command pending=new Command(0,0,0);
    private SkyrimAttack() {}
    public static void clear() {target=0;targetKind=0;freshUntil=0;pending=new Command(0,0,0);}
    /** True while Skyrim's crosshair is freshly reported on a container or a dead body. */
    public static boolean lootTarget() {
        return System.nanoTime()<freshUntil && target!=0 && target!=-1 && (targetKind==2 || targetKind==4);
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
        double fullDamage=attribute*DAMAGE_SCALE;
        if(!Double.isFinite(speed) || speed<=0 || !Double.isFinite(fullDamage)
            || fullDamage<=0 || fullDamage>1000) return;
        long currentWorld=PassthroughClient.worldId();
        boolean first=rechargePlayer!=mc.player || rechargeWorld!=currentWorld;
        long elapsedTicks=first ? -1 : Math.max(0L,(long)mc.player.tickCount-lastAttackTick);
        double charge=first ? 1.0 : Math.min(1.0,(elapsedTicks+0.5)*speed/20.0);
        double multiplier=0.2+0.8*charge*charge;
        // Minecraft's own critical-hit rule: a charged hit while falling, not on the ground,
        // not climbing, swimming, sprinting, riding or blinded.
        var player=mc.player;
        boolean critical=charge>0.9 && player.fallDistance>0 && !player.onGround() && !player.onClimbable()
            && !player.isInWater() && !player.isSprinting() && !player.isPassenger()
            && !player.hasEffect(net.minecraft.world.effect.MobEffects.BLINDNESS);
        float requested=(float)(fullDamage*multiplier*(critical ? CRITICAL_SCALE : 1.0));
        if(!Float.isFinite(requested) || requested<=0) return;
        // Consume charge only for a request; item switches and stale snapshots do not reset it.
        rechargePlayer=mc.player;rechargeWorld=currentWorld;lastAttackTick=mc.player.tickCount;
        lastClick=now;pendingWorld=PassthroughClient.worldId();expires=now+200_000_000L;
        pending=new Command(++sequence,target,requested);
        if(critical) player.playSound(net.minecraft.sounds.SoundEvents.PLAYER_ATTACK_CRIT,1.0F,1.0F);
        wear(held,sequence);
        LoggerFactory.getLogger("PassthroughPractice").info("event=SCENE_ATTACK_SEND sequence={} handle={} world={} mode=held_item_recharge item={} base={} client_attribute={} component_modifiers={} attribute={} speed={} elapsed_ticks={} charge={} multiplier={} scale={} critical={} full_damage={} requested={}",sequence,Integer.toUnsignedString(target),pendingWorld,net.minecraft.core.registries.BuiltInRegistries.ITEM.getKey(held.getItem()),base,observed,modifiers.modifiers().size(),attribute,speed,elapsedTicks,charge,multiplier,DAMAGE_SCALE,critical,fullDamage,requested);
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
