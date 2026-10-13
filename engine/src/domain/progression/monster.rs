use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

use crate::Screen;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
#[auto_register_type(plugin = MonsterProgressionPlugin)]
pub struct MonsterProgression {
    pub corruption: u32,
    pub enemy_health_mult: f32,
    pub enemy_damage_mult: f32,
    pub enemy_speed_mult: f32,
    pub reward_quantity_mult: f32,
    pub reward_rarity_mult: f32,
}

impl Default for MonsterProgression {
    fn default() -> Self {
        Self {
            corruption: 0,
            enemy_health_mult: 1.0,
            enemy_damage_mult: 1.0,
            enemy_speed_mult: 1.0,
            reward_quantity_mult: 1.0,
            reward_rarity_mult: 1.0,
        }
    }
}

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
pub(super) struct MonsterProgressionPlugin;

#[auto_system(
    plugin = MonsterProgressionPlugin,
    schedule = OnEnter(Screen::Gameplay)
)]
fn reset_monster_progression(mut commands: Commands) {
    commands.insert_resource(MonsterProgression::default());
}

#[auto_system(
    plugin = MonsterProgressionPlugin,
    schedule = OnExit(Screen::Gameplay)
)]
fn remove_monster_progression(mut commands: Commands) {
    commands.remove_resource::<MonsterProgression>();
}
