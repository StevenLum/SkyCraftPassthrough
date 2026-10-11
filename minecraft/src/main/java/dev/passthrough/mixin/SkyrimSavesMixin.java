package dev.passthrough.mixin;

import dev.passthrough.SkyrimInteraction;
import dev.passthrough.SkyrimMenus;
import dev.passthrough.SkyrimSaves;
import net.minecraft.client.Minecraft;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;

/** Runs before vanilla reads its key clicks, so a shared key cannot also toggle the camera. */
@Mixin(Minecraft.class)
public abstract class SkyrimSavesMixin {
    @Inject(method="handleKeybinds",at=@At("HEAD"))
    private void passthrough$saves(CallbackInfo ci) {
        SkyrimSaves.beforeKeybinds();SkyrimMenus.beforeKeybinds();SkyrimInteraction.beforeKeybinds();
        dev.passthrough.SkyrimShortcuts.beforeKeybinds();
    }
}
