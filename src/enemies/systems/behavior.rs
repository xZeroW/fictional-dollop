use bevy::prelude::*;
use std::time::Duration;

use crate::{
    components::{Behavior, Enemy, Movement, Player, WanderState},
    config,
};

const COWARD_DISTANCE: f32 = 100.0;
const COWARD_DISTANCE_SQUARED: f32 = COWARD_DISTANCE * COWARD_DISTANCE;
const WANDERING_SPEED: f32 = 0.5;
const FOLLOW_SPEED: f32 = 1.0;
const COWARD_RUN_SPEED: f32 = 1.5;

pub fn behavior(
    time: Res<Time>,
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemies: Query<
        (&Transform, &Behavior, &mut Movement, &mut WanderState),
        (With<Enemy>, Without<Player>),
    >,
) {
    let Ok(player_pos) = player.single() else {
        return;
    };

    let player_pos_2d = player_pos.translation.truncate();

    for (transform, behavior, mut movement, mut wander_state) in &mut enemies {
        let enemy_pos_2d = transform.translation.truncate();

        let (direction, speed) = match behavior {
            Behavior::Wandering => wandering(time.delta(), &mut wander_state),
            Behavior::FollowAndAttack => follow_and_attack(player_pos_2d, enemy_pos_2d),
            Behavior::Coward => coward(player_pos_2d, enemy_pos_2d, &mut wander_state),
        };

        movement.intent = direction * speed;
    }
}

fn wandering(delta: Duration, state: &mut WanderState) -> (Vec2, f32) {
    state.timer.tick(delta);

    if state.timer.just_finished() {
        state.direction =
            Vec2::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5).normalize_or_zero();
    }

    (state.direction, WANDERING_SPEED)
}

fn follow_and_attack(player_pos: Vec2, enemy_pos: Vec2) -> (Vec2, f32) {
    let contact_radius = config::PLAYER_ENEMY_CONTACT_RADIUS;
    if player_pos.distance_squared(enemy_pos) <= contact_radius * contact_radius {
        return (Vec2::ZERO, 0.0);
    }

    ((player_pos - enemy_pos).normalize_or_zero(), FOLLOW_SPEED)
}

fn coward(player_pos: Vec2, enemy_pos: Vec2, wander_state: &mut WanderState) -> (Vec2, f32) {
    if player_pos.distance_squared(enemy_pos) <= COWARD_DISTANCE_SQUARED {
        (
            (enemy_pos - player_pos).normalize_or_zero(),
            COWARD_RUN_SPEED,
        )
    } else {
        wandering(Duration::ZERO, wander_state)
    }
}

pub fn prevent_enemy_player_overlap(
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemies: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
) {
    let Ok(player_transform) = player.single() else {
        return;
    };

    let player_pos = player_transform.translation.truncate();
    let min_distance = config::PLAYER_ENEMY_CONTACT_RADIUS;
    let min_distance_squared = min_distance * min_distance;
    let (min_x, max_x, min_y, max_y) = config::map_bounds();

    for mut enemy_transform in &mut enemies {
        let enemy_pos = enemy_transform.translation.truncate();
        let offset = enemy_pos - player_pos;
        let distance_squared = offset.length_squared();

        if distance_squared >= min_distance_squared {
            continue;
        }

        let direction = if distance_squared <= f32::EPSILON {
            Vec2::X
        } else {
            offset.normalize()
        };
        let resolved_pos = player_pos + direction * min_distance;

        enemy_transform.translation.x = resolved_pos.x.clamp(min_x, max_x);
        enemy_transform.translation.y = resolved_pos.y.clamp(min_y, max_y);
    }
}
