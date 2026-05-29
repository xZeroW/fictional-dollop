use bevy::prelude::*;

const BULLET_LIFETIME: f32 = 2.0;

#[derive(Component, Debug, Clone, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct Bullet {
    pub velocity: Vec2,
    pub damage: f32,
    pub lifetime: Timer,
}

impl Bullet {
    pub fn new(direction: Vec2, velocity: f32, damage: f32) -> Self {
        Self {
            velocity: direction.normalize() * velocity,
            damage,
            lifetime: Timer::from_seconds(BULLET_LIFETIME, TimerMode::Once),
        }
    }
}
