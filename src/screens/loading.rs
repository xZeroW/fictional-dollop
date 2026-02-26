//! A loading screen during which game assets are loaded if necessary.
//! This reduces stuttering, especially for audio on Wasm.

use bevy::prelude::*;

use crate::{screens::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), spawn_loading_screen);

    app.add_systems(
        Update,
        spawn_loading_screen.run_if(in_state(Screen::Loading)),
    );

    // TODO: Remove this once we have a real loading process that takes time.
    app.add_systems(OnExit(Screen::Loading), slow_loading_system);
}

fn spawn_loading_screen(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Loading Screen"),
        DespawnOnExit(Screen::Loading),
        children![widget::label("Loading...")],
    ));
}

fn slow_loading_system() {
    // Simulate a long loading time to demonstrate the loading screen.
    std::thread::sleep(std::time::Duration::from_secs(2));
}
