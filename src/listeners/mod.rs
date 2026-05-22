pub mod bullet_collision;
pub mod damage;
pub mod death;
pub mod player_collision;

pub struct ListenersPlugin;

use crate::messages::CollisionMessage;
use bevy::prelude::*;

impl Plugin for ListenersPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Messages<CollisionMessage>>();
        app.add_plugins((
            damage::DamageListener,
            death::DeathListener,
            bullet_collision::BulletCollisionListener,
            player_collision::PlayerCollisionListener,
        ));
    }
}
