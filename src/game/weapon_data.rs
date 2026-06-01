//! Weapon data loaded from assets.

use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use std::collections::HashMap;

use crate::screens::Screen;

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct Weapons(pub HashMap<String, WeaponData>);

#[allow(dead_code)]
#[derive(Debug, Clone, serde::Deserialize)]
pub struct WeaponData {
    pub name: String,
    pub damage: f32,
    pub velocity: f32,
    pub attack_speed: f32,
    pub attack_range: f32,
    pub bullet_sprite_index: usize,
    pub weapon_sprite_index: usize,
    pub fire_sound_key: String,
}

pub(super) struct WeaponDataPlugin;

impl Plugin for WeaponDataPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<Weapons>::new(&["weapon_data.ron"]));
        app.add_systems(OnEnter(Screen::Loading), load_weapon_data);
    }
}

#[derive(Resource)]
pub struct WeaponsHandle(pub Handle<Weapons>);

fn load_weapon_data(mut commands: Commands, server: Res<AssetServer>) {
    let handle = server.load("data/weapon_data.ron");
    commands.insert_resource(WeaponsHandle(handle));
}
