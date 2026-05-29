mod animation;
mod auto_attack;
mod bullet;
mod collision;
mod enemy_spatial;
mod flip_sprite;
mod health;
mod movement;
mod wave;

use bevy::prelude::*;

pub(crate) use animation::PlayerAnimation;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            animation::AnimationSystemsPlugin,
            auto_attack::AutoAttackSystemsPlugin,
            bullet::BulletSystemsPlugin,
            collision::CollisionSystemsPlugin,
            enemy_spatial::EnemySpatialPlugin,
            health::HealthSystemsPlugin,
            movement::MovementSystemsPlugin,
            wave::WaveSystemsPlugin,
        ));
        app.add_systems(
            Update,
            flip_sprite::flip_sprite.in_set(crate::PausableSystems),
        );
    }
}
