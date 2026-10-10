package dev.passthrough.mixin;

import dev.passthrough.SkyrimInteraction;
import dev.passthrough.SkyrimMenus;
import dev.passthrough.SkyrimSaves;
import java.util.Arrays;
import net.minecraft.client.KeyMapping;
import net.minecraft.client.Options;
import org.spongepowered.asm.mixin.*;
import org.spongepowered.asm.mixin.injection.*;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;

/** Include the key before vanilla loads options, so rebinding persists normally. */
@Mixin(Options.class)
public abstract class InteractionOptionsMixin {
    @Shadow @Final @Mutable public KeyMapping[] keyMappings;
    @Inject(method="load",at=@At("HEAD"))
    private void passthrough$register(CallbackInfo ci) {
        var additions=Arrays.copyOf(SkyrimMenus.bindings(),10);
        additions[6]=SkyrimInteraction.binding();
        additions[7]=SkyrimSaves.bindings()[0];
        additions[8]=SkyrimSaves.bindings()[1];
        additions[9]=SkyrimMenus.pointerClickBinding();
        for(var key:additions) {
            boolean found=false;
            for(var existing:keyMappings) if(existing==key) found=true;
            if(!found) {
                keyMappings=Arrays.copyOf(keyMappings,keyMappings.length+1);
                keyMappings[keyMappings.length-1]=key;
            }
        }
    }
}
