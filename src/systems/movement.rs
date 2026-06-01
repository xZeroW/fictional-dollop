//! Apply movement based on [`Movement`] intent and maximum speed.

use bevy::prelude::*;

use crate::{AppSystems, PausableSystems, components::Movement, config::map_bounds};

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
    let (min_x, max_x, min_y, max_y) = map_bounds();

    for (movement, mut transform) in &mut movement_query {
        let velocity = movement.intent * movement.speed;
        transform.translation += velocity.extend(0.0) * time.delta_secs();
        transform.translation.x = transform.translation.x.clamp(min_x, max_x);
        transform.translation.y = transform.translation.y.clamp(min_y, max_y);
    }
}
