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

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
enum AssetStates {
    #[default]
    Loading,
    Done,
}

pub fn plugin(app: &mut App) {
    app.init_state::<AssetStates>();
    // Main asset loading that runs when entering the Loading screen.
    app.add_loading_state(
        LoadingState::new(Screen::Loading)
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                "rons/characters.assets.ron",
            )
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>("rons/audio.assets.ron")
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>("rons/weapon.assets.ron")
            .load_collection::<CharacterAssets>()
            .load_collection::<AudioAssets>()
            .continue_to_state(Screen::Gameplay),
    );
}
