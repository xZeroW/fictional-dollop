use bevy::prelude::*;

use crate::{
    components::{Damage, Enemy, Player},
    messages::{ApplyDamageMessage, CollisionMessage},
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
    player_query: Query<(), With<Player>>,
    enemy_query: Query<&Damage, With<Enemy>>,
    mut damage_writer: MessageWriter<ApplyDamageMessage>,
) {
    for collision in collision_reader.read() {
        let entity_a = collision.entity_a;
        let entity_b = collision.entity_b;

        let a_is_player = player_query.get(entity_a).is_ok();
        let b_is_player = player_query.get(entity_b).is_ok();
        let a_is_enemy = enemy_query.get(entity_a).is_ok();
        let b_is_enemy = enemy_query.get(entity_b).is_ok();

        let (player_entity, enemy_entity) = match (a_is_player, b_is_player, a_is_enemy, b_is_enemy) {
            (true, false, false, true) => (entity_a, entity_b),
            (false, true, true, false) => (entity_b, entity_a),
            _ => continue,
        };

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