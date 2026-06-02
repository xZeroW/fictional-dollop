use bevy::prelude::*;
use kd_tree::{KdPoint, KdTree};
use typenum::U2;

use crate::{
    AppSystems, PausableSystems, components::Enemy, config::KD_TREE_REFRESH_RATE, screens::Screen,
};

#[derive(Clone)]
struct EnemySpatialEntry {
    pos: [f32; 2],
    entity: Entity,
}

impl KdPoint for EnemySpatialEntry {
    type Scalar = f32;
    type Dim = U2;

    fn at(&self, k: usize) -> f32 {
        self.pos[k]
    }
}

#[derive(Resource)]
pub(crate) struct EnemySpatialIndex {
    tree: KdTree<EnemySpatialEntry>,
    rebuild_timer: Timer,
    initialized: bool,
}

impl Default for EnemySpatialIndex {
    fn default() -> Self {
        Self {
            tree: KdTree::build_by_ordered_float(vec![]),
            rebuild_timer: Timer::from_seconds(KD_TREE_REFRESH_RATE, TimerMode::Repeating),
            initialized: false,
        }
    }
}

impl EnemySpatialIndex {
    fn rebuild(&mut self, items: Vec<EnemySpatialEntry>) {
        self.tree = KdTree::build_by_ordered_float(items);
        self.initialized = true;
    }

    pub(crate) fn nearest_enemy(&self, loc: Vec2, max_distance: f32) -> Option<(Entity, Vec2)> {
        if self.tree.is_empty() {
            return None;
        }

        let key = [loc.x, loc.y];
        let found = self.tree.nearest(&key)?;
        let item = found.item;
        let pos = Vec2::new(item.pos[0], item.pos[1]);

        if loc.distance_squared(pos) <= max_distance * max_distance {
            Some((item.entity, pos))
        } else {
            None
        }
    }

    pub(crate) fn enemies_within(&self, loc: Vec2, radius: f32) -> Vec<(Entity, Vec2)> {
        if self.tree.is_empty() {
            return vec![];
        }

        let key = [loc.x, loc.y];
        let radius_squared = radius * radius;
        self.tree
            .within_radius(&key, radius)
            .into_iter()
            .filter_map(|entry| {
                let pos = Vec2::new(entry.pos[0], entry.pos[1]);
                (loc.distance_squared(pos) <= radius_squared).then_some((entry.entity, pos))
            })
            .collect()
    }
}

pub(super) struct EnemySpatialPlugin;

impl Plugin for EnemySpatialPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpatialIndex>();
        app.add_systems(
            Update,
            rebuild_enemy_spatial_index
                .in_set(AppSystems::SpatialIndex)
                .in_set(PausableSystems)
                .run_if(in_state(Screen::Gameplay)),
        );
    }
}

fn rebuild_enemy_spatial_index(
    time: Res<Time>,
    mut index: ResMut<EnemySpatialIndex>,
    enemy_query: Query<(&Transform, Entity), With<Enemy>>,
) {
    index.rebuild_timer.tick(time.delta());

    if index.initialized && !index.rebuild_timer.just_finished() {
        return;
    }

    let mut items = Vec::with_capacity(enemy_query.iter().len());
    for (transform, entity) in enemy_query.iter() {
        let pos = transform.translation.truncate();
        items.push(EnemySpatialEntry {
            pos: [pos.x, pos.y],
            entity,
        });
    }

    index.rebuild(items);
}
