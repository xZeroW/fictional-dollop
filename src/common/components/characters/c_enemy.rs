use bevy::prelude::Component;

use crate::common::components::characters::c_movement::Movement;
use crate::common::components::characters::damage::Damage;
use crate::common::components::characters::health::Health;

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
