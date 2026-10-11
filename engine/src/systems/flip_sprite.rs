use bevy::prelude::*;

use crate::components::Movement;

pub fn flip_sprite(mut sprites: Query<(&Movement, &mut Sprite)>) {
    for (movement, mut sprite) in &mut sprites {
        if movement.intent.x != 0.0 {
            sprite.flip_x = movement.intent.x < 0.0;
        }
    }
}
