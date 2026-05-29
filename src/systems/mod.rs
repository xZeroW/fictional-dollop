mod auto_attack;
mod bullet;
mod collision;
mod flip_sprite;
mod health;
mod wave;

use bevy::prelude::*;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            auto_attack::AutoAttackSystemsPlugin,
            bullet::BulletSystemsPlugin,
            collision::CollisionSystemsPlugin,
            health::HealthSystemsPlugin,
            wave::WaveSystemsPlugin,
        ));
        app.add_systems(
            Update,
            flip_sprite::flip_sprite.in_set(crate::PausableSystems),
        );
    }
}
