//! Weapon data loaded from assets.

use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use std::collections::HashMap;

use crate::{
    demo::weapon::AvailableWeapons,
    screens::Screen,
};

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct Weapons(pub HashMap<String, WeaponData>);

#[derive(Debug, Clone, serde::Deserialize)]
pub struct WeaponData {
    pub name: String,
    pub damage: i32,
    pub velocity: f32,
    pub cooldown: f32,
    pub scale: f32,
    pub bullet_sprite_index: usize,
    pub weapon_sprite_index: usize,
    pub fire_sound_key: String,
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(RonAssetPlugin::<Weapons>::new(&["weapon_data.ron"]));
    app.add_systems(OnEnter(Screen::Loading), load_weapon_data);
}

#[derive(Resource)]
pub struct WeaponsHandle(pub Handle<Weapons>);

fn load_weapon_data(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut available_weapons: ResMut<AvailableWeapons>,
) {
    let handle = server.load("data/weapon_data.ron");
    commands.insert_resource(WeaponsHandle(handle));

    available_weapons.weapons = vec!["dagger".to_string(), "sword".to_string()];
}
