pub mod assets;
pub mod config;
pub mod weapon;

use bevy::prelude::*;

pub(crate) struct InfrastructurePlugin;

impl Plugin for InfrastructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            assets::AssetsPlugin,
            config::ConfigPlugin,
            weapon::WeaponDataPlugin,
        ));
    }
}
