use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

mod assets;
pub(crate) mod data;
pub(crate) mod spatial;
pub(crate) mod systems;

pub use assets::EnemyVisualsHandle;
pub use data::{Enemies, EnemiesDataHandle, EnemySpawner};
pub use systems::HitFlash;

#[derive(AutoPlugin)]
pub(crate) struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    #[auto_plugin]
    fn build(&self, app: &mut App) {
        app.add_plugins((
            data::EnemyDataPlugin,
            assets::EnemyAssetsPlugin,
            spatial::EnemySpatialPlugin,
            systems::SystemsPlugin,
        ));
    }
}

#[auto_system(
    plugin = EnemiesPlugin,
    schedule = OnEnter(crate::Screen::Loading)
)]
fn load_enemies_data(mut commands: Commands, server: Res<AssetServer>) {
    let enemies_data_handle = server.load("data/enemies_data.ron");
    commands.insert_resource(EnemiesDataHandle(enemies_data_handle));
}

#[auto_system(
    plugin = EnemiesPlugin,
    schedule = OnEnter(crate::Screen::Loading)
)]
fn load_enemy_visuals_data(mut commands: Commands, server: Res<AssetServer>) {
    let enemy_visuals_handle = server.load("data/enemies.assets.ron");
    commands.insert_resource(EnemyVisualsHandle(enemy_visuals_handle));
}
