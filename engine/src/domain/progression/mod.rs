mod monster;
mod wave;

use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

pub use monster::MonsterProgression;
pub use wave::WaveState;

#[derive(AutoPlugin)]
pub(crate) struct ProgressionPlugin;

impl Plugin for ProgressionPlugin {
    #[auto_plugin]
    fn build(&self, app: &mut App) {
        app.add_plugins((monster::MonsterProgressionPlugin, wave::WaveSystemsPlugin));
    }
}
