use bevy::prelude::*;

use crate::{
    components::{AttackCooldown, Damage, Enemy},
    messages::{ApplyDamageMessage, CollisionKind, CollisionMessage},
};

pub struct PlayerCollisionListener;

impl Plugin for PlayerCollisionListener {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_enemy_player_collision);
    }
}

fn handle_enemy_player_collision(
    mut collision_reader: MessageReader<CollisionMessage>,
    enemy_damage_query: Query<&Damage, With<Enemy>>,
    enemy_cooldown_query: Query<&AttackCooldown, With<Enemy>>,
    mut damage_writer: MessageWriter<ApplyDamageMessage>,
) {
    for collision in collision_reader.read() {
        if collision.kind != CollisionKind::DamagePlayer {
            continue;
        }

        let player_entity = collision.entity_a;
        let enemy_entity = collision.entity_b;

        let Ok(damage) = enemy_damage_query.get(enemy_entity) else {
            continue;
        };

        let Ok(cooldown) = enemy_cooldown_query.get(enemy_entity) else {
            continue;
        };

        if !cooldown.timer.just_finished() {
            continue;
        }

        damage_writer.write(ApplyDamageMessage {
            target: player_entity,
            damage: damage.value,
        });
    }
}
