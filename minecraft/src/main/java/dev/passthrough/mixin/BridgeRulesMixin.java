package dev.passthrough.mixin;
import dev.passthrough.BridgeWorld;
import net.minecraft.client.server.IntegratedServer;
import net.minecraft.server.MinecraftServer;
import net.minecraft.world.level.gamerules.GameRules;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.*;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;
@Mixin(MinecraftServer.class)
public abstract class BridgeRulesMixin {
    @Inject(method="loadLevel",at=@At("TAIL"))
    private void passthrough$rules(CallbackInfo ci) {
        if((Object)this instanceof IntegratedServer server && BridgeWorld.marked(server)) {
            var rules=server.getGameRules();
            rules.set(GameRules.SPAWN_MOBS,false,server);
            rules.set(GameRules.SPAWN_PATROLS,false,server);
            rules.set(GameRules.SPAWN_PHANTOMS,false,server);
            rules.set(GameRules.SPAWN_WANDERING_TRADERS,false,server);
            rules.set(GameRules.RESPAWN_RADIUS,0,server);
            // Death is shown and resolved by Skyrim: no Minecraft death screen, no item drop.
            rules.set(GameRules.IMMEDIATE_RESPAWN,true,server);
            rules.set(GameRules.KEEP_INVENTORY,true,server);
        }
    }
}
