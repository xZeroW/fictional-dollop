use bevy::prelude::*;
use kd_tree::{KdPoint, KdTree};
use typenum::U2;

#[derive(Clone)]
pub struct Collidable {
    pub pos: [f32; 2],
    pub entity: Entity,
}

impl KdPoint for Collidable {
    type Scalar = f32;
    type Dim = U2;

    fn at(&self, k: usize) -> f32 {
        self.pos[k]
    }
}

#[derive(Resource)]
pub struct KDTree2 {
    pub tree: KdTree<Collidable>,
}

impl Default for KDTree2 {
    fn default() -> Self {
        Self {
            tree: KdTree::build_by_ordered_float(vec![]),
        }
    }
}

impl KDTree2 {
    pub fn rebuild(&mut self, items: Vec<Collidable>) {
        self.tree = KdTree::build_by_ordered_float(items);
    }

    pub fn nearest_neighbour(&self, loc: Vec2) -> Option<(Vec2, Entity)> {
        if self.tree.len() == 0 {
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

    pub fn within_distance(&self, loc: Vec2, distance: f32) -> Vec<(Vec2, Entity)> {
        if self.tree.len() == 0 {
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