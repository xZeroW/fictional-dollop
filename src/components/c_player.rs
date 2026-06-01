use bevy::prelude::*;
use bevy_gauge::prelude::*;

use crate::components::{Health, Movement};

#[derive(Component, Debug, Clone, PartialEq, Default, Reflect)]
#[reflect(Component)]
#[require(Attributes, Health, Movement)]
pub struct Player;
