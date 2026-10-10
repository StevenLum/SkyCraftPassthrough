package dev.passthrough.mixin;

import dev.passthrough.SkyrimAttack;
import dev.passthrough.SkyrimMenus;
import net.minecraft.client.Minecraft;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;

@Mixin(Minecraft.class)
public abstract class SkyrimAttackMixin {
    @Inject(method="startAttack",at=@At("HEAD"),cancellable=true)
    private void passthrough$attack(CallbackInfoReturnable<Boolean> ci) {
        // In a Skyrim menu the click confirms a choice; it must not also hit a Minecraft block.
        if(SkyrimMenus.click()) {ci.setReturnValue(false);return;}
        SkyrimAttack.click();
    }
    @Inject(method="continueAttack",at=@At("HEAD"),cancellable=true)
    private void passthrough$hold(boolean down,CallbackInfo ci) {
        // Also while the confirming press is still held after the menu has closed.
        if(down && (SkyrimMenus.active() || SkyrimMenus.confirmHeld())) ci.cancel();
    }
    @Inject(method="startUseItem",at=@At("HEAD"),cancellable=true)
    private void passthrough$use(CallbackInfo ci) {
        // In a Skyrim menu a right click goes back; it must not also use the held item.
        if(SkyrimMenus.back()) ci.cancel();
    }
}
