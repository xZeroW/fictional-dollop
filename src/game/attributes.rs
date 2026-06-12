//! Gameplay attributes and derived stats.

use bevy_gauge::prelude::*;

pub(crate) const STRENGTH: &str = "Strength";
pub(crate) const DEXTERITY: &str = "Dexterity";
pub(crate) const INTELLIGENCE: &str = "Intelligence";
pub(crate) const VITALITY: &str = "Vitality";
pub(crate) const ATTACK_DAMAGE: &str = "AttackDamage";
pub(crate) const ATTACK_DAMAGE_BASE: &str = "AttackDamage.base";

pub(crate) fn player_attributes() -> AttributeInitializer {
    attributes! {
        "Strength" => 0.0,
        "Dexterity" => 0.0,
        "Intelligence" => 0.0,
        "Vitality" => 0.0,
        "Health" => "100.0 + Vitality * 10.0",
        @complex "AttackDamage" => [
            ("base", ReduceFn::Sum),
            ("increased", ReduceFn::Sum),
        ] => "base * (1.0 + increased)",
        "AttackDamage.increased" => "Strength * 0.01",
        @complex "MagicDamage" => [
            ("base", ReduceFn::Sum),
            ("increased", ReduceFn::Sum),
        ] => "base * (1.0 + increased)",
        "MagicDamage.increased" => "Intelligence * 0.01",
    }
}
