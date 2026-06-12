//! The game's menus and transitions between them.

mod credits;
mod equipment;
mod game_over;
mod inventory;
mod item_tooltip;
mod main;
mod pause;
mod settings;

use bevy::prelude::*;
use bevy::{camera::ClearColorConfig, camera::visibility::RenderLayers};

const MENU_CAMERA_ORDER: isize = 30;
const MENU_RENDER_LAYER: usize = 2;

#[derive(Component)]
pub(super) struct MenuCamera;

pub(super) struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<Menu>();
        app.add_systems(Startup, spawn_menu_camera);

        app.add_plugins((
            credits::CreditsMenuPlugin,
            equipment::EquipmentMenuPlugin,
            game_over::GameOverMenuPlugin,
            inventory::InventoryMenuPlugin,
            main::MainMenuPlugin,
            settings::SettingsMenuPlugin,
            pause::PauseMenuPlugin,
        ));
    }
}

fn spawn_menu_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Menu Camera"),
        MenuCamera,
        Camera2d,
        Camera {
            order: MENU_CAMERA_ORDER,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        Msaa::Off,
        RenderLayers::layer(MENU_RENDER_LAYER),
    ));
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
