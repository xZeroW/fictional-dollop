//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use bevy::prelude::*;
use bevy_gauge::prelude::*;
use leafwing_input_manager::prelude::*;

mod animation;
pub mod camera;
pub mod collision;
pub mod level;
mod map;
pub mod movement;
pub mod player;
mod spatial;
pub mod weapon;
mod weapon_data;

use player::PlayerAction;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<crate::config::GameConfig>();
    app.add_plugins((
        InputManagerPlugin::<PlayerAction>::default(),
        animation::plugin,
        camera::CameraPlugin,
        collision::CollisionPlugin,
        level::plugin,
        movement::plugin,
        player::plugin,
        weapon::plugin,
        weapon_data::plugin,
        map::MapPlugin,
        AttributesPlugin,
    ));
}
