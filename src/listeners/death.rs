use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems, Pause, menus::Menu, messages::EntityDiedMessage, screens::Screen,
};

pub struct DeathListenerPlugin;

impl Plugin for DeathListenerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_death
                .in_set(AppSystems::DeathEvents)
                .in_set(PausableSystems)
                .run_if(in_state(Screen::Gameplay)),
        );
    }
}

fn handle_death(
    mut reader: MessageReader<EntityDiedMessage>,
    mut next_menu: ResMut<NextState<Menu>>,
    mut next_pause: ResMut<NextState<Pause>>,
) {
    for msg in reader.read() {
        let _ = msg.entity;

        if msg.is_player {
            next_pause.set(Pause(true));
            next_menu.set(Menu::GameOver);
        }
    }
}
