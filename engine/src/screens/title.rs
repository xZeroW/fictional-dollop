//! The title screen that appears after the splash screen.

use bevy::prelude::*;

use crate::{menus::Menu, screens::Screen};

pub(super) struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Title), open_main_menu);
        app.add_systems(OnExit(Screen::Title), close_menu);
    }
}

fn open_main_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
