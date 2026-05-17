//! Spawn the main level.

use bevy::prelude::*;

use crate::{
    assets::{AudioAssets, CharacterAssets, WeaponAssets},
    audio::music,
    components::Player,
    enemies::EnemySpawner,
    game::{
        player::player,
        weapon::weapon,
        weapon_data::{Weapons, WeaponsHandle},
    },
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
    weapons_handle: Res<WeaponsHandle>,
    weapons_assets: Res<Assets<Weapons>>,
) {
    commands.insert_resource(EnemySpawner::default());

    let weapon_data = weapons_assets
        .get(&weapons_handle.0)
        .and_then(|weapons| weapons.0.get("dagger"))
        .cloned()
        .expect("Missing dagger weapon data");

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

    let weapon_bundle = weapon(&weapon_assets, &weapon_data);
    let weapon_entity = commands.spawn(weapon_bundle).id();

    commands.entity(player_entity).add_child(weapon_entity);
    commands.entity(level).add_child(player_entity);

    commands.entity(level).with_children(|parent| {
        parent.spawn((
            Name::new("Gameplay Music"),
            music(audio_assets.background.clone()),
        ));
    });

    commands.entity(player_entity).insert(Player {
        weapon: "dagger".to_string(),
        weapon_entity: Some(weapon_entity),
        last_shot_time: 0.0,
        switching_weapon: false,
        switch_timer: Timer::from_seconds(3.0, TimerMode::Once),
        can_shoot_timer: Timer::from_seconds(0.2, TimerMode::Once),
    });
}
