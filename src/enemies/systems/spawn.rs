use bevy::prelude::*;

use crate::assets::EnemyAssets;
use crate::components::{AttackCooldown, Damage, Enemy, Health, Movement, WanderState};
use crate::game::config;
use crate::game::level::LevelEntity;

use crate::enemies::{Enemies, EnemiesDataHandle, EnemySpawner, EnemyType};

pub fn spawn_enemies(
    mut commands: Commands,
    spawner: Option<ResMut<EnemySpawner>>,
    level_entity: Option<Res<LevelEntity>>,
    enemies_data_handle: Option<Res<EnemiesDataHandle>>,
    enemies_data: Option<Res<Assets<Enemies>>>,
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

    let Some(enemies_data) = enemies_data else {
        return;
    };

    let Some(enemies_data) = enemies_data.get(enemies_data_handle.0.id()) else {
        return;
    };

    let num_enemies = current_enemies.iter().len();
    if num_enemies >= config::MAX_NUM_ENEMIES {
        return;
    }

    let spawn_count = (config::MAX_NUM_ENEMIES - num_enemies).min(config::SPAWN_RATE_PER_SECOND);

    if spawner.enemy_keys.is_empty() {
        for (key, data) in enemies_data.0.iter() {
            spawner.enemy_keys.push((key.clone(), data.spawn_rate));
            spawner.total_spawn_weight += data.spawn_rate;
        }
    }

    for _ in 0..spawn_count {
        let Some(enemy_key) = spawner.select_enemy_key() else {
            continue;
        };

        let Some(enemy_data) = enemies_data.0.get(&enemy_key) else {
            continue;
        };

        let (image, layout) = match enemy_key.as_str() {
            "green" => (
                enemy_assets.green_sprite.clone(),
                enemy_assets.green_layout.clone(),
            ),
            "red" => (
                enemy_assets.red_sprite.clone(),
                enemy_assets.red_layout.clone(),
            ),
            _ => continue,
        };

        let (min_x, max_x, min_y, max_y) = get_map_bounds();
        let x = min_x + rand::random::<f32>() * (max_x - min_x);
        let y = min_y + rand::random::<f32>() * (max_y - min_y);

        let bundle = enemy_bundle(
            &enemy_key,
            enemy_data,
            Vec3::new(x, y, 0.0),
            image,
            layout,
        );

        commands.entity(level_entity.0).with_children(|parent| {
            parent.spawn(bundle);
        });
    }
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

fn enemy_bundle(
    key: &str,
    data: &crate::enemies::monster_data::EnemyData,
    position: Vec3,
    image: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
) -> impl Bundle {
    (
        Name::new(data.name.clone()),
        Enemy::new(key.to_string()),
        EnemyType::from_key(key),
        Health::new(data.health as f32),
        Movement::new(data.speed),
        WanderState::default(),
        Damage::new(data.damage as f32),
        AttackCooldown::new(data.attack_speed),
        Sprite::from_atlas_image(
            image,
            TextureAtlas {
                layout,
                index: data.sprite_index,
            },
        ),
        Transform::from_translation(position).with_scale(Vec2::splat(data.scale).extend(1.0)),
    )
}