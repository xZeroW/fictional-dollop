use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;
use listeners::ListenersPlugin;
use systems::SystemsPlugin;

pub mod assets;
pub mod audio;
pub mod components;
pub mod config;
pub mod domain;
mod enemies;
pub mod game;
mod listeners;
mod messages;
pub mod systems;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Window {
                    title: "My game".to_string(),
                    fit_canvas_to_parent: true,
                    ..default()
                }
                .into(),
                ..default()
            }),
        );

        app.init_state::<Screen>();
        app.init_state::<Menu>();
        app.init_state::<Pause>();

        app.add_plugins((
            assets::AssetsPlugin,
            audio::AudioPlugin,
            game::GamePlugin,
            SystemsPlugin,
            ListenersPlugin,
            enemies::EnemiesPlugin,
            enemies::systems::SystemsPlugin,
        ));

        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
                AppSystems::ResolveContacts,
                AppSystems::SpatialIndex,
                AppSystems::SpatialQueries,
                AppSystems::CollisionEvents,
                AppSystems::ApplyDamage,
                AppSystems::DamageEvents,
                AppSystems::DeathEvents,
                AppSystems::WaveTransitions,
            )
                .chain(),
        );
        app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));
    }
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum AppSystems {
    TickTimers,
    RecordInput,
    Update,
    ResolveContacts,
    SpatialIndex,
    SpatialQueries,
    CollisionEvents,
    ApplyDamage,
    DamageEvents,
    DeathEvents,
    WaveTransitions,
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Pause(pub bool);

#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PausableSystems;

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Screen {
    #[default]
    Splash,
    Title,
    Loading,
    Gameplay,
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Menu {
    #[default]
    None,
    Main,
    Credits,
    Settings,
    Inventory,
    Equipment,
    Pause,
    GameOver,
}
