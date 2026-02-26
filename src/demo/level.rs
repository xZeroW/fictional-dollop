//! Spawn the main level.

use bevy::prelude::*;

use crate::{
    audio::music,
    demo::{
        player::player,
        weapon::{WeaponAssets, weapon}
    },
    ron_asset::{AudioAssets, CharacterAssets},
    screens::Screen
};

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    player_assets: Res<CharacterAssets>,
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
        parent.spawn(player(400.0, &player_assets)).with_children(|p| {
            p.spawn(weapon(&weapon_assets, &mut texture_atlas_layouts));
        });

        parent.spawn((
            Name::new("Gameplay Music"),
            music(audio_assets.background.clone()),
        ));
    });
}
