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

pub use components::{AttackTimer, Bullet, Damage, Weapon};
pub(crate) use messages::EntityDiedMessage;

pub(crate) struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Messages<messages::CollisionMessage>>();
        app.init_resource::<Messages<messages::BulletHitEnemyMessage>>();
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
