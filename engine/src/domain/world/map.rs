use crate::Screen;
use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;
use bevy_ecs_tiled::prelude::*;

#[derive(AutoPlugin)]
pub struct MapPlugin;

impl Plugin for MapPlugin {
    #[auto_plugin]
    fn build(&self, app: &mut App) {
        app.add_plugins(TiledPlugin::default());
    }
}

#[auto_system(plugin = MapPlugin, schedule = OnEnter(Screen::Gameplay))]
fn load_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<TiledMapAsset> = asset_server.load("maps/map1.tmx");

    commands.spawn((
        TiledMap(map_handle),
        TilemapAnchor::Center,
        DespawnOnExit(Screen::Gameplay),
    ));
}
