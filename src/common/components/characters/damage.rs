use bevy::prelude::Component;

#[derive(Component, Default)]
pub struct Damage {
    pub value: f32,
}

impl Damage {
    pub fn new(value: f32) -> Self {
        Damage { value }
    }
}