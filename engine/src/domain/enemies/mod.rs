use bevy::prelude::*;

mod assets;
pub(crate) mod data;
pub(crate) mod spatial;
pub(crate) mod systems;

pub use assets::EnemyVisualsHandle;
pub use data::{Enemies, EnemiesDataHandle, EnemySpawner};
pub use systems::HitFlash;

pub(crate) struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            data::EnemyDataPlugin,
            assets::EnemyAssetsPlugin,
            spatial::EnemySpatialPlugin,
            systems::SystemsPlugin,
        ));
        app.add_systems(OnEnter(crate::Screen::Loading), load_enemies_data);
        app.add_systems(OnEnter(crate::Screen::Loading), load_enemy_visuals_data);
    }
}

fn load_enemies_data(mut commands: Commands, server: Res<AssetServer>) {
    let enemies_data_handle = server.load("data/enemies_data.ron");
    commands.insert_resource(EnemiesDataHandle(enemies_data_handle));
}

fn load_enemy_visuals_data(mut commands: Commands, server: Res<AssetServer>) {
    let enemy_visuals_handle = server.load("data/enemies.assets.ron");
    commands.insert_resource(EnemyVisualsHandle(enemy_visuals_handle));
}
