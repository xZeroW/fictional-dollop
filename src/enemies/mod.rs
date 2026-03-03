use bevy::prelude::*;

mod monster_data;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((monster_data::plugin,));
}
