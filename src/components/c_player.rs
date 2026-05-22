use bevy::prelude::*;
use bevy_gauge::prelude::*;

use crate::components::{Health, Movement};

#[derive(Component, Debug, Clone, PartialEq, Reflect)]
#[reflect(Component)]
#[require(Attributes, Health, Movement)]
pub struct Player {
    pub weapon: String,
    pub attack_range: f32,
    pub last_shot_time: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            weapon: "dagger".to_string(),
            attack_range: 200.0,
            last_shot_time: 0.0,
        }
    }
}
