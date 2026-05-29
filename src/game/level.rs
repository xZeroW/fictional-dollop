//! Spawn the main level.

use bevy::prelude::*;

use crate::{
    assets::{AudioAssets, CharacterAssets},
    audio::music,
    components::Player,
    enemies::EnemySpawner,
    game::player::player,
    screens::Screen,
};

#[derive(Component)]
pub struct Level;

#[derive(Resource)]
pub struct LevelEntity(pub Entity);

pub(super) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Gameplay), spawn_level);
        app.add_systems(OnExit(Screen::Gameplay), remove_level_resources);
    }
}

/// A system that spawns the main level.
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

    let player_bundle = player(&player_assets, "dagger".to_string());
    let player_entity = commands.spawn(player_bundle).id();

    commands.entity(level).add_child(player_entity);

    commands.entity(level).with_children(|parent| {
        parent.spawn((
            Name::new("Gameplay Music"),
            music(audio_assets.background.clone()),
        ));
    });

    commands.entity(player_entity).insert(Player {
        weapon: "dagger".to_string(),
        attack_range: 200.0,
        last_shot_time: 0.0,
    });
}

fn remove_level_resources(mut commands: Commands) {
    commands.remove_resource::<EnemySpawner>();
    commands.remove_resource::<LevelEntity>();
}
