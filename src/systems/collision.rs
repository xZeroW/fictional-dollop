use bevy::prelude::*;

use super::enemy_spatial::EnemySpatialIndex;

use crate::{
    AppSystems, PausableSystems,
    components::{Bullet, Enemy, Player},
    config,
    messages::{BulletHitEnemyMessage, CollisionKind, CollisionMessage},
    screens::Screen,
};

pub(super) struct CollisionSystemsPlugin;

impl Plugin for CollisionSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (check_player_enemy_collisions, check_bullet_enemy_collisions)
                .in_set(AppSystems::SpatialQueries)
                .in_set(PausableSystems)
                .run_if(in_state(Screen::Gameplay)),
        );
    }
}

fn check_player_enemy_collisions(
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
    spatial_index: Res<EnemySpatialIndex>,
    mut writer: MessageWriter<CollisionMessage>,
) {
    if player_query.is_empty() {
        return;
    }

    let (player_entity, player_transform) = match player_query.single() {
        Ok(v) => v,
        Err(_) => return,
    };
    let player_pos = player_transform.translation.truncate();
    let contact_radius = config::PLAYER_ENEMY_CONTACT_RADIUS;
    let contact_radius_squared = contact_radius * contact_radius;

    for (enemy_entity, _) in spatial_index.enemies_within(player_pos, contact_radius) {
        let Ok(enemy_transform) = enemy_query.get(enemy_entity) else {
            continue;
        };

        let enemy_pos = enemy_transform.translation.truncate();
        let distance_squared = player_pos.distance_squared(enemy_pos);
        if distance_squared > contact_radius_squared {
            continue;
        }

        writer.write(CollisionMessage {
            entity_a: player_entity,
            entity_b: enemy_entity,
            kind: CollisionKind::DamagePlayer,
        });
    }
}

fn check_bullet_enemy_collisions(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &Bullet)>,
    spatial_index: Res<EnemySpatialIndex>,
    mut writer: MessageWriter<BulletHitEnemyMessage>,
) {
    if bullet_query.is_empty() {
        return;
    }

    for (bullet_entity, bullet_transform, bullet) in bullet_query.iter() {
        let bullet_pos = bullet_transform.translation.truncate();

        if let Some((enemy_entity, _)) =
            spatial_index.nearest_enemy(bullet_pos, config::BULLET_ENEMY_COLLISION_RADIUS)
        {
            writer.write(BulletHitEnemyMessage {
                enemy: enemy_entity,
                damage: bullet.damage,
            });
            commands.entity(bullet_entity).despawn();
        }
    }
}
