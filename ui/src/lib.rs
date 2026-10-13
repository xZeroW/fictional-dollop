use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;
use engine::{AppSystems, Pause, domain, infrastructure};

#[cfg(all(feature = "dev", debug_assertions))]
mod dev_tools;
mod hud;
mod menus;
pub mod screen;
mod theme;

#[derive(AutoPlugin)]
pub struct UiPlugin;

impl Plugin for UiPlugin {
    #[auto_plugin]
    fn build(&self, app: &mut App) {
        // Add other plugins.
        app.add_plugins((
            hud::HudPlugin,
            menus::MenusPlugin,
            menus::credits::CreditsMenuPlugin,
            menus::equipment::EquipmentMenuPlugin,
            menus::game_over::GameOverMenuPlugin,
            menus::inventory::InventoryMenuPlugin,
            menus::main::MainMenuPlugin,
            menus::settings::SettingsMenuPlugin,
            menus::pause::PauseMenuPlugin,
            screen::ScreensPlugin,
            theme::ThemePlugin,
        ));

        app.add_plugins((
            #[cfg(all(feature = "dev", debug_assertions))]
            dev_tools::DevToolsPlugin,
        ));
    }
}
