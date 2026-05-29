//! Weapon and bullet entity definitions.

use bevy::prelude::*;

use crate::{assets::WeaponAssets, game::weapon_data::WeaponData};

const BULLET_LIFETIME: f32 = 2.0;

#[derive(Component, Debug, Clone, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct Bullet {
    pub velocity: Vec2,
    pub lifetime: Timer,
}

impl Bullet {
    pub fn new(direction: Vec2, velocity: f32) -> Self {
        Self {
            velocity: direction.normalize() * velocity,
            lifetime: Timer::from_seconds(BULLET_LIFETIME, TimerMode::Once),
        }
    }
}

pub(crate) fn bullet(
    weapon_assets: &WeaponAssets,
    weapon_data: &WeaponData,
    position: Vec2,
    direction: Vec2,
) -> impl Bundle {
    (
        Name::new("Bullet"),
        Bullet::new(direction, weapon_data.velocity),
        Sprite::from_atlas_image(
            weapon_assets.sprite.clone(),
            TextureAtlas {
                layout: weapon_assets.layout.clone(),
                index: weapon_data.bullet_sprite_index,
            },
        ),
        Transform::from_translation(position.extend(10.0))
            .with_scale(Vec3::splat(weapon_data.scale)),
    )
}
