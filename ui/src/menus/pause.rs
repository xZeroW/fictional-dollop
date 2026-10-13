//! The pause menu.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_auto_plugin::prelude::*;

use crate::{menus::Menu, screen::Screen, theme::widget};

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
pub(crate) struct PauseMenuPlugin;

#[auto_system(plugin = PauseMenuPlugin, schedule = OnEnter(Menu::Pause))]
fn spawn_pause_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Pause Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Pause),
        children![
            widget::header("Game paused"),
            widget::button("Continue", close_menu),
            widget::button("Settings", open_settings_menu),
            widget::button("Quit to title", quit_to_title),
        ],
    ));
}

fn open_settings_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn close_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn quit_to_title(_: On<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

#[auto_system(
    plugin = PauseMenuPlugin,
    schedule = Update,
    config(run_if = in_state(Menu::Pause).and_then(input_just_pressed(KeyCode::Escape)))
)]
fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
