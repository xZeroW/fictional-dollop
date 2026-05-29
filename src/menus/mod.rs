//! The game's menus and transitions between them.

mod credits;
mod main;
mod pause;
mod settings;

use bevy::prelude::*;

pub(super) struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<Menu>();

        app.add_plugins((
            credits::CreditsMenuPlugin,
            main::MainMenuPlugin,
            settings::SettingsMenuPlugin,
            pause::PauseMenuPlugin,
        ));
    }
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Menu {
    #[default]
    None,
    Main,
    Credits,
    Settings,
    Pause,
}
