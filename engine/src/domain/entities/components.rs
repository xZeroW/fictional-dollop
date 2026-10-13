use bevy::prelude::*;
use bevy_gauge::prelude::*;

use crate::domain::combat::Damage;

// Player
#[derive(Component, Debug, Clone, PartialEq, Default, Reflect)]
#[reflect(Component)]
#[require(Attributes, Health, Movement)]
pub struct Player;

// Enemy
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(Health, Movement, Damage, Behavior, WanderState)]
pub struct Enemy {
    pub enemy_type: String,
}

impl Enemy {
    pub fn new(enemy_type: String) -> Self {
        Enemy { enemy_type }
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub enum Behavior {
    #[default]
    Wandering,
    FollowAndAttack,
    Coward,
}

#[derive(Component)]
pub struct WanderState {
    pub direction: Vec2,
    pub timer: Timer,
}

impl Default for WanderState {
    fn default() -> Self {
        Self {
            direction: Vec2::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5)
                .normalize_or_zero(),
            timer: Timer::from_seconds(3.0, TimerMode::Repeating),
        }
    }
}

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

// Health
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

    pub fn apply_damage(&mut self, damage: f32) -> f32 {
        if self.current <= 0.0 || damage <= 0.0 {
            return 0.0;
        }

        let applied = damage.min(self.current);
        self.current -= applied;
        applied
    }
}
