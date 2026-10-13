use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[derive(Message, Debug, Clone)]
#[auto_add_message(plugin = super::CombatPlugin)]
pub struct CollisionMessage {
    pub entity_a: Entity,
    pub entity_b: Entity,
    pub kind: CollisionKind,
}

#[derive(Message, Debug, Clone)]
#[auto_add_message(plugin = super::CombatPlugin)]
pub struct BulletHitEnemyMessage {
    pub enemy: Entity,
    pub damage: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CollisionKind {
    DamagePlayer,
}

#[derive(Message, Debug, Clone)]
#[auto_add_message(plugin = super::CombatPlugin)]
pub struct ApplyDamageMessage {
    pub target: Entity,
    pub damage: f32,
}

#[derive(Message, Debug, Clone)]
#[auto_add_message(plugin = super::CombatPlugin)]
pub struct DamageMessage {
    pub target: Entity,
    /// Damage that was actually applied after clamping to remaining health.
    pub damage: f32,
    pub remaining_health: f32,
    pub killed: bool,
}

#[derive(Message, Debug, Clone)]
#[auto_add_message(plugin = super::CombatPlugin)]
pub struct EntityDiedMessage {
    pub entity: Entity,
    pub is_player: bool,
    pub position: Vec3,
    pub enemy_type: Option<String>,
}
