use bevy::prelude::*;

use crate::components::{Health, Movement};

#[derive(Component, Debug, Clone, PartialEq, Reflect)]
#[reflect(Component)]
#[require(Health, Movement)]
pub struct Player {
    pub weapon: String,
    pub weapon_entity: Option<Entity>,
    pub last_shot_time: f32,
    pub can_shoot_timer: Timer,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            weapon: "dagger".to_string(),
            weapon_entity: None,
            last_shot_time: 0.0,
            can_shoot_timer: Timer::from_seconds(0.2, TimerMode::Once),
        }
    }
}