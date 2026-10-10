package dev.passthrough.mixin;
import dev.passthrough.BridgeWorld;
import dev.passthrough.BridgeSaves;
import net.minecraft.client.gui.screens.Screen;
import net.minecraft.client.gui.screens.worldselection.SelectWorldScreen;
import net.minecraft.client.gui.components.Button;
import net.minecraft.network.chat.Component;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.*;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;
@Mixin(SelectWorldScreen.class)
public abstract class BridgeWorldScreenMixin extends Screen {
    protected BridgeWorldScreenMixin() { super(Component.empty()); }
    @Inject(method="init",at=@At("TAIL"))
    private void passthrough$button(CallbackInfo ci) {
        addRenderableWidget(Button.builder(Component.literal("New Skyrim bridge"),b->BridgeSaves.confirmNew(this)).bounds(8,6,140,20).build());
        addRenderableWidget(Button.builder(Component.literal("Resume bridge"),b->BridgeSaves.resume(this,b)).bounds(width-148,6,140,20).build());
    }
}
