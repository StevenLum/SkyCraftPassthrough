package dev.passthrough.mixin;
import dev.passthrough.BridgeInventoryLog;
import net.minecraft.server.players.PlayerList;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.network.Connection;
import net.minecraft.server.network.CommonListenerCookie;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.*;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;
@Mixin(PlayerList.class)
public abstract class BridgeInventoryMixin {
    @Inject(method="placeNewPlayer",at=@At("RETURN"))
    private void passthrough$loaded(Connection connection,ServerPlayer player,CommonListenerCookie cookie,CallbackInfo ci) {
        BridgeInventoryLog.write(player,"join_return");
    }
    @Inject(method="save",at=@At("RETURN"))
    private void passthrough$saved(ServerPlayer player,CallbackInfo ci) {
        BridgeInventoryLog.write(player,"save_return");
    }
}
