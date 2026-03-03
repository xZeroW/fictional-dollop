//! Spawn the main level.

use bevy::prelude::*;

use crate::{
    assets::{AudioAssets, CharacterAssets, WeaponAssets},
    audio::music,
    demo::{player::player, weapon::weapon},
    enemies::EnemySpawner,
    screens::Screen,
};

#[derive(Component)]
pub struct Level;

#[derive(Resource)]
pub struct LevelEntity(pub Entity);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_level);
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    player_assets: Res<CharacterAssets>,
    weapon_assets: Res<WeaponAssets>,
) {
    commands.insert_resource(EnemySpawner::default());

    let level = commands
        .spawn((
            Name::new("Level"),
            Level,
            Transform::default(),
            Visibility::default(),
            DespawnOnExit(Screen::Gameplay),
        ))
        .id();

    commands.insert_resource(LevelEntity(level));

    commands.entity(level).with_children(|parent| {
        parent
            .spawn(player(400.0, &player_assets, "dagger".to_string()))
            .with_children(|p| {
                p.spawn(weapon(&weapon_assets));
            });

        parent.spawn((
            Name::new("Gameplay Music"),
            music(audio_assets.background.clone()),
        ));
    });
}
