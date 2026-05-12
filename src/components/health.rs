use bevy::prelude::*;

use crate::game::player::Player;
use crate::game::weapon::Weapon;
use crate::screens::Screen;

/// Event emitted when an entity dies. Non-player deaths are used for item drops.
pub struct EntityDiedEvent {
    pub entity: Entity,
    pub position: Option<Vec3>,
    pub is_player: bool,
}

/// Simple resource queue to hold death notifications for other systems.
#[derive(Default, Resource)]
pub struct DeathQueue(pub Vec<EntityDiedEvent>);

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

    pub fn take_damage(&mut self, damage: f32) {
        if self.current <= 0.0 {
            return;
        }

        self.current -= damage;
        if self.current < 0.0 {
            self.current = 0.0;
        }
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
    mut death_queue: ResMut<DeathQueue>,
) {
    // Loop por todas entidades com componente Health
    for (entity, health, maybe_player, maybe_transform) in query.iter() {
        // se health <= 0
        if health.is_dead() {
            // if it's a player, despawn weapons as well
            if maybe_player.is_some() {
                for weapon_entity in weapon_query.iter() {
                    commands.entity(weapon_entity).despawn();
                }
                (*death_queue).0.push(EntityDiedEvent {
                    entity,
                    position: maybe_transform.map(|t| t.translation),
                    is_player: true,
                });
            } else {
                // non-player entity died: queue for item drops
                (*death_queue).0.push(EntityDiedEvent {
                    entity,
                    position: maybe_transform.map(|t| t.translation),
                    is_player: false,
                });
            }

            commands.entity(entity).despawn();
        }
    }
}
