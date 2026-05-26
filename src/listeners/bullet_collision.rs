use bevy::prelude::*;

use crate::messages::{ApplyDamageMessage, CollisionKind, CollisionMessage};

use crate::config;

pub struct BulletCollisionListener;

impl Plugin for BulletCollisionListener {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_bullet_enemy_collision);
    }
}

fn handle_bullet_enemy_collision(
    mut collision_reader: MessageReader<CollisionMessage>,
    mut damage_writer: MessageWriter<ApplyDamageMessage>,
) {
    for collision in collision_reader.read() {
        if collision.kind != CollisionKind::DamageEnemy {
            continue;
        }

        let enemy_entity = collision.entity_b;

        damage_writer.write(ApplyDamageMessage {
            target: enemy_entity,
            damage: config::BULLET_DAMAGE,
        });
    }
}
