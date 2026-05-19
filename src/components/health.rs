use bevy::prelude::*;

use crate::components::Player;
use crate::game::weapon::Weapon;
use crate::messages::{DamageMessage, EntityDiedMessage};
use crate::screens::Screen;

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Default for Health {
    fn default() -> Self {
        Health {
            current: 100.0,
            max: 100.0,
        }
    }
}

impl Health {
    pub fn new(max: f32) -> Self {
        Health { current: max, max }
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }

    pub fn take_damage(
        &mut self,
        target: Entity,
        damage: f32,
        writer: &mut MessageWriter<DamageMessage>,
    ) {
        if self.current <= 0.0 {
            return;
        }

        self.current -= damage;
        if self.current < 0.0 {
            self.current = 0.0;
        }

        writer.write(DamageMessage { target, damage });
    }

    pub fn heal(&mut self, amount: f32) {
        self.current += amount;
        if self.current > self.max {
            self.current = self.max;
        }
    }
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Messages<DamageMessage>>();
        app.init_resource::<Messages<EntityDiedMessage>>();
        app.add_systems(
            Update,
            despawn_dead_entities.run_if(in_state(Screen::Gameplay)),
        );
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
