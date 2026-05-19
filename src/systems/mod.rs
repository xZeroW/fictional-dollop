mod flip_sprite;
mod health;

use bevy::prelude::*;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(health::HealthSystemsPlugin);
        app.add_systems(
            Update,
            flip_sprite::flip_sprite.in_set(crate::PausableSystems),
        );
    }
}
