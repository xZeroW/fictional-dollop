use bevy::image::{ImageLoaderSettings, ImageSampler};
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;

use crate::components::c_enemy::Enemy;
use crate::components::c_movement::Movement;
use crate::components::damage::Damage;
use crate::components::health::Health;
use crate::{AppSystems, PausableSystems, game::level::LevelEntity};

mod monster_data;

pub use monster_data::{Enemies, EnemyAssets};

#[derive(Resource)]
pub struct EnemySpawner {
    pub spawned_count: u32,
    pub max_enemies: u32,
    pub enemy_keys: Vec<String>,
}

impl Default for EnemySpawner {
    fn default() -> Self {
        Self {
            spawned_count: 0,
            max_enemies: 50000,
            enemy_keys: vec![],
        }
    }
}

#[derive(Resource)]
pub struct EnemyAssetsHandle(pub Handle<EnemyAssets>);

#[derive(Resource)]
pub struct EnemiesDataHandle(pub Handle<Enemies>);

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(monster_data::plugin);
    app.add_systems(OnEnter(crate::screens::Screen::Loading), load_enemy_data);
    app.add_systems(
        Update,
        spawn_enemies
            .in_set(PausableSystems)
            .in_set(AppSystems::Update)
            .run_if(in_state(crate::screens::Screen::Gameplay))
            .run_if(on_timer(Duration::from_secs_f32(1.0))),
    );
}

fn load_enemy_data(mut commands: Commands, server: Res<AssetServer>) {
    let enemy_assets_handle = server.load("data/enemies.assets.ron");
    let enemies_data_handle = server.load("data/enemies_data.ron");

    commands.insert_resource(EnemyAssetsHandle(enemy_assets_handle));
    commands.insert_resource(EnemiesDataHandle(enemies_data_handle));
}

fn spawn_enemies(
    mut commands: Commands,
    spawner: Option<ResMut<EnemySpawner>>,
    level_entity: Option<Res<LevelEntity>>,
    enemy_assets_handle: Option<Res<EnemyAssetsHandle>>,
    enemies_data_handle: Option<Res<EnemiesDataHandle>>,
    enemy_assets: Option<Res<Assets<EnemyAssets>>>,
    enemies_data: Option<Res<Assets<Enemies>>>,
    player_query: Query<&GlobalTransform, With<crate::game::player::Player>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
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

    let Some(enemy_assets_handle) = enemy_assets_handle else {
        return;
    };

    let Some(enemy_assets) = enemy_assets else {
        return;
    };

    let Some(enemies_data) = enemies_data.get(enemies_data_handle.0.id()) else {
        return;
    };

    if spawner.enemy_keys.is_empty() {
        spawner.enemy_keys = enemies_data.0.keys().cloned().collect();
    }

    if spawner.spawned_count >= spawner.max_enemies {
        return;
    }

    let player_pos = match player_query.single() {
        Ok(gt) => gt.translation().truncate(),
        Err(_) => return,
    };

    let random_index = (spawner.spawned_count as usize + 1) % spawner.enemy_keys.len();
    let enemy_key = &spawner.enemy_keys[random_index];

    let enemy_data = match enemies_data.0.get(enemy_key) {
        Some(data) => data,
        None => return,
    };

    let Some(enemy_assets) = enemy_assets.get(enemy_assets_handle.0.id()) else {
        return;
    };

    let enemy_assets = match enemy_assets.0.get(enemy_key) {
        Some(asset) => asset,
        None => return,
    };

    let image = asset_server.load_with_settings(
        &enemy_assets.sprite_path,
        |settings: &mut ImageLoaderSettings| {
            settings.sampler = ImageSampler::nearest();
        },
    );
    let layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(
            enemy_assets.layout.tile_size_x,
            enemy_assets.layout.tile_size_y,
        ),
        enemy_assets.layout.columns,
        enemy_assets.layout.rows,
        None,
        None,
    ));

    let angle = spawner.spawned_count as f32 * std::f32::consts::TAU / 5.0;
    let distance = 150.0;
    let offset = Vec2::new(angle.cos() * distance, angle.sin() * distance);
    let spawn_pos = player_pos + offset;

    commands.entity(level_entity.0).with_children(|parent| {
        parent.spawn((
            Name::new(enemy_data.name.clone()),
            Enemy::new(enemy_key.clone()),
            Health::new(enemy_data.health as f32),
            Movement::new(enemy_data.speed),
            Damage::new(enemy_data.damage as f32),
            Sprite::from_atlas_image(
                image,
                TextureAtlas {
                    layout,
                    index: enemy_data.sprite_index,
                },
            ),
            Transform::from_translation(spawn_pos.extend(0.0))
                .with_scale(Vec2::splat(enemy_data.scale).extend(1.0)),
        ));
    });

    spawner.spawned_count += 1;
}
