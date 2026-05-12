use bevy::{prelude::Component, reflect::Reflect};

#[derive(Component, Reflect)]
pub struct Movement {
    pub speed: f32,
}

impl Default for Movement {
    fn default() -> Self {
        Self { speed: 150.0 }
    }
}

impl Movement {
    pub fn new(speed: f32) -> Self {
        Movement { speed }
    }
}
