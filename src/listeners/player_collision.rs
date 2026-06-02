use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    components::{AttackCooldown, Damage, Enemy},
    messages::{ApplyDamageMessage, CollisionKind, CollisionMessage},
    screens::Screen,
};

pub struct PlayerCollisionListenerPlugin;

impl Plugin for PlayerCollisionListenerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_enemy_player_collision
                .in_set(AppSystems::CollisionEvents)
                .in_set(PausableSystems)
                .run_if(in_state(Screen::Gameplay)),
        );
    }
}

fn handle_enemy_player_collision(
    mut collision_reader: MessageReader<CollisionMessage>,
    enemy_query: Query<(&Damage, &AttackCooldown), With<Enemy>>,
    mut damage_writer: MessageWriter<ApplyDamageMessage>,
) {
    for collision in collision_reader.read() {
        if collision.kind != CollisionKind::DamagePlayer {
            continue;
        }

        let player_entity = collision.entity_a;
        let enemy_entity = collision.entity_b;

        let Ok((damage, cooldown)) = enemy_query.get(enemy_entity) else {
            continue;
        };

        if !cooldown.timer.just_finished() {
            continue;
        }

        damage_writer.write(ApplyDamageMessage {
            target: player_entity,
            damage: damage.value,
        });
    }
}
