mod level;
mod map;

use bevy::prelude::*;

pub(crate) use level::LevelEntity;

pub(crate) struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((level::LevelPlugin, map::MapPlugin));
    }
}
