//! The screen state for the main gameplay.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{Pause, menus::Menu, screens::Screen, theme::widget};

pub(super) struct GameplayScreenPlugin;

impl Plugin for GameplayScreenPlugin {
    fn build(&self, app: &mut App) {
        // Toggle pause on key press.
        app.add_systems(
            Update,
            (
                (pause, spawn_pause_overlay, open_pause_menu).run_if(
                    in_state(Screen::Gameplay).and(in_state(Menu::None)).and(
                        input_just_pressed(KeyCode::KeyP).or(input_just_pressed(KeyCode::Escape)),
                    ),
                ),
                close_menu.run_if(
                    in_state(Screen::Gameplay)
                        .and(not(in_state(Menu::None)))
                        .and(not(in_state(Menu::MonsterBuff)))
                        .and(not(in_state(Menu::GameOver)))
                        .and(input_just_pressed(KeyCode::KeyP)),
                ),
            ),
        );
        app.add_systems(OnExit(Screen::Gameplay), (close_menu, unpause));
        app.add_systems(
            OnEnter(Menu::None),
            unpause.run_if(in_state(Screen::Gameplay)),
        );
    }
}

fn unpause(mut next_pause: ResMut<NextState<Pause>>) {
    next_pause.set(Pause(false));
}

fn pause(mut next_pause: ResMut<NextState<Pause>>) {
    next_pause.set(Pause(true));
}

fn spawn_pause_overlay(mut commands: Commands) {
    commands.spawn((
        widget::full_screen_overlay("Pause Overlay", Color::srgba(0.0, 0.0, 0.0, 0.8)),
        DespawnOnExit(Pause(true)),
    ));
}

fn open_pause_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pause);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
