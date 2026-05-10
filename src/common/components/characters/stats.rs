use bevy::ecs::component::Component;
use serde::Deserialize;

#[derive(Deserialize, Component)]
pub struct Stats {
    pub damage: f32,
    pub health: f32,
    pub attack_speed: f32,
    pub move_speed: f32,
}

impl Default for Stats {
    fn default() -> Self {
        Stats {
            damage: 15.0,
            health: 100.0,
            attack_speed: 1.0,
            move_speed: 75.0,
        }
    }
}
