package dev.passthrough.mixin;
import dev.passthrough.BridgeWorld;
import net.minecraft.server.network.ServerGamePacketListenerImpl;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.network.protocol.game.ServerboundMovePlayerPacket;
import net.minecraft.world.phys.Vec3;
import org.spongepowered.asm.mixin.*;
import org.spongepowered.asm.mixin.injection.*;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;
@Mixin(ServerGamePacketListenerImpl.class)
public abstract class BridgeServerMoveMixin {
    @Shadow public ServerPlayer player;
    @Inject(method="handleMovePlayer",at=@At("HEAD"),cancellable=true)
    private void passthrough$hold(ServerboundMovePlayerPacket packet,CallbackInfo ci) {
        // Network dispatch first re-enters on the server thread; never mutate players on Netty.
        if(!player.level().getServer().isSameThread()) return;
        var movement=new Vec3(packet.getX(player.getX())-player.getX(),packet.getY(player.getY())-player.getY(),packet.getZ(player.getZ())-player.getZ());
        if(BridgeWorld.hold(player,movement)) ci.cancel();
    }
}
