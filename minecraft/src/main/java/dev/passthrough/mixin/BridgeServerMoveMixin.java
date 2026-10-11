package dev.passthrough.mixin;
import dev.passthrough.BridgeWorld;
import dev.passthrough.SkyCollision;
import dev.passthrough.Switches;
import net.minecraft.server.network.ServerGamePacketListenerImpl;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.network.protocol.game.ServerboundMovePlayerPacket;
import net.minecraft.world.phys.Vec3;
import org.spongepowered.asm.mixin.*;
import org.spongepowered.asm.mixin.injection.*;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;
/**
 * The local server checks every step the player reports and sends the player back when the
 * new position touches a collision shape the old one did not, or when its own replay of the
 * step ends elsewhere. Skyrim's shapes change under the player (a character's stand-in box
 * moves several times a second, doors swing), so those checks would undo real steps. In a
 * bridge world the client is the side that moves against Skyrim's shapes; while the server
 * checks a reported step, they are left out. Minecraft's own blocks are still checked.
 */
@Mixin(ServerGamePacketListenerImpl.class)
public abstract class BridgeServerMoveMixin {
    @Shadow public ServerPlayer player;
    @Inject(method="handleMovePlayer",at=@At("HEAD"),cancellable=true)
    private void passthrough$check(ServerboundMovePlayerPacket packet,CallbackInfo ci) {
        // Network dispatch first re-enters on the server thread; never mutate players on Netty.
        if(!player.level().getServer().isSameThread()) return;
        if(Switches.on(Switches.SERVER_TRUST)) {
            if(BridgeWorld.eligible(player.level())) SkyCollision.quiet(SkyCollision.SERVER_CHECK,true);
            return;
        }
        // Switched off in switches.txt: as before 0.10.38, the server holds and drops steps itself.
        var movement=new Vec3(packet.getX(player.getX())-player.getX(),packet.getY(player.getY())-player.getY(),packet.getZ(player.getZ())-player.getZ());
        if(BridgeWorld.hold(player,movement)) ci.cancel();
    }
    @Inject(method="handleMovePlayer",at=@At("RETURN"))
    private void passthrough$checked(ServerboundMovePlayerPacket packet,CallbackInfo ci) {
        SkyCollision.quiet(SkyCollision.SERVER_CHECK,false);
    }
}
