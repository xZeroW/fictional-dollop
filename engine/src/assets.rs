use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use std::collections::HashMap;

use crate::screens::Screen;

#[derive(AssetCollection, Resource)]
pub struct CharacterAssets {
    #[asset(key = "player.layout")]
    pub layout: Handle<TextureAtlasLayout>,
    #[asset(key = "player.sprite")]
    pub sprite: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(key = "sounds.background")]
    pub background: Handle<AudioSource>,
    #[asset(key = "sounds.steps", collection(typed))]
    pub steps_sound: Vec<Handle<AudioSource>>,
}

#[derive(AssetCollection, Resource)]
pub struct WeaponAssets {
    #[asset(key = "weapon.sprite")]
    #[allow(dead_code)]
    pub sprite: Handle<Image>,
    #[asset(key = "weapon.layout")]
    #[allow(dead_code)]
    pub layout: Handle<TextureAtlasLayout>,
    #[asset(key = "weapon.bullet_sprite")]
    pub bullet_sprite: Handle<Image>,
    #[asset(key = "weapon.bullet_layout")]
    pub bullet_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "weapon.fire_sound")]
    #[allow(dead_code)]
    pub fire_sound: Handle<AudioSource>,
}

#[derive(Resource, Default)]
pub struct EnemyAssets {
    pub sprites: HashMap<String, Handle<Image>>,
    pub layouts: HashMap<String, Handle<TextureAtlasLayout>>,
}

pub(super) struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyAssets>();
        app.add_loading_state(
            LoadingState::new(Screen::Loading)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                    "data/characters.assets.ron",
                )
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>("data/audio.assets.ron")
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                    "data/weapon.assets.ron",
                )
                .load_collection::<CharacterAssets>()
                .load_collection::<AudioAssets>()
                .load_collection::<WeaponAssets>()
                .continue_to_state(Screen::Gameplay),
        );
    }
}

impl EnemyAssets {
    pub fn get(&self, key: &str) -> Option<(&Handle<Image>, &Handle<TextureAtlasLayout>)> {
        let sprite = self.sprites.get(key)?;
        let layout = self.layouts.get(key)?;
        Some((sprite, layout))
    }
}
