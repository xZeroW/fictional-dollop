use bevy::{math::Vec2, prelude::Component, reflect::Reflect};

#[derive(Component, Reflect)]
pub struct Movement {
    pub intent: Vec2,
    pub speed: f32,
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            intent: Vec2::ZERO,
            speed: 150.0,
        }
    }
}

impl Movement {
    pub fn new(speed: f32) -> Self {
        Self {
            intent: Vec2::ZERO,
            speed,
        }
    }
}
