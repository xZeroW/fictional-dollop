//! Gameplay attributes and derived stats.

use bevy_gauge::prelude::*;

pub(crate) const STRENGTH: &str = "Strength";
pub(crate) const DEXTERITY: &str = "Dexterity";
pub(crate) const INTELLIGENCE: &str = "Intelligence";
pub(crate) const VITALITY: &str = "Vitality";
pub(crate) const ATTACK_DAMAGE: &str = "AttackDamage";
pub(crate) const ATTACK_DAMAGE_BASE: &str = "AttackDamage.base";
pub(crate) const ATTACK_DAMAGE_INCREASED: &str = "AttackDamage.increased";
pub(crate) const ATTACK_SPEED: &str = "AttackSpeed";
pub(crate) const ATTACK_SPEED_BASE: &str = "AttackSpeed.base";
pub(crate) const ATTACK_SPEED_INCREASED: &str = "AttackSpeed.increased";
pub(crate) const ATTACK_RANGE: &str = "AttackRange";
pub(crate) const ATTACK_RANGE_BASE: &str = "AttackRange.base";
pub(crate) const ATTACK_RANGE_INCREASED: &str = "AttackRange.increased";
pub(crate) const PROJECTILE_SPEED: &str = "ProjectileSpeed";
pub(crate) const PROJECTILE_SPEED_BASE: &str = "ProjectileSpeed.base";
pub(crate) const PROJECTILE_SPEED_INCREASED: &str = "ProjectileSpeed.increased";
pub(crate) const CRITICAL_CHANCE: &str = "CriticalChance";
pub(crate) const MOVEMENT_SPEED: &str = "MovementSpeed";
pub(crate) const MOVEMENT_SPEED_BASE: &str = "MovementSpeed.base";
pub(crate) const MOVEMENT_SPEED_INCREASED: &str = "MovementSpeed.increased";

pub(crate) fn player_attributes() -> AttributeInitializer {
    attributes! {
        "Strength" => 0.0,
        "Dexterity" => 0.0,
        "Intelligence" => 0.0,
        "Vitality" => 0.0,
        "Health" => "100.0 + Vitality * 5.0",
        @complex "AttackDamage" => [
            ("base", ReduceFn::Sum),
            ("increased", ReduceFn::Sum),
        ] => "base * (1.0 + increased)",
        "AttackDamage.increased" => "Strength * 0.01",
        @complex "AttackSpeed" => [
            ("base", ReduceFn::Sum),
            ("increased", ReduceFn::Sum),
        ] => "base * (1.0 + increased)",
        @complex "AttackRange" => [
            ("base", ReduceFn::Sum),
            ("increased", ReduceFn::Sum),
        ] => "base * (1.0 + increased)",
        @complex "ProjectileSpeed" => [
            ("base", ReduceFn::Sum),
            ("increased", ReduceFn::Sum),
        ] => "base * (1.0 + increased)",
        "CriticalChance" => 0.0,
        @complex "MovementSpeed" => [
            ("base", ReduceFn::Sum),
            ("increased", ReduceFn::Sum),
        ] => "base * (1.0 + increased)",
        @complex "MagicDamage" => [
            ("base", ReduceFn::Sum),
            ("increased", ReduceFn::Sum),
        ] => "base * (1.0 + increased)",
        "MagicDamage.increased" => "Intelligence * 0.01",
    }
}
