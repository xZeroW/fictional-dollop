//! A loading screen during which game assets are loaded if necessary.
//! This reduces stuttering, especially for audio on Wasm.

use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

use crate::{screen::Screen, theme::prelude::*};

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
pub(super) struct LoadingScreenPlugin;

#[auto_system(plugin = LoadingScreenPlugin, schedule = OnEnter(Screen::Loading))]
fn spawn_loading_screen(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Loading Screen"),
        DespawnOnExit(Screen::Loading),
        children![widget::label("Loading...")],
    ));
}
