use bevy::prelude::*;

use crate::{
    components::{Health, Player},
    game::weapon::Weapon,
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
            (
                apply_damage,
                despawn_dead_entities,
            )
                .run_if(in_state(Screen::Gameplay)),
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
    query: Query<(Entity, &Health, Option<&Player>, Option<&Transform>)>,
    weapon_query: Query<Entity, With<Weapon>>,
    mut death_writer: MessageWriter<EntityDiedMessage>,
) {
    let mut to_despawn = Vec::new();

    for (entity, health, maybe_player, maybe_transform) in query.iter() {
        if health.is_dead() {
            to_despawn.push((entity, maybe_player.is_some(), maybe_transform.map(|t| t.translation)));
        }
    }

    for (entity, is_player, position) in to_despawn {
        if is_player {
            for weapon_entity in weapon_query.iter() {
                if commands.get_entity(weapon_entity).is_ok() {
                    commands.entity(weapon_entity).despawn();
                }
            }
        }

        death_writer.write(EntityDiedMessage {
            entity,
            position,
            is_player,
        });

        if commands.get_entity(entity).is_ok() {
            commands.entity(entity).despawn();
        }
    }
}