//! The title screen that appears after the splash screen.

use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

use crate::{menus::Menu, screen::Screen};

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
pub(super) struct TitleScreenPlugin;

#[auto_system(plugin = TitleScreenPlugin, schedule = OnEnter(Screen::Title))]
fn open_main_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

#[auto_system(plugin = TitleScreenPlugin, schedule = OnExit(Screen::Title))]
fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
