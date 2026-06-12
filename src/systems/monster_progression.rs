use bevy::prelude::*;

use crate::screens::Screen;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
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

pub(super) struct MonsterProgressionPlugin;

impl Plugin for MonsterProgressionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MonsterProgression>();
        app.add_systems(OnEnter(Screen::Gameplay), reset_monster_progression);
        app.add_systems(OnExit(Screen::Gameplay), remove_monster_progression);
    }
}

fn reset_monster_progression(mut commands: Commands) {
    commands.insert_resource(MonsterProgression::default());
}

fn remove_monster_progression(mut commands: Commands) {
    commands.remove_resource::<MonsterProgression>();
}
