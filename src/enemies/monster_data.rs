//! Enemy assets and data loaded from .ron files.

use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use std::collections::HashMap;

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct EnemyAssets(pub HashMap<String, EnemyAssetData>);

#[derive(Debug, Clone, serde::Deserialize)]
pub struct EnemyAssetData {
    pub sprite_path: String,
    pub layout: EnemyLayoutData,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct EnemyLayoutData {
    pub tile_size_x: u32,
    pub tile_size_y: u32,
    pub columns: u32,
    pub rows: u32,
}

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct Enemies(pub HashMap<String, EnemyData>);

#[derive(Debug, Clone, serde::Deserialize)]
pub struct EnemyData {
    pub name: String,
    pub sprite_key: String,
    pub layout_key: String,
    pub health: i32,
    pub damage: i32,
    pub speed: f32,
    pub scale: f32,
    pub sprite_index: usize,
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        RonAssetPlugin::<EnemyAssets>::new(&["enemies.assets.ron"]),
        RonAssetPlugin::<Enemies>::new(&["enemies_data.ron"]),
    ));
}
