pub mod audio;
pub mod camera;
pub mod combat;
pub mod enemies;
pub mod entities;
pub mod inventory;
pub mod progression;
pub mod world;

use bevy::prelude::*;

pub(crate) struct DomainPlugin;

impl Plugin for DomainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            audio::AudioPlugin,
            camera::CameraPlugin,
            entities::EntitiesPlugin,
            world::WorldPlugin,
            enemies::EnemiesPlugin,
            combat::CombatPlugin,
            inventory::InventoryPlugin,
            progression::ProgressionPlugin,
        ));
    }
}
