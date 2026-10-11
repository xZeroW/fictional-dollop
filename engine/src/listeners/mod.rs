pub mod bullet_collision;
pub mod damage;
pub mod death;
pub mod loot;
pub mod player_collision;

pub struct ListenersPlugin;

use crate::messages::{BulletHitEnemyMessage, CollisionMessage};
use bevy::prelude::*;

impl Plugin for ListenersPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Messages<CollisionMessage>>();
        app.init_resource::<Messages<BulletHitEnemyMessage>>();
        app.add_plugins((
            damage::DamageListenerPlugin,
            death::DeathListenerPlugin,
            loot::LootListenerPlugin,
            bullet_collision::BulletCollisionListenerPlugin,
            player_collision::PlayerCollisionListenerPlugin,
        ));
    }
}
