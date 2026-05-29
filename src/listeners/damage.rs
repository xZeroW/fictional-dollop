use bevy::prelude::*;

use crate::messages::DamageMessage;

pub struct DamageListener;

impl Plugin for DamageListener {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_damage);
    }
}

fn handle_damage(mut reader: MessageReader<DamageMessage>) {
    for msg in reader.read() {
        let _ = (msg.target, msg.damage);
    }
}
