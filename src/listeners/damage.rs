use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems, components::Enemy, enemies::HitFlash, messages::DamageMessage,
    screens::Screen,
};

pub struct DamageListenerPlugin;

impl Plugin for DamageListenerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_damage
                .in_set(AppSystems::DamageEvents)
                .in_set(PausableSystems)
                .run_if(in_state(Screen::Gameplay)),
        );
    }
}

fn handle_damage(
    mut commands: Commands,
    mut reader: MessageReader<DamageMessage>,
    mut target_query: Query<(Option<&Enemy>, Option<&mut Sprite>, Option<&mut HitFlash>)>,
) {
    for msg in reader.read() {
        if msg.damage <= 0.0 || msg.remaining_health <= 0.0 || msg.killed {
            continue;
        }

        let Ok((maybe_enemy, maybe_sprite, maybe_flash)) = target_query.get_mut(msg.target) else {
            continue;
        };

        if maybe_enemy.is_none() {
            continue;
        }

        let Some(mut sprite) = maybe_sprite else {
            continue;
        };

        if let Some(mut flash) = maybe_flash {
            flash.restart();
            sprite.color = HitFlash::FLASH_COLOR;
        } else {
            let original_color = sprite.color;
            sprite.color = HitFlash::FLASH_COLOR;
            commands
                .entity(msg.target)
                .try_insert(HitFlash::new(original_color));
        }
    }
}
