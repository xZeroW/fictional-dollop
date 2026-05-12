use bevy::prelude::Component;

use crate::components::{c_movement::Movement, health::Health};

#[derive(Component)]
#[require(Health, Movement)]
pub struct Player;
