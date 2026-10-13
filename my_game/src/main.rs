#![cfg_attr(
    not(all(feature = "dev", debug_assertions)),
    windows_subsystem = "windows"
)]

use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "My game".to_string(),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((engine::EnginePlugin, ui::UiPlugin))
        .run()
}
