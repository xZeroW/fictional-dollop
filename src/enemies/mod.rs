use bevy::image::{ImageLoaderSettings, ImageSampler};
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;

use crate::components::{Damage, Enemy, Health, Movement};
use crate::game::config;
use crate::{AppSystems, PausableSystems, game::level::LevelEntity};

mod monster_data;

pub use monster_data::{Enemies, EnemyAssets};

#[derive(Component)]
pub enum EnemyType {
    Slime,
    Goblin,
    Orc,
}

impl EnemyType {
    pub fn from_key(key: &str) -> Self {
        match key.to_lowercase().as_str() {
            "slime" => EnemyType::Slime,
            "goblin" => EnemyType::Goblin,
            "orc" => EnemyType::Orc,
            _ => EnemyType::Slime,
        }
    }
}

#[derive(Resource)]
pub struct EnemySpawner {
    pub spawned_count: u32,
    pub max_enemies: u32,
    pub enemy_keys: Vec<(String, f32)>,
    total_spawn_weight: f32,
}

impl Default for EnemySpawner {
    fn default() -> Self {
        Self {
            spawned_count: 0,
            max_enemies: config::MAX_NUM_ENEMIES as u32,
            enemy_keys: vec![],
            total_spawn_weight: 0.0,
        }
    }
}

impl EnemySpawner {
    fn select_enemy_key(&self) -> Option<String> {
        if self.enemy_keys.is_empty() || self.total_spawn_weight <= 0.0 {
            return None;
        }

        let pick = rand::random::<f32>() * self.total_spawn_weight;
        let mut accum = 0.0;

        for (key, weight) in &self.enemy_keys {
            accum += *weight;
            if pick <= accum {
                return Some(key.clone());
            }
        }

        self.enemy_keys.first().map(|(k, _)| k.clone())
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
            .run_if(on_timer(Duration::from_secs_f32(
                config::ENEMY_SPAWN_INTERVAL,
            ))),
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
    enemies_data_handle: Option<Res<EnemiesDataHandle>>,
    enemies_data: Option<Res<Assets<Enemies>>>,
    current_enemies: Query<Entity, With<Enemy>>,
    enemy_assets_handle: Option<Res<EnemyAssetsHandle>>,
    enemy_assets: Option<Res<Assets<EnemyAssets>>>,
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

    let Some(enemy_assets_handle) = enemy_assets_handle else {
        return;
    };

    let Some(enemy_assets) = enemy_assets else {
        return;
    };

    let Some(enemy_assets) = enemy_assets.get(enemy_assets_handle.0.id()) else {
        return;
    };

    let (min_x, max_x, min_y, max_y) = get_map_bounds();

    for _ in 0..spawn_count {
        let Some(enemy_key) = spawner.select_enemy_key() else {
            continue;
        };

        let enemy_data = match enemies_data.0.get(&enemy_key) {
            Some(data) => data,
            None => continue,
        };

        let enemy_asset = match enemy_assets.0.get(&enemy_key) {
            Some(asset) => asset,
            None => continue,
        };

        let x = min_x + rand::random::<f32>() * (max_x - min_x);
        let y = min_y + rand::random::<f32>() * (max_y - min_y);

        let image = asset_server.load_with_settings(
            &enemy_asset.sprite_path,
            |settings: &mut ImageLoaderSettings| {
                settings.sampler = ImageSampler::nearest();
            },
        );
        let layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::new(
                enemy_asset.layout.tile_size_x,
                enemy_asset.layout.tile_size_y,
            ),
            enemy_asset.layout.columns,
            enemy_asset.layout.rows,
            None,
            None,
        ));

        commands.entity(level_entity.0).with_children(|parent| {
            parent.spawn(enemy_bundle(
                &enemy_key,
                enemy_data,
                Vec3::new(x, y, 0.0),
                image,
                layout,
            ));
        });

        spawner.spawned_count += 1;
    }
}

fn enemy_bundle(
    key: &str,
    data: &monster_data::EnemyData,
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
        Damage::new(data.damage as f32),
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
