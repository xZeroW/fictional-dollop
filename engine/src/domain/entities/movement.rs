//! Apply movement based on [`Movement`] intent and maximum speed.

use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;
use bevy_gauge::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    domain::entities::{
        Movement, Player,
        attributes::{MOVEMENT_SPEED, MOVEMENT_SPEED_BASE},
    },
    infrastructure::config::map_bounds,
};

#[derive(AutoPlugin)]
pub(super) struct MovementSystemsPlugin;

impl Plugin for MovementSystemsPlugin {
    #[auto_plugin]
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            apply_movement
                .in_set(AppSystems::Update)
                .in_set(PausableSystems),
        );
    }
}

fn apply_movement(
    time: Res<Time>,
    mut attributes: AttributesMut,
    mut movement_query: Query<(Entity, &Movement, &mut Transform, Option<&Player>)>,
) {
    let (min_x, max_x, min_y, max_y) = map_bounds();

    for (entity, movement, mut transform, maybe_player) in &mut movement_query {
        let speed = if maybe_player.is_some() {
            attributes.set_base(entity, MOVEMENT_SPEED_BASE, movement.speed);
            attributes.evaluate(entity, MOVEMENT_SPEED)
        } else {
            movement.speed
        };
        let velocity = movement.intent * speed;
        transform.translation += velocity.extend(0.0) * time.delta_secs();
        transform.translation.x = transform.translation.x.clamp(min_x, max_x);
        transform.translation.y = transform.translation.y.clamp(min_y, max_y);
    }
}
