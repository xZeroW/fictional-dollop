//! The game's menus and transitions between them.

pub(crate) mod credits;
pub(crate) mod equipment;
pub(crate) mod game_over;
pub(crate) mod inventory;
mod item_tooltip;
pub(crate) mod main;
pub(crate) mod pause;
pub(crate) mod settings;

use bevy::prelude::*;
use bevy::{camera::ClearColorConfig, camera::visibility::RenderLayers};
pub use engine::Menu;

const MENU_CAMERA_ORDER: isize = 30;
const MENU_RENDER_LAYER: usize = 2;

#[derive(Component)]
pub(super) struct MenuCamera;

pub(super) struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_menu_camera);
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
