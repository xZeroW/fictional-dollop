//! Spawn the main level.

use crate::Screen;
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

use crate::{
    domain::{audio::music, enemies::EnemySpawner, entities::player},
    infrastructure::assets::{AudioAssets, CharacterAssets},
};

#[derive(Component)]
pub struct Level;

#[derive(Resource)]
pub struct LevelEntity(pub Entity);

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
pub(super) struct LevelPlugin;

/// A system that spawns the main level.
#[auto_system(plugin = LevelPlugin, schedule = OnEnter(Screen::Gameplay))]
pub fn spawn_level(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    player_assets: Res<CharacterAssets>,
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

    let player_bundle = player(&player_assets, "dagger_01".to_string());
    let player_entity = commands.spawn(player_bundle).id();

    commands.entity(level).add_child(player_entity);

    commands.entity(level).with_children(|parent| {
        parent.spawn((
            Name::new("Gameplay Music"),
            music(audio_assets.background.clone()),
        ));
    });
}

#[auto_system(plugin = LevelPlugin, schedule = OnExit(Screen::Gameplay))]
fn remove_level_resources(mut commands: Commands) {
    commands.remove_resource::<EnemySpawner>();
    commands.remove_resource::<LevelEntity>();
}
