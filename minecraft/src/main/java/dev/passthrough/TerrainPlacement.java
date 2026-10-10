package dev.passthrough;

import java.util.ArrayDeque;
import java.util.UUID;
import net.minecraft.client.Minecraft;
import net.minecraft.client.server.IntegratedServer;
import net.minecraft.world.entity.player.Player;
import net.minecraft.world.item.BlockItem;
import net.minecraft.world.item.Item;
import net.minecraft.world.item.context.UseOnContext;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.ClipContext;
import net.minecraft.world.phys.HitResult;
import net.minecraft.world.phys.AABB;
import org.slf4j.LoggerFactory;

/** Offline contact metadata; vanilla packets still own prediction and placement. */
public final class TerrainPlacement {
    private record Candidate(Level level,IntegratedServer server,UUID player,SkyCollision.TerrainHit target,long expires) {}
    private record Intent(long id,Candidate candidate,InteractionHand hand,Item item,long expires) {}
    private static final ArrayDeque<Intent> pending=new ArrayDeque<>();
    private static volatile Candidate candidate;
    private static long sequence,lastTargetLog;
    private TerrainPlacement() {}
    public static synchronized void clear() { candidate=null; pending.clear(); }
    public static boolean local(Level level) {
        var mc=Minecraft.getInstance(); var server=mc.getSingleplayerServer();
        return server!=null && !server.isPublished()
            && (level.isClientSide()?level==mc.level:level.getServer()==server);
    }
    public static HitResult pick(Player player,double reach,float partial,boolean fluids,HitResult vanilla) {
        candidate=null;
        if(!local(player.level()) || fluids || !(supported(player.getMainHandItem().getItem())
            || supported(player.getOffhandItem().getItem()))) return vanilla;
        var start=player.getEyePosition(partial);
        var target=SkyCollision.target(player.level(),start,start.add(player.getViewVector(partial).scale(reach)));
        if(target==null) return vanilla;
        String decision="selected";
        if(vanilla.getType()!=HitResult.Type.MISS
            && start.distanceToSqr(vanilla.getLocation())<=start.distanceToSqr(target.surface())) decision="nearer_vanilla";
        else if(!player.level().getBlockState(target.hit().getBlockPos()).isAir()) decision="cell_not_air";
        long now=System.nanoTime();
        if(now-lastTargetLog>1_000_000_000L) {
            lastTargetLog=now;
            LoggerFactory.getLogger("PassthroughPractice").info(
                "event=SCENE_TARGET sender={} epoch={} snapshot={} point={} surface={} face={} cell={} decision={} eye={} reach={} surface_distance={} mapping=contact_cell",
                target.sender(),target.epoch(),target.snapshot(),target.hit().getLocation(),target.surface(),
                target.hit().getDirection(),target.hit().getBlockPos(),decision,start,reach,start.distanceTo(target.surface()));
        }
        if(!decision.equals("selected")) return vanilla;
        candidate=new Candidate(player.level(),Minecraft.getInstance().getSingleplayerServer(),player.getUUID(),target,now+500_000_000L);
        return target.hit();
    }
    private static boolean supported(Item item) {
        return item instanceof BlockItem || TerrainBucket.supported(item);
    }
    private static boolean matches(Candidate c,UseOnContext context) {
        var player=context.getPlayer(); var level=context.getLevel();
        return player!=null && c.player.equals(player.getUUID()) && c.server==Minecraft.getInstance().getSingleplayerServer()
            && (level.isClientSide()?level==c.level:level.getServer()==c.server && level.dimension().equals(c.level.dimension()))
            && c.target.hit().getBlockPos().equals(context.getClickedPos())
            && c.target.hit().getDirection()==context.getClickedFace()
            && c.target.hit().getLocation().distanceToSqr(context.getClickLocation())<1e-8;
    }
    private static synchronized Intent consume(UseOnContext context,long now) {
        pending.removeIf(i->i.expires<now || i.candidate.server!=Minecraft.getInstance().getSingleplayerServer());
        var it=pending.iterator();
        while(it.hasNext()) {
            var intent=it.next();
            if(intent.hand==context.getHand() && intent.item==context.getItemInHand().getItem() && matches(intent.candidate,context)) {
                it.remove(); return intent;
            }
        }
        return null;
    }
    public static boolean validate(UseOnContext context) {
        long now=System.nanoTime(); long id=0;
        var level=context.getLevel(); Candidate selected;
        if(level.isClientSide()) {
            selected=candidate;
            if(selected==null || selected.expires<now || !matches(selected,context)) return reject(context,"no_current_intent");
        } else {
            var intent=consume(context,now);
            if(intent==null) return reject(context,"missing_or_expired_intent");
            selected=intent.candidate; id=intent.id;
        }
        var original=selected.target;
        var player=context.getPlayer();
        var start=player.getEyePosition();
        var delta=original.surface().subtract(start);
        var end=original.surface().add(delta.normalize().scale(.02));
        var target=SkyCollision.target(level,start,end);
        String reason="accepted";
        if(delta.lengthSqr()>Math.pow(player.blockInteractionRange()+.1,2)) reason="reach";
        else if(target==null) reason="no_fresh_surface";
        else if(target.sender()!=original.sender() || target.epoch()!=original.epoch()) reason="origin_changed";
        else if(!target.hit().getBlockPos().equals(context.getClickedPos())
            || target.hit().getDirection()!=context.getClickedFace()
            || target.surface().distanceToSqr(original.surface())>.01) reason="surface_changed";
        else {
            var normal=level.clip(new ClipContext(start,end,ClipContext.Block.OUTLINE,ClipContext.Fluid.NONE,player));
            if(normal.getType()!=HitResult.Type.MISS
                && start.distanceToSqr(normal.getLocation())+1e-6<start.distanceToSqr(target.surface())) reason="blocked_ray";
            else for(var shape:SkyCollision.query(level,new AABB(context.getClickedPos()).deflate(.001))) {
                if(shape.terrain()) { reason="occupied_terrain"; break; }
            }
        }
        if(level.isClientSide() && reason.equals("accepted")) {
            synchronized(TerrainPlacement.class) {
                pending.removeIf(i->i.expires<now || i.candidate.server!=selected.server);
                if(pending.size()>=16) reason="intent_capacity";
                else { id=++sequence; pending.addLast(new Intent(id,selected,context.getHand(),context.getItemInHand().getItem(),now+500_000_000L)); }
            }
        }
        LoggerFactory.getLogger("PassthroughPractice").info(
            "event=SCENE_PLACE side={} decision={} intent={} sender={} epoch={} snapshot={} point={} surface={} face={} cell={} eye={} surface_distance={}",
            level.isClientSide()?"client":"server",reason,id,target==null?0:target.sender(),
            target==null?0:target.epoch(),target==null?0:target.snapshot(),context.getClickLocation(),original.surface(),
            context.getClickedFace(),context.getClickedPos(),start,delta.length());
        return reason.equals("accepted");
    }
    private static boolean reject(UseOnContext context,String reason) {
        LoggerFactory.getLogger("PassthroughPractice").info("event=SCENE_PLACE side={} decision={} cell={}",
            context.getLevel().isClientSide()?"client":"server",reason,context.getClickedPos());
        return false;
    }
}
