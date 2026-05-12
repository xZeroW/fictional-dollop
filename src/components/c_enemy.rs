use bevy::prelude::Component;

use crate::components::c_movement::Movement;
use crate::components::damage::Damage;
use crate::components::health::Health;

#[derive(Component, Default)]
#[require(Health, Movement, Damage)]
pub struct Enemy {
    pub enemy_type: String,
}

impl Enemy {
    pub fn new(enemy_type: String) -> Self {
        Enemy { enemy_type }
    }
}
