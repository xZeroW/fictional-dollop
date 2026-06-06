use bevy::prelude::*;

use crate::{components::ItemRarity, screens::Screen};

#[derive(Debug, Clone, Reflect)]
pub struct InventoryItem {
    pub item_id: String,
    pub rarity: ItemRarity,
}

#[derive(Resource, Default, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct RunInventory {
    items: Vec<InventoryItem>,
}

impl RunInventory {
    pub fn add_item(&mut self, item_id: impl Into<String>, rarity: ItemRarity) {
        self.items.push(InventoryItem {
            item_id: item_id.into(),
            rarity,
        });
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

pub(super) struct InventorySystemsPlugin;

impl Plugin for InventorySystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Gameplay), reset_run_inventory);
        app.add_systems(OnExit(Screen::Gameplay), remove_run_inventory);
    }
}

fn reset_run_inventory(mut commands: Commands) {
    commands.insert_resource(RunInventory::default());
}

fn remove_run_inventory(mut commands: Commands) {
    commands.remove_resource::<RunInventory>();
}
