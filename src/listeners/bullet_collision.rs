use bevy::prelude::*;

use crate::{
    components::Enemy,
    game::weapon::Bullet,
    messages::{ApplyDamageMessage, CollisionMessage},
};

use crate::game::config;

pub struct BulletCollisionListener;

impl Plugin for BulletCollisionListener {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_bullet_enemy_collision);
    }
}

fn handle_bullet_enemy_collision(
    mut commands: Commands,
    mut collision_reader: MessageReader<CollisionMessage>,
    bullet_query: Query<(), With<Bullet>>,
    enemy_query: Query<(), With<Enemy>>,
    mut damage_writer: MessageWriter<ApplyDamageMessage>,
) {
    let mut processed_bullets = Vec::new();

    for collision in collision_reader.read() {
        let bullet_entity = collision.entity_a;
        let enemy_entity = collision.entity_b;

        if processed_bullets.contains(&bullet_entity) {
            continue;
        }

        let is_bullet = bullet_query.get(bullet_entity).is_ok();
        let is_enemy = enemy_query.get(enemy_entity).is_ok();

        if is_bullet && is_enemy {
            processed_bullets.push(bullet_entity);
            if let Ok(mut e) = commands.get_entity(bullet_entity) {
                e.despawn();
            }
            damage_writer.write(ApplyDamageMessage {
                target: enemy_entity,
                damage: config::BULLET_DAMAGE,
            });
        }
    }
}