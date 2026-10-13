mod animation;
pub(crate) mod attributes;
mod components;
mod flip_sprite;
mod movement;
mod player;

use bevy::prelude::*;
use bevy_gauge::prelude::AttributesPlugin;
use leafwing_input_manager::prelude::InputManagerPlugin;

pub use components::{Behavior, Enemy, Health, Movement, Player, WanderState};
pub use player::PlayerAction;
pub(crate) use player::player;

pub(crate) struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AttributesPlugin,
            InputManagerPlugin::<PlayerAction>::default(),
            player::PlayerPlugin,
            animation::AnimationSystemsPlugin,
            movement::MovementSystemsPlugin,
        ));
        app.add_systems(
            Update,
            flip_sprite::flip_sprite.in_set(crate::PausableSystems),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_gauge::graph::DependencyGraph;

    #[test]
    fn entities_plugin_initializes_attribute_resources() {
        let mut app = App::new();
        app.add_plugins(EntitiesPlugin);

        assert!(app.world().contains_resource::<DependencyGraph>());
    }
}
