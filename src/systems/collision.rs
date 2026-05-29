use bevy::prelude::*;
use kd_tree::{KdPoint, KdTree};
use typenum::U2;

use crate::{
    AppSystems, PausableSystems,
    components::{Bullet, Enemy, Player},
    config,
    messages::{CollisionKind, CollisionMessage},
    screens::Screen,
};

#[derive(Clone)]
struct Collidable {
    pos: [f32; 2],
    entity: Entity,
}

impl KdPoint for Collidable {
    type Scalar = f32;
    type Dim = U2;

    fn at(&self, k: usize) -> f32 {
        self.pos[k]
    }
}

#[derive(Resource)]
struct KDTree2 {
    tree: KdTree<Collidable>,
}

impl Default for KDTree2 {
    fn default() -> Self {
        Self {
            tree: KdTree::build_by_ordered_float(vec![]),
        }
    }
}

impl KDTree2 {
    fn rebuild(&mut self, items: Vec<Collidable>) {
        self.tree = KdTree::build_by_ordered_float(items);
    }

    fn nearest_neighbour(&self, loc: Vec2) -> Option<(Vec2, Entity)> {
        if self.tree.is_empty() {
            return None;
        }
        let key = [loc.x, loc.y];
        if let Some(found) = self.tree.nearest(&key) {
            let item = found.item;
            let pos = Vec2::new(item.pos[0], item.pos[1]);
            return Some((pos, item.entity));
        }
        None
    }

    #[allow(dead_code)]
    fn within_distance(&self, loc: Vec2, distance: f32) -> Vec<(Vec2, Entity)> {
        if self.tree.is_empty() {
            return vec![];
        }
        let key = [loc.x, loc.y];
        let found = self.tree.within_radius(&key, distance);
        found
            .into_iter()
            .map(|c| (Vec2::new(c.pos[0], c.pos[1]), c.entity))
            .collect()
    }
}

pub(super) struct CollisionSystemsPlugin;

impl Plugin for CollisionSystemsPlugin {
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
