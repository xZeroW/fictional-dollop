use bevy::prelude::*;

use crate::messages::EntityDiedMessage;

pub struct DeathListener;

impl Plugin for DeathListener {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_death);
    }
}

fn handle_death(mut reader: MessageReader<EntityDiedMessage>) {
    for msg in reader.read() {
        let _ = msg.entity;

        if msg.is_player {
            println!("Player died!");
        }
    }
}
