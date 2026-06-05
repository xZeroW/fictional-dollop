use bevy::prelude::*;
use std::collections::HashMap;

use crate::{components::ItemRarity, screens::Screen};

#[derive(Debug, Clone, Reflect)]
pub struct ItemStack {
    pub rarity: ItemRarity,
    pub quantity: u32,
}

#[derive(Resource, Default, Debug, Clone, Reflect)]
pub struct RunInventory {
    items: HashMap<String, ItemStack>,
}

impl RunInventory {
    pub fn add_item(&mut self, item_id: impl Into<String>, rarity: ItemRarity, quantity: u32) {
        if quantity == 0 {
            return;
        }

        let stack = self
            .items
            .entry(item_id.into())
            .or_insert_with(|| ItemStack {
                rarity,
                quantity: 0,
            });
        stack.quantity = stack.quantity.saturating_add(quantity);
    }

    pub fn summary(&self) -> String {
        let mut total_items = 0;
        let mut rarity_totals = [0; ItemRarity::ALL.len()];

        for stack in self.items.values() {
            total_items += stack.quantity;
            rarity_totals[stack.rarity.index()] += stack.quantity;
        }

        let rarity_summary = ItemRarity::ALL
            .into_iter()
            .zip(rarity_totals)
            .map(|(rarity, quantity)| format!("{} {quantity}", rarity.label()))
            .collect::<Vec<_>>()
            .join("  ");

        format!("Loot {total_items} | {rarity_summary}")
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
