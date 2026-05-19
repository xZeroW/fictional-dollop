use bevy::prelude::*;

use crate::{
    components::{Damage, Enemy, Health, Player},
    game::{spatial::{KDTree2, Collidable}, weapon::Bullet},
    messages::DamageMessage,
    screens::Screen,
    AppSystems, PausableSystems,
};

use super::config;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(KDTree2::default())
            .add_systems(
                Update,
                (
                    update_enemy_kd_tree,
                    handle_bullet_enemy_collision,
                    handle_enemy_player_collision,
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

fn handle_enemy_player_collision(
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<(Entity, &Transform, &Damage), With<Enemy>>,
    tree: Res<KDTree2>,
    mut health_query: Query<(&mut Health, &Transform), With<Player>>,
    mut writer: MessageWriter<DamageMessage>,
) {
    if player_query.is_empty() || health_query.is_empty() {
        return;
    }

    let (player_entity, player_transform) = match player_query.single() {
        Ok(v) => v,
        Err(_) => return,
    };
    let player_pos = player_transform.translation.truncate();

    if let Some((nearest_pos, entity)) = tree.nearest_neighbour(player_pos) {
        if player_pos.distance(nearest_pos) <= config::COLLISION_RADIUS {
            let damage = enemy_query
                .get(entity)
                .ok()
                .and_then(|(_, _, d)| Some(d.value))
                .unwrap_or(config::ENEMY_DAMAGE);

            if let Ok((mut health, _transform)) = health_query.single_mut() {
                health.take_damage(player_entity, damage, &mut writer);
            }
        }
    }
}

fn handle_bullet_enemy_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &Bullet), Without<Enemy>>,
    tree: Res<KDTree2>,
    mut enemy_query: Query<(Entity, &Transform, &mut Health), With<Enemy>>,
    mut writer: MessageWriter<DamageMessage>,
) {
    if bullet_query.is_empty() {
        return;
    }

    for (bullet_entity, bullet_transform, _) in bullet_query.iter() {
        let bullet_pos = bullet_transform.translation.truncate();

        if let Some((nearest_pos, entity)) = tree.nearest_neighbour(bullet_pos) {
            if bullet_pos.distance(nearest_pos) <= config::COLLISION_RADIUS {
                if let Ok((enemy_entity, _enemy_transform, mut health)) = enemy_query.get_mut(entity) {
                    commands.entity(bullet_entity).despawn();
                    health.take_damage(enemy_entity, config::BULLET_DAMAGE, &mut writer);
                }
            }
        }
    }
}
