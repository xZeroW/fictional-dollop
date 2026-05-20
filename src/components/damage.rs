use bevy::prelude::{Component, Timer, TimerMode};

#[derive(Component, Default)]
pub struct Damage {
    pub value: f32,
}

impl Damage {
    pub fn new(value: f32) -> Self {
        Damage { value }
    }
}

#[derive(Component)]
pub struct AttackCooldown {
    pub timer: Timer,
}

impl AttackCooldown {
    pub fn new(attack_speed: f32) -> Self {
        let cooldown = if attack_speed > 0.0 {
            1.0 / attack_speed
        } else {
            1.0
        };
        AttackCooldown {
            timer: Timer::from_seconds(cooldown, TimerMode::Repeating),
        }
    }
}
