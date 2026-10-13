mod level;
mod map;

use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

pub(crate) use level::LevelEntity;

#[derive(AutoPlugin)]
pub(crate) struct WorldPlugin;

impl Plugin for WorldPlugin {
    #[auto_plugin]
    fn build(&self, app: &mut App) {
        app.add_plugins((level::LevelPlugin, map::MapPlugin));
    }
}
