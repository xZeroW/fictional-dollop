pub mod damage;
pub mod death;

pub struct ListenersPlugin;

use bevy::prelude::*;

impl Plugin for ListenersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((damage::DamageListener, death::DeathListener));
    }
}
