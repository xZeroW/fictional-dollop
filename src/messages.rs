use bevy::prelude::*;

#[derive(Message, Debug, Clone)]
pub struct CollisionMessage {
    pub entity_a: Entity,
    pub entity_b: Entity,
    pub kind: CollisionKind,
}

#[derive(Message, Debug, Clone)]
pub struct BulletHitEnemyMessage {
    pub enemy: Entity,
    pub damage: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CollisionKind {
    DamagePlayer,
}

#[derive(Message, Debug, Clone)]
pub struct ApplyDamageMessage {
    pub target: Entity,
    pub damage: f32,
}

#[derive(Message, Debug, Clone)]
pub struct DamageMessage {
    pub target: Entity,
    /// Damage that was actually applied after clamping to remaining health.
    pub damage: f32,
    pub remaining_health: f32,
    pub killed: bool,
}

#[derive(Message, Debug, Clone)]
pub struct EntityDiedMessage {
    pub entity: Entity,
    pub is_player: bool,
}
