//! Enemy visual data loaded from .ron files.

use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};
use bevy_auto_plugin::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use std::collections::HashMap;

use crate::infrastructure::assets::EnemyAssets;

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct EnemyVisuals(pub HashMap<String, EnemyVisualData>);

#[derive(Debug, Clone, serde::Deserialize)]
pub struct EnemyVisualData {
    pub sprite_path: String,
    pub tile_size_x: u32,
    pub tile_size_y: u32,
    pub columns: usize,
    pub rows: usize,
}

#[derive(Resource)]
pub struct EnemyVisualsHandle(pub Handle<EnemyVisuals>);

#[derive(AutoPlugin)]
pub(super) struct EnemyAssetsPlugin;

impl Plugin for EnemyAssetsPlugin {
    #[auto_plugin]
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<EnemyVisuals>::new(&["enemies.assets.ron"]));
    }
}

#[auto_system(
    plugin = EnemyAssetsPlugin,
    schedule = OnEnter(crate::Screen::Gameplay)
)]
pub(super) fn load_visual_cache(
    mut enemy_assets: ResMut<EnemyAssets>,
    visuals_handle: Option<Res<EnemyVisualsHandle>>,
    visuals_assets: Res<Assets<EnemyVisuals>>,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let Some(visuals_handle) = visuals_handle else {
        return;
    };

    let Some(visuals) = visuals_assets.get(visuals_handle.0.id()) else {
        return;
    };

    enemy_assets.sprites.clear();
    enemy_assets.layouts.clear();

    for (key, visual) in &visuals.0 {
        let image = asset_server
            .load_builder()
            .with_settings(|settings: &mut ImageLoaderSettings| {
                settings.sampler = ImageSampler::nearest();
            })
            .load(&visual.sprite_path);
        let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::new(visual.tile_size_x, visual.tile_size_y),
            visual.columns.try_into().unwrap(),
            visual.rows.try_into().unwrap(),
            None,
            None,
        ));

        enemy_assets.sprites.insert(key.clone(), image);
        enemy_assets.layouts.insert(key.clone(), layout);
    }
}
