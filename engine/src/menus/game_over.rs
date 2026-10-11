//! The game over menu shown over paused gameplay when the player dies.

use bevy::prelude::*;

use crate::{menus::Menu, screens::Screen, theme::widget};

pub(super) struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Menu::GameOver), spawn_game_over_menu);
    }
}

fn spawn_game_over_menu(mut commands: Commands) {
    commands.spawn((
        widget::full_screen_overlay("Game Over Overlay", Color::srgba(0.02, 0.0, 0.01, 0.86)),
        DespawnOnExit(Menu::GameOver),
    ));

    commands.spawn((
        widget::ui_root("Game Over Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::GameOver),
        children![
            widget::header("Game Over"),
            widget::label("You were overwhelmed."),
            widget::button("Retry", retry),
            widget::button("Quit to title", quit_to_title),
        ],
    ));
}

fn retry(_: On<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Loading);
}

fn quit_to_title(_: On<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
