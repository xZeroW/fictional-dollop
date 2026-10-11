use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    components::{Enemy, Health, Player},
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
            apply_damage
                .in_set(AppSystems::ApplyDamage)
                .in_set(PausableSystems)
                .run_if(in_state(Screen::Gameplay)),
        );
        app.add_systems(
            Update,
            despawn_dead_entities
                .in_set(AppSystems::DamageEvents)
                .in_set(PausableSystems)
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
            let damage = health.apply_damage(msg.damage);

            if damage <= 0.0 {
                continue;
            }

            damage_writer.write(DamageMessage {
                target: msg.target,
                damage,
                remaining_health: health.current,
                killed: health.is_dead(),
            });
        }
    }
}

fn despawn_dead_entities(
    mut commands: Commands,
    query: Query<(Entity, &Health, &Transform, Option<&Player>, Option<&Enemy>)>,
    mut death_writer: MessageWriter<EntityDiedMessage>,
    mut spawner: ResMut<crate::enemies::EnemySpawner>,
) {
    let mut to_despawn = Vec::new();

    for (entity, health, transform, maybe_player, maybe_enemy) in query.iter() {
        if health.is_dead() {
            to_despawn.push((
                entity,
                maybe_player.is_some(),
                transform.translation,
                maybe_enemy.map(|enemy| enemy.enemy_type.clone()),
            ));
        }
    }

    for (entity, is_player, position, enemy_type) in to_despawn {
        let was_enemy = enemy_type.is_some();

        death_writer.write(EntityDiedMessage {
            entity,
            is_player,
            position,
            enemy_type,
        });

        if commands.get_entity(entity).is_ok() {
            if was_enemy {
                spawner.spawned_count = spawner.spawned_count.saturating_sub(1);
            }
            commands.entity(entity).despawn();
        }
    }
}
