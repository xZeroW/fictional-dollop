use bevy::prelude::*;
use std::time::Duration;

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

// Weapon
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Weapon {
    pub key: String,
    pub attack_timer: Timer,
}

impl Weapon {
    pub fn new(key: impl Into<String>) -> Self {
        let mut attack_timer = Timer::from_seconds(1.0, TimerMode::Once);
        attack_timer.set_elapsed(attack_timer.duration());

        Self {
            key: key.into(),
            attack_timer,
        }
    }

    pub fn set_attack_speed(&mut self, attack_speed: f32) {
        let cooldown = if attack_speed > 0.0 {
            Duration::from_secs_f32(1.0 / attack_speed)
        } else {
            Duration::from_secs_f32(1.0)
        };
        if self.attack_timer.duration() == cooldown {
            return;
        }

        self.attack_timer.set_duration(cooldown);
        self.attack_timer.set_elapsed(cooldown);
    }

    #[cfg_attr(not(all(feature = "dev", debug_assertions)), allow(dead_code))]
    pub fn equip(&mut self, key: impl Into<String>) {
        let key = key.into();
        if self.key == key {
            return;
        }

        self.key = key;
        self.attack_timer.set_elapsed(self.attack_timer.duration());
    }
}
