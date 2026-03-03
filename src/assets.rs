use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

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
    pub sprite: Handle<Image>,
    #[asset(key = "weapon.layout")]
    pub layout: Handle<TextureAtlasLayout>,
    #[asset(key = "weapon.fire_sound")]
    pub fire_sound: Handle<AudioSource>,
}

pub fn plugin(app: &mut App) {
    // Main asset loading that runs when entering the Loading screen.
    app.add_loading_state(
        LoadingState::new(Screen::Loading)
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                "data/characters.assets.ron",
            )
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>("data/audio.assets.ron")
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>("data/weapon.assets.ron")
            .load_collection::<CharacterAssets>()
            .load_collection::<AudioAssets>()
            .load_collection::<WeaponAssets>()
            .continue_to_state(Screen::Gameplay),
    );
}
