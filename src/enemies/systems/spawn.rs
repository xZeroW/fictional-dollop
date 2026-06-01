use bevy::prelude::*;

use crate::assets::EnemyAssets;
use crate::config::{GameConfig, MAP_HEIGHT_TILES, MAP_MARGIN, MAP_WIDTH_TILES, TILE_SIZE};
use crate::game::level::LevelEntity;
use crate::systems::MonsterProgression;

use crate::enemies::{Enemies, EnemiesDataHandle, EnemySpawner};

pub fn spawn_enemies(
    mut commands: Commands,
    mut spawner: ResMut<EnemySpawner>,
    level_entity: Res<LevelEntity>,
    enemies_data_handle: Res<EnemiesDataHandle>,
    enemies_data_assets: Res<Assets<Enemies>>,
    config: Res<GameConfig>,
    enemy_assets: Res<EnemyAssets>,
    progression: Res<MonsterProgression>,
) {
    let enemies_data = enemies_data_assets
        .get(enemies_data_handle.0.id())
        .expect("Enemy data asset should be loaded");

    if spawner.spawned_count >= config.max_num_enemies {
        return;
    }

    let spawn_count =
        (config.max_num_enemies - spawner.spawned_count).min(config.spawn_rate_per_second);

    if spawner.enemy_keys.is_empty() {
        spawner.init_weights(enemies_data);
    }

    commands.entity(level_entity.0).with_children(|parent| {
        for _ in 0..spawn_count {
            let Some(enemy_key) = spawner.select_enemy_key() else {
                continue;
            };

            let Some(enemy_data) = enemies_data.0.get(enemy_key) else {
                continue;
            };

            let Some((image, layout)) = enemy_assets.get(&enemy_data.asset_key) else {
                continue;
            };

            let (min_x, max_x, min_y, max_y) = get_map_bounds();

            let x = min_x + rand::random::<f32>() * (max_x - min_x);
            let y = min_y + rand::random::<f32>() * (max_y - min_y);
            let spawn_pos = Vec3::new(x, y, 0.0);

            parent.spawn(enemy_data.bundle(
                enemy_key,
                spawn_pos,
                image.clone(),
                layout.clone(),
                &progression,
            ));
            spawner.spawned_count += 1;
        }
    });
}

fn get_map_bounds() -> (f32, f32, f32, f32) {
    let half_width = (MAP_WIDTH_TILES as f32 * TILE_SIZE) / 2.0;
    let half_height = (MAP_HEIGHT_TILES as f32 * TILE_SIZE) / 2.0;
    let margin = MAP_MARGIN;

    let min_x = -half_width + margin;
    let max_x = half_width - margin;
    let min_y = -half_height + margin;
    let max_y = half_height - margin;

    (min_x, max_x, min_y, max_y)
}
