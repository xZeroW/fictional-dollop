mod animation;
mod auto_attack;
mod bullet;
mod collision;
mod crafting;
mod enemy_spatial;
mod equipment;
mod flip_sprite;
mod health;
mod inventory;
mod loot;
mod monster_progression;
mod movement;
mod wave;

use bevy::prelude::*;

pub(crate) use animation::PlayerAnimation;
pub(crate) use crafting::{CraftingAffix, CraftingMaterial, CraftingMaterials};
pub(crate) use equipment::{
    Equipment, EquipmentSlot, move_equipment_item_to_safe, move_safe_item_to_equipment,
};
pub(crate) use inventory::{
    InventoryItem, RunInventory, SAFE_INVENTORY_CAPACITY, SafeInventory, move_run_item_to_safe,
    move_safe_item_to_run,
};
pub(crate) use monster_progression::{MONSTER_BUFF_CHOICES, MonsterBuff, MonsterProgression};
pub(crate) use wave::WaveState;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            animation::AnimationSystemsPlugin,
            auto_attack::AutoAttackSystemsPlugin,
            bullet::BulletSystemsPlugin,
            collision::CollisionSystemsPlugin,
            crafting::CraftingSystemsPlugin,
            enemy_spatial::EnemySpatialPlugin,
            equipment::EquipmentSystemsPlugin,
            health::HealthSystemsPlugin,
            inventory::InventorySystemsPlugin,
            loot::LootSystemsPlugin,
            monster_progression::MonsterProgressionPlugin,
            movement::MovementSystemsPlugin,
            wave::WaveSystemsPlugin,
        ));
        app.add_systems(
            Update,
            flip_sprite::flip_sprite.in_set(crate::PausableSystems),
        );
    }
}
