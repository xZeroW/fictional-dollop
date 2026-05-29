// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows outside debug dev builds.
#![cfg_attr(
    not(all(feature = "dev", debug_assertions)),
    windows_subsystem = "windows"
)]

mod assets;
mod audio;
mod components;
mod config;
#[cfg(all(feature = "dev", debug_assertions))]
mod dev_tools;
mod enemies;
mod game;
mod hud;
mod listeners;
mod menus;
mod messages;
mod screens;
mod systems;
mod theme;

use listeners::ListenersPlugin;
use systems::SystemsPlugin;

use bevy::{prelude::*, window::PresentMode};

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Window {
                    title: "My game".to_string(),
                    fit_canvas_to_parent: true,
                    present_mode: PresentMode::AutoNoVsync,
                    ..default()
                }
                .into(),
                ..default()
            }),
        );

        // Add other plugins.
        app.add_plugins((
            assets::AssetsPlugin,
            audio::AudioPlugin,
            game::GamePlugin,
            hud::HudPlugin,
            SystemsPlugin,
            ListenersPlugin,
            menus::MenusPlugin,
            screens::ScreensPlugin,
            theme::ThemePlugin,
            enemies::EnemiesPlugin,
        ));

        app.add_plugins((
            #[cfg(all(feature = "dev", debug_assertions))]
            dev_tools::DevToolsPlugin,
        ));

        // Order new `AppSystems` variants by adding them here:
        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
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

        // Set up the `Pause` state.
        app.init_state::<Pause>();
        // Configure systems that should only run when the game is not paused.
        app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
    /// Rebuild shared spatial indexes after normal gameplay updates.
    SpatialIndex,
    /// Run systems that query shared spatial indexes.
    SpatialQueries,
    /// Convert collision and hit messages into gameplay effects.
    CollisionEvents,
    /// Apply pending damage to health components.
    ApplyDamage,
    /// Run reactions to applied damage.
    DamageEvents,
    /// Run reactions to entity deaths.
    DeathEvents,
    /// Process wave transitions after gameplay updates.
    WaveTransitions,
}

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
struct Pause(pub bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct PausableSystems;
