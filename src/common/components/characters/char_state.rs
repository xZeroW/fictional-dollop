use bevy::{prelude::Component, reflect::Reflect};

#[derive(Component, Default, Debug, Clone, Copy, Reflect)]
pub enum State {
    #[default]
    Idle,
    Moving,
}
