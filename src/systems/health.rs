use bevy::prelude::*;

use crate::{
    components::{Health, Player},
    messages::{ApplyDamageMessage, DamageMessage, EntityDiedMessage},
    screens::Screen,
};

pub struct HealthSystemsPlugin;

impl Plugin for HealthSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Messages<ApplyDamageMessage>>();
        app.init_resource::<Messages<DamageMessage>>();
        app.init_resource::<Messages<EntityDiedMessage>>();
        app.add_systems(
            Update,
            (apply_damage, despawn_dead_entities).run_if(in_state(Screen::Gameplay)),
        );
    }
}

fn apply_damage(
    mut damage_reader: MessageReader<ApplyDamageMessage>,
    mut health_query: Query<&mut Health>,
    mut damage_writer: MessageWriter<DamageMessage>,
) {
    for msg in damage_reader.read() {
        if let Ok(mut health) = health_query.get_mut(msg.target) {
            health.apply_damage(msg.damage);
            damage_writer.write(DamageMessage {
                target: msg.target,
                damage: msg.damage,
            });
        }
    }
}

fn despawn_dead_entities(
    mut commands: Commands,
    query: Query<(Entity, &Health, Option<&Player>)>,
    mut death_writer: MessageWriter<EntityDiedMessage>,
    mut spawner: ResMut<crate::enemies::EnemySpawner>,
    enemy_query: Query<&crate::components::Enemy>,
) {
    let mut to_despawn = Vec::new();

    for (entity, health, maybe_player) in query.iter() {
        if health.is_dead() {
            to_despawn.push((entity, maybe_player.is_some()));
        }
    }

    for (entity, is_player) in to_despawn {
        death_writer.write(EntityDiedMessage { entity, is_player });

        if commands.get_entity(entity).is_ok() {
            if enemy_query.get(entity).is_ok() {
                spawner.spawned_count = spawner.spawned_count.saturating_sub(1);
            }
            commands.entity(entity).despawn();
        }
    }
}
