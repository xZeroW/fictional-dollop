//! Apply movement based on [`Movement`] intent and maximum speed.

use bevy::prelude::*;

use crate::{AppSystems, PausableSystems, components::Movement};

pub(super) struct MovementSystemsPlugin;

impl Plugin for MovementSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            apply_movement
                .in_set(AppSystems::Update)
                .in_set(PausableSystems),
        );
    }
}

fn apply_movement(time: Res<Time>, mut movement_query: Query<(&Movement, &mut Transform)>) {
    for (movement, mut transform) in &mut movement_query {
        let velocity = movement.intent * movement.speed;
        transform.translation += velocity.extend(0.0) * time.delta_secs();
    }
}
