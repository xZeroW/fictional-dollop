use bevy::prelude::*;

use crate::components::{Damage, Health, Movement};

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(Health, Movement, Damage, Behavior, WanderState)]
pub struct Enemy {
    pub enemy_type: String,
}

impl Enemy {
    pub fn new(enemy_type: String) -> Self {
        Enemy { enemy_type }
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub enum Behavior {
    #[default]
    Wandering,
    FollowAndAttack,
    Coward,
}

#[derive(Component)]
pub struct WanderState {
    pub direction: Vec2,
    pub timer: Timer,
}

impl Default for WanderState {
    fn default() -> Self {
        Self {
            direction: Vec2::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5)
                .normalize_or_zero(),
            timer: Timer::from_seconds(3.0, TimerMode::Repeating),
        }
    }
}
