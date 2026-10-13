mod auto_attack;
mod bullet;
mod bullet_collision;
mod collision;
mod components;
mod damage_feedback;
mod death;
mod health;
mod messages;
mod player_collision;

use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

pub use components::{AttackTimer, Bullet, Damage, Weapon};
pub(crate) use messages::EntityDiedMessage;

#[derive(AutoPlugin)]
pub(crate) struct CombatPlugin;

impl Plugin for CombatPlugin {
    #[auto_plugin]
    fn build(&self, app: &mut App) {
        app.add_plugins((
            auto_attack::AutoAttackSystemsPlugin,
            bullet::BulletSystemsPlugin,
            collision::CollisionSystemsPlugin,
            health::HealthSystemsPlugin,
            bullet_collision::BulletCollisionListenerPlugin,
            player_collision::PlayerCollisionListenerPlugin,
            damage_feedback::DamageListenerPlugin,
            death::DeathListenerPlugin,
        ));
    }
}
