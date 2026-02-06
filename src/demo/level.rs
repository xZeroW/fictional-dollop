//! Spawn the main level.

use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    audio::music,
    demo::player::{PlayerAssets, player},
    demo::weapon::{WeaponAssets, weapon},
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
        }
    }
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    player_assets: Res<PlayerAssets>,
    weapon_assets: Res<WeaponAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let level = commands
        .spawn((
            Name::new("Level"),
            Transform::default(),
            Visibility::default(),
            DespawnOnExit(Screen::Gameplay),
        ))
        .id();

    commands.entity(level).with_children(|parent| {
        parent.spawn(player(400.0, &player_assets, &mut texture_atlas_layouts)).with_children(|p| {
            p.spawn(weapon(&weapon_assets, &mut texture_atlas_layouts));
        });

        parent.spawn((
            Name::new("Gameplay Music"),
            music(level_assets.music.clone()),
        ));
    });
}
