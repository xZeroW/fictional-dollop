mod monster;
mod wave;

use bevy::prelude::*;

pub use monster::MonsterProgression;
pub use wave::WaveState;

pub(crate) struct ProgressionPlugin;

impl Plugin for ProgressionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((monster::MonsterProgressionPlugin, wave::WaveSystemsPlugin));
    }
}
