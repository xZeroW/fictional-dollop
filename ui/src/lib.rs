use bevy::prelude::*;
use engine::{AppSystems, Pause, assets, audio, components, config, domain, game, systems};

#[cfg(all(feature = "dev", debug_assertions))]
mod dev_tools;
mod hud;
mod menus;
pub mod screen;
mod theme;

pub struct UiPlugin;

impl Plugin for UiPlugin {
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
