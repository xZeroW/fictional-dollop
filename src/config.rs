//! Game configuration.

use bevy::prelude::*;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct GameConfig {
    pub max_num_enemies: usize,
    pub spawn_rate_per_second: usize,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            max_num_enemies: MAX_NUM_ENEMIES,
            spawn_rate_per_second: SPAWN_RATE_PER_SECOND,
        }
    }
}

pub const ORTHO_MIN_SCALE: f32 = 0.5;
pub const ORTHO_MAX_SCALE: f32 = 1.0;

pub const MAX_NUM_ENEMIES: usize = 50_000;
pub const SPAWN_RATE_PER_SECOND: usize = 2;
pub const ENEMY_SPAWN_INTERVAL: f32 = 0.5;
pub const WAVE_DURATION: f32 = 60.0;

pub const MAP_WIDTH_TILES: u32 = 50;
pub const MAP_HEIGHT_TILES: u32 = 30;
pub const TILE_SIZE: f32 = 64.0;
pub const MAP_MARGIN: f32 = 64.0;

pub const PLAYER_BODY_RADIUS: f32 = 24.0;
pub const ENEMY_BODY_RADIUS: f32 = 15.0;
pub const PLAYER_ENEMY_CONTACT_RADIUS: f32 = PLAYER_BODY_RADIUS + ENEMY_BODY_RADIUS;
pub const BULLET_ENEMY_COLLISION_RADIUS: f32 = 10.0;
pub const KD_TREE_REFRESH_RATE: f32 = 0.1;

pub(crate) fn map_bounds() -> (f32, f32, f32, f32) {
    let half_width = (MAP_WIDTH_TILES as f32 * TILE_SIZE) / 2.0;
    let half_height = (MAP_HEIGHT_TILES as f32 * TILE_SIZE) / 2.0;

    (
        -half_width + MAP_MARGIN,
        half_width - MAP_MARGIN,
        -half_height + MAP_MARGIN,
        half_height - MAP_MARGIN,
    )
}
