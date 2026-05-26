use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    components::{Enemy, Player},
    config,
    game::{
        spatial::{Collidable, KDTree2},
        weapon::Bullet,
    },
    messages::{CollisionKind, CollisionMessage},
    screens::Screen,
};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(KDTree2::default()).add_systems(
            Update,
            (
                update_enemy_kd_tree,
                check_player_enemy_collisions,
                check_bullet_enemy_collisions,
            )
                .in_set(AppSystems::Update)
                .in_set(PausableSystems)
                .run_if(in_state(Screen::Gameplay)),
        );
    }
}

fn update_enemy_kd_tree(
    mut tree: ResMut<KDTree2>,
    enemy_query: Query<(&Transform, Entity), With<Enemy>>,
) {
    let mut items = Vec::new();
    for (transform, entity) in enemy_query.iter() {
        items.push(Collidable {
            pos: [transform.translation.x, transform.translation.y],
            entity,
        });
    }
    tree.rebuild(items);
}

fn check_player_enemy_collisions(
    player_query: Query<(Entity, &Transform), With<Player>>,
    tree: Res<KDTree2>,
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

    if let Some((nearest_pos, enemy_entity)) = tree.nearest_neighbour(player_pos)
        && player_pos.distance(nearest_pos) <= config::COLLISION_RADIUS
    {
        writer.write(CollisionMessage {
            entity_a: player_entity,
            entity_b: enemy_entity,
            position: player_pos,
            kind: CollisionKind::DamagePlayer,
        });
    }
}

fn check_bullet_enemy_collisions(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &Bullet)>,
    tree: Res<KDTree2>,
    mut writer: MessageWriter<CollisionMessage>,
) {
    if bullet_query.is_empty() {
        return;
    }

    for (bullet_entity, bullet_transform, _) in bullet_query.iter() {
        let bullet_pos = bullet_transform.translation.truncate();

        if let Some((nearest_pos, enemy_entity)) = tree.nearest_neighbour(bullet_pos)
            && bullet_pos.distance(nearest_pos) <= config::COLLISION_RADIUS
        {
            writer.write(CollisionMessage {
                entity_a: bullet_entity,
                entity_b: enemy_entity,
                position: bullet_pos,
                kind: CollisionKind::DamageEnemy,
            });
            commands.entity(bullet_entity).despawn();
        }
    }
}
