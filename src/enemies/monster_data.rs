//! Enemy data loaded from .ron files.

use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use std::collections::HashMap;

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
    pub spawn_rate: f32,
    pub attack_speed: f32,
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(RonAssetPlugin::<Enemies>::new(&["enemies_data.ron"]));
}
