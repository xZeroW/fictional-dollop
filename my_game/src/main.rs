#![cfg_attr(
    not(all(feature = "dev", debug_assertions)),
    windows_subsystem = "windows"
)]

use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins((engine::AppPlugin, ui::UiPlugin))
        .run()
}
