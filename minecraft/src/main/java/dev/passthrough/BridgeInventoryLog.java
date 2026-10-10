package dev.passthrough;

import net.minecraft.client.server.IntegratedServer;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.world.entity.EquipmentSlot;
import net.minecraft.world.level.storage.LevelResource;
import org.slf4j.LoggerFactory;

/** Read-only summaries. A save hook is not a disk-integrity check. */
public final class BridgeInventoryLog {
    private BridgeInventoryLog() {}
    public static void write(ServerPlayer player,String phase) {
        if(!(player.level().getServer() instanceof IntegratedServer server) || !BridgeWorld.marked(server)) return;
        StringBuilder slots=new StringBuilder(),equipment=new StringBuilder(); int count=0;
        var inventory=player.getInventory();
        for(int slot=0;slot<inventory.getContainerSize();slot++) {
            var stack=inventory.getItem(slot);
            if(stack.isEmpty()) continue;
            count+=stack.getCount();
            slots.append(slot).append(':').append(BuiltInRegistries.ITEM.getKey(stack.getItem())).append('*').append(stack.getCount()).append(';');
        }
        for(var slot:new EquipmentSlot[]{EquipmentSlot.MAINHAND,EquipmentSlot.OFFHAND,EquipmentSlot.HEAD,EquipmentSlot.CHEST,EquipmentSlot.LEGS,EquipmentSlot.FEET}) {
            var stack=player.getItemBySlot(slot);
            if(!stack.isEmpty()) equipment.append(slot).append(':').append(BuiltInRegistries.ITEM.getKey(stack.getItem())).append('*').append(stack.getCount()).append(';');
        }
        LoggerFactory.getLogger("PassthroughPractice").info(
            "event=SCENE_BRIDGE_INVENTORY phase={} save={} player={} inventory_count={} slots={} equipment={} xp_level={} xp_total={}",
            phase,server.getWorldPath(LevelResource.ROOT).normalize().getFileName(),player.getUUID(),count,slots,equipment,player.experienceLevel,player.totalExperience);
    }
}
