use bevy::prelude::*;
use std::time::Duration;

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
