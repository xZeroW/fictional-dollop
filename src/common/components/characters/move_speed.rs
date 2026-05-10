use bevy::{prelude::Component, reflect::Reflect};

#[derive(Component, Debug, Clone, Copy, Reflect)]
pub struct MoveSpeed(pub u32);

impl Default for MoveSpeed {
    fn default() -> Self {
        MoveSpeed(75)
    }
}
