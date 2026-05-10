use bevy::prelude::Component;

#[derive(Component, Default)]
pub struct Movement {
    pub speed: f32,
}

impl Movement {
    pub fn new(speed: f32) -> Self {
        Movement { speed }
    }
}
