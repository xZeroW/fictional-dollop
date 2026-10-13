pub mod assets;
pub mod config;
pub mod weapon;

use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(AutoPlugin)]
pub(crate) struct InfrastructurePlugin;

impl Plugin for InfrastructurePlugin {
    #[auto_plugin]
    fn build(&self, app: &mut App) {
        app.add_plugins((
            assets::AssetsPlugin,
            config::ConfigPlugin,
            weapon::WeaponDataPlugin,
        ));
    }
}
