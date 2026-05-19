pub mod damage;
pub mod death;
pub mod bullet_collision;
pub mod player_collision;

pub struct ListenersPlugin;

use bevy::prelude::*;
use crate::messages::CollisionMessage;

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
