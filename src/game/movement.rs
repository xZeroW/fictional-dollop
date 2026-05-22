//! Handle player input and translate it into movement through a character
//! controller. A character controller is the collection of systems that govern
//! the movement of characters.
//!
//! In our case, the character controller has the following logic:
//! - Set [`Movement`] intent based on directional keyboard input.
//!   This is done in the `player` module, as it is specific to the player
//!   character.
//! - Apply movement based on [`Movement`] intent and maximum speed.
//! - Wrap the character within the window.
//!
//! Note that the implementation used here is limited for demonstration
//! purposes. If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/main/examples/movement/physics_in_fixed_timestep.rs).

use bevy::prelude::*;

use crate::{AppSystems, PausableSystems, components::Movement};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        apply_movement
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

fn apply_movement(time: Res<Time>, mut movement_query: Query<(&Movement, &mut Transform)>) {
    for (movement, mut transform) in &mut movement_query {
        let velocity = movement.intent * movement.speed;
        transform.translation += velocity.extend(0.0) * time.delta_secs();
    }
}
