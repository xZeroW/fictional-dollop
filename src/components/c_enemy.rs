use bevy::prelude::Component;

use crate::components::{Damage, Health, Movement};

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
