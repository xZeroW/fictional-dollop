use bevy::prelude::*;

use crate::messages::{ApplyDamageMessage, BulletHitEnemyMessage};

pub struct BulletCollisionListener;

impl Plugin for BulletCollisionListener {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_bullet_enemy_collision);
    }
}

fn handle_bullet_enemy_collision(
    mut hit_reader: MessageReader<BulletHitEnemyMessage>,
    mut damage_writer: MessageWriter<ApplyDamageMessage>,
) {
    for hit in hit_reader.read() {
        damage_writer.write(ApplyDamageMessage {
            target: hit.enemy,
            damage: hit.damage,
        });
    }
}
