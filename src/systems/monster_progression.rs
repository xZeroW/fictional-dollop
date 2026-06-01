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

impl MonsterProgression {
    pub fn apply(&mut self, buff: MonsterBuff) {
        self.corruption += 1;
        self.enemy_health_mult *= buff.enemy_health_mult;
        self.enemy_damage_mult *= buff.enemy_damage_mult;
        self.enemy_speed_mult *= buff.enemy_speed_mult;
        self.reward_quantity_mult *= buff.reward_quantity_mult;
        self.reward_rarity_mult *= buff.reward_rarity_mult;
    }
}

#[derive(Clone, Copy)]
pub struct MonsterBuff {
    pub title: &'static str,
    pub danger: &'static str,
    pub reward: &'static str,
    pub enemy_health_mult: f32,
    pub enemy_damage_mult: f32,
    pub enemy_speed_mult: f32,
    pub reward_quantity_mult: f32,
    pub reward_rarity_mult: f32,
}

pub const MONSTER_BUFF_CHOICES: [MonsterBuff; 3] = [
    MonsterBuff {
        title: "Thick Blood",
        danger: "Enemies gain +25% health",
        reward: "Future loot quantity +15%",
        enemy_health_mult: 1.25,
        enemy_damage_mult: 1.0,
        enemy_speed_mult: 1.0,
        reward_quantity_mult: 1.15,
        reward_rarity_mult: 1.0,
    },
    MonsterBuff {
        title: "Frenzy",
        danger: "Enemies gain +20% speed",
        reward: "Future loot rarity +15%",
        enemy_health_mult: 1.0,
        enemy_damage_mult: 1.0,
        enemy_speed_mult: 1.20,
        reward_quantity_mult: 1.0,
        reward_rarity_mult: 1.15,
    },
    MonsterBuff {
        title: "Cruel Claws",
        danger: "Enemies gain +25% damage",
        reward: "Future loot quantity +20%",
        enemy_health_mult: 1.0,
        enemy_damage_mult: 1.25,
        enemy_speed_mult: 1.0,
        reward_quantity_mult: 1.20,
        reward_rarity_mult: 1.0,
    },
];

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
