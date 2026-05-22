use bevy::prelude::*;
use bevy_gauge::prelude::*;

#[derive(Component, AttributeComponent, Reflect, Debug)]
pub struct Health {
    #[read("Health")]
    pub max: f32,
    #[write("Health.current")]
    #[init_from("Health")]
    pub current: f32,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            max: 100.0,
            current: 100.0,
        }
    }
}

impl Health {
    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }

    pub fn apply_damage(&mut self, damage: f32) {
        if self.current <= 0.0 {
            return;
        }

        self.current -= damage;
        if self.current < 0.0 {
            self.current = 0.0;
        }
    }

    pub fn heal(&mut self, amount: f32) {
        self.current += amount;
        if self.current > self.max {
            self.current = self.max;
        }
    }
}
