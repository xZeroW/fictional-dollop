use bevy::prelude::*;

use crate::{components::ItemRarity, screens::Screen};

use super::crafting::CraftingAffix;

pub const SAFE_INVENTORY_CAPACITY: usize = 20;

#[derive(Debug, Clone, PartialEq, Eq, Reflect)]
pub struct InventoryItem {
    pub item_id: String,
    pub rarity: ItemRarity,
    pub quality: u8,
    pub affixes: Vec<CraftingAffix>,
}

impl InventoryItem {
    pub fn new(item_id: impl Into<String>, rarity: ItemRarity) -> Self {
        Self {
            item_id: item_id.into(),
            rarity,
            quality: 0,
            affixes: Vec::new(),
        }
    }
}

#[derive(Resource, Default, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct RunInventory {
    items: Vec<InventoryItem>,
}

impl RunInventory {
    pub fn add_item(&mut self, item_id: impl Into<String>, rarity: ItemRarity) {
        self.items.push(InventoryItem::new(item_id, rarity));
    }

    pub fn push(&mut self, item: InventoryItem) {
        self.items.push(item);
    }

    pub fn take(&mut self, index: usize) -> Option<InventoryItem> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn items(&self) -> &[InventoryItem] {
        &self.items
    }

    pub fn item_mut(&mut self, index: usize) -> Option<&mut InventoryItem> {
        self.items.get_mut(index)
    }

    pub fn summary(&self) -> String {
        let mut rarity_totals = [0; ItemRarity::ALL.len()];

        for item in &self.items {
            rarity_totals[item.rarity.index()] += 1;
        }

        let rarity_summary = ItemRarity::ALL
            .into_iter()
            .zip(rarity_totals)
            .map(|(rarity, quantity)| format!("{} {quantity}", rarity.label()))
            .collect::<Vec<_>>()
            .join("  ");

        format!("Loot {} | {rarity_summary}", self.items.len())
    }
}

#[derive(Resource, Default, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct SafeInventory {
    items: Vec<InventoryItem>,
}

impl SafeInventory {
    pub fn try_push(&mut self, item: InventoryItem) -> Result<(), InventoryItem> {
        if self.is_full() {
            Err(item)
        } else {
            self.items.push(item);
            Ok(())
        }
    }

    pub fn take(&mut self, index: usize) -> Option<InventoryItem> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    pub fn items(&self) -> &[InventoryItem] {
        &self.items
    }

    pub fn item_mut(&mut self, index: usize) -> Option<&mut InventoryItem> {
        self.items.get_mut(index)
    }

    pub fn is_full(&self) -> bool {
        self.items.len() >= SAFE_INVENTORY_CAPACITY
    }
}

pub fn move_run_item_to_safe(
    run_inventory: &mut RunInventory,
    safe_inventory: &mut SafeInventory,
    index: usize,
) -> bool {
    if safe_inventory.is_full() {
        return false;
    }

    let Some(item) = run_inventory.take(index) else {
        return false;
    };

    if let Err(item) = safe_inventory.try_push(item) {
        run_inventory.push(item);
        false
    } else {
        true
    }
}

pub fn move_safe_item_to_run(
    run_inventory: &mut RunInventory,
    safe_inventory: &mut SafeInventory,
    index: usize,
) -> bool {
    let Some(item) = safe_inventory.take(index) else {
        return false;
    };

    run_inventory.push(item);
    true
}

pub(super) struct InventorySystemsPlugin;

impl Plugin for InventorySystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Gameplay), reset_run_inventory);
        app.add_systems(OnExit(Screen::Gameplay), remove_run_inventory);
    }
}

fn reset_run_inventory(mut commands: Commands) {
    commands.insert_resource(RunInventory::default());
    commands.insert_resource(SafeInventory::default());
}

fn remove_run_inventory(mut commands: Commands) {
    commands.remove_resource::<RunInventory>();
    commands.remove_resource::<SafeInventory>();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(id: usize) -> InventoryItem {
        InventoryItem::new(format!("item_{id}"), ItemRarity::Common)
    }

    #[test]
    fn run_inventory_accepts_unbounded_items() {
        let mut inventory = RunInventory::default();

        for index in 0..(SAFE_INVENTORY_CAPACITY + 5) {
            inventory.push(item(index));
        }

        assert_eq!(inventory.items().len(), SAFE_INVENTORY_CAPACITY + 5);
    }

    #[test]
    fn safe_inventory_rejects_items_after_capacity() {
        let mut inventory = SafeInventory::default();

        for index in 0..SAFE_INVENTORY_CAPACITY {
            assert!(inventory.try_push(item(index)).is_ok());
        }

        assert!(inventory.is_full());
        assert_eq!(
            inventory.try_push(item(100)).unwrap_err().item_id,
            "item_100"
        );
        assert_eq!(inventory.items().len(), SAFE_INVENTORY_CAPACITY);
    }

    #[test]
    fn failed_safe_transfer_keeps_item_in_run_inventory() {
        let mut run_inventory = RunInventory::default();
        let mut safe_inventory = SafeInventory::default();

        run_inventory.push(item(100));
        for index in 0..SAFE_INVENTORY_CAPACITY {
            safe_inventory.try_push(item(index)).unwrap();
        }

        assert!(!move_run_item_to_safe(
            &mut run_inventory,
            &mut safe_inventory,
            0,
        ));
        assert_eq!(run_inventory.items()[0].item_id, "item_100");
        assert_eq!(safe_inventory.items().len(), SAFE_INVENTORY_CAPACITY);
    }

    #[test]
    fn safe_to_run_transfer_is_unbounded() {
        let mut run_inventory = RunInventory::default();
        let mut safe_inventory = SafeInventory::default();

        safe_inventory.try_push(item(7)).unwrap();

        assert!(move_safe_item_to_run(
            &mut run_inventory,
            &mut safe_inventory,
            0,
        ));
        assert!(safe_inventory.items().is_empty());
        assert_eq!(run_inventory.items()[0].item_id, "item_7");
    }
}
