use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use crate::screens::Screen;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TiledPlugin::default())
            .add_systems(OnEnter(Screen::Gameplay), startup);
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<TiledMapAsset> = asset_server.load("maps/map1.tmx");

    commands.spawn((TiledMap(map_handle), TilemapAnchor::Center));
}
