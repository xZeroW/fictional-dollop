use bevy::prelude::*;

use crate::{
    components::{Enemy, Health, Player},
    enemies::HitFlash,
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
    mut commands: Commands,
    mut damage_reader: MessageReader<ApplyDamageMessage>,
    mut health_query: Query<(
        Entity,
        &mut Health,
        Option<&Enemy>,
        Option<&mut Sprite>,
        Option<&mut HitFlash>,
    )>,
    mut damage_writer: MessageWriter<DamageMessage>,
) {
    for msg in damage_reader.read() {
        if let Ok((entity, mut health, maybe_enemy, maybe_sprite, maybe_flash)) =
            health_query.get_mut(msg.target)
        {
            health.apply_damage(msg.damage);
            damage_writer.write(DamageMessage {
                target: msg.target,
                damage: msg.damage,
            });

            if health.is_dead() {
                continue;
            }

            if maybe_enemy.is_some() {
                if let Some(mut sprite) = maybe_sprite {
                    if let Some(mut flash) = maybe_flash {
                        flash.restart();
                        sprite.color = HitFlash::FLASH_COLOR;
                    } else {
                        let original_color = sprite.color;
                        sprite.color = HitFlash::FLASH_COLOR;
                        commands
                            .entity(entity)
                            .try_insert(HitFlash::new(original_color));
                    }
                }
            }
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
