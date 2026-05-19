use bevy::prelude::*;

use crate::{
    components::{Damage, Enemy},
    messages::{ApplyDamageMessage, CollisionKind, CollisionMessage},
};

use crate::game::config;

pub struct PlayerCollisionListener;

impl Plugin for PlayerCollisionListener {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_enemy_player_collision);
    }
}

fn handle_enemy_player_collision(
    mut collision_reader: MessageReader<CollisionMessage>,
    enemy_query: Query<&Damage, With<Enemy>>,
    mut damage_writer: MessageWriter<ApplyDamageMessage>,
) {
    for collision in collision_reader.read() {
        if collision.kind != CollisionKind::DamagePlayer {
            continue;
        }

        let player_entity = collision.entity_a;
        let enemy_entity = collision.entity_b;

        let damage = enemy_query
            .get(enemy_entity)
            .map(|d| d.value)
            .unwrap_or(config::ENEMY_DAMAGE);

        damage_writer.write(ApplyDamageMessage {
            target: player_entity,
            damage,
        });
    }
}