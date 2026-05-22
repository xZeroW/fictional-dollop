use bevy::prelude::*;

use crate::assets::EnemyAssets;
use crate::components::Enemy;
use crate::game::config;
use crate::game::level::LevelEntity;

use crate::enemies::{Enemies, EnemiesDataHandle, EnemySpawner};

pub fn spawn_enemies(
    mut commands: Commands,
    spawner: Option<ResMut<EnemySpawner>>,
    level_entity: Option<Res<LevelEntity>>,
    enemies_data_handle: Option<Res<EnemiesDataHandle>>,
    enemies_data_assets: Option<Res<Assets<Enemies>>>,
    current_enemies: Query<Entity, With<Enemy>>,
    enemy_assets: Res<EnemyAssets>,
) {
    let Some(level_entity) = level_entity else {
        return;
    };

    let Some(mut spawner) = spawner else {
        return;
    };

    let Some(enemies_data_handle) = enemies_data_handle else {
        return;
    };

    let Some(enemies_data_assets) = enemies_data_assets else {
        return;
    };

    let Some(enemies_data) = enemies_data_assets.get(enemies_data_handle.0.id()) else {
        return;
    };

    let num_enemies = current_enemies.iter().len();

    if num_enemies >= config::MAX_NUM_ENEMIES {
        return;
    }

    let spawn_count = (config::MAX_NUM_ENEMIES - num_enemies).min(config::SPAWN_RATE_PER_SECOND);

    // Build weighted table once
    if spawner.enemy_keys.is_empty() {
        for (key, data) in &enemies_data.0 {
            spawner.enemy_keys.push((key.clone(), data.spawn_rate));
            spawner.total_spawn_weight += data.spawn_rate;
        }
    }

    commands.entity(level_entity.0).with_children(|parent| {
        for _ in 0..spawn_count {
            let Some(enemy_key) = spawner.select_enemy_key() else {
                continue;
            };

            let Some(enemy_data) = enemies_data.0.get(&enemy_key) else {
                continue;
            };

            let Some((image, layout)) = enemy_assets.get(&enemy_data.sprite_key) else {
                continue;
            };

            let (min_x, max_x, min_y, max_y) = get_map_bounds();

            let x = min_x + rand::random::<f32>() * (max_x - min_x);
            let y = min_y + rand::random::<f32>() * (max_y - min_y);

            parent.spawn(enemy_data.bundle(
                &enemy_key,
                Vec3::new(x, y, 0.0),
                image.clone(),
                layout.clone(),
            ));
        }
    });
}

fn get_map_bounds() -> (f32, f32, f32, f32) {
    let half_width = (config::MAP_WIDTH_TILES as f32 * config::TILE_SIZE) / 2.0;
    let half_height = (config::MAP_HEIGHT_TILES as f32 * config::TILE_SIZE) / 2.0;
    let margin = config::MAP_MARGIN;

    let min_x = -half_width + margin;
    let max_x = half_width - margin;
    let min_y = -half_height + margin;
    let max_y = half_height - margin;

    (min_x, max_x, min_y, max_y)
}
