use bevy::prelude::*;

#[derive(Message, Debug, Clone)]
pub struct CollisionMessage {
    pub entity_a: Entity,
    pub entity_b: Entity,
    pub position: Vec2,
    pub kind: CollisionKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CollisionKind {
    DamagePlayer,
    DamageEnemy,
}

#[derive(Message, Debug, Clone)]
pub struct ApplyDamageMessage {
    pub target: Entity,
    pub damage: f32,
}

#[derive(Message, Debug, Clone)]
pub struct DamageMessage {
    pub target: Entity,
    pub damage: f32,
}

#[derive(Message, Debug, Clone)]
pub struct EntityDiedMessage {
    pub entity: Entity,
    pub is_player: bool,
}
