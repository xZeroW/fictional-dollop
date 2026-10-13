mod components;
mod crafting;
mod drops;
mod equipment;
mod pickup;
mod storage;

use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

pub use components::{ItemDrop, ItemRarity, PickupRadius};
pub use crafting::{CraftingAffix, CraftingMaterial, CraftingMaterials, InventoryItem};
pub use equipment::{
    Equipment, EquipmentSlot, move_equipment_item_to_safe, move_safe_item_to_equipment,
};
pub use storage::{
    RunInventory, SAFE_INVENTORY_CAPACITY, SafeInventory, move_run_item_to_safe,
    move_safe_item_to_run,
};

#[derive(AutoPlugin)]
pub(crate) struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    #[auto_plugin]
    fn build(&self, app: &mut App) {
        app.add_plugins((
            crafting::CraftingSystemsPlugin,
            equipment::EquipmentSystemsPlugin,
            storage::InventorySystemsPlugin,
            pickup::LootSystemsPlugin,
            drops::LootListenerPlugin,
        ));
    }
}
