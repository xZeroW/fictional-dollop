use bevy::prelude::*;

// Bullet
const BULLET_LIFETIME: f32 = 2.0;

#[derive(Component, Debug, Clone, PartialEq, Default, Reflect)]
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

// Damage
#[derive(Component, Default)]
pub struct Damage {
    pub value: f32,
}

impl Damage {
    pub fn new(value: f32) -> Self {
        Damage { value }
    }
}

// AttackTimer
#[derive(Component)]
pub struct AttackTimer {
    pub timer: Timer,
}

impl AttackTimer {
    pub fn new(attack_speed: f32) -> Self {
        let cooldown = 1.0 / attack_speed.max(f32::EPSILON);
        let mut timer = Timer::from_seconds(cooldown, TimerMode::Once);
        timer.set_elapsed(timer.duration());
        Self { timer }
    }
}
