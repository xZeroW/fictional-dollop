//! Enemy data loaded from .ron files.

use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use std::collections::HashMap;

use crate::components::{AttackCooldown, Behavior, Damage, Enemy, Health, Movement, WanderState};

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct Enemies(pub HashMap<String, EnemyData>);

#[derive(Debug, Clone, serde::Deserialize)]
pub struct EnemyData {
    pub name: String,
    pub asset_key: String,
    pub behavior: String,
    pub health: f32,
    pub damage: f32,
    pub speed: f32,
    pub scale: f32,
    pub sprite_index: usize,
    pub spawn_rate: f32,
    pub attack_speed: f32,
}

pub(super) struct EnemyDataPlugin;

impl Plugin for EnemyDataPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<Enemies>::new(&["enemies_data.ron"]));
    }
}

impl EnemyData {
    pub fn bundle(
        &self,
        key: &str,
        position: Vec3,
        image: Handle<Image>,
        layout: Handle<TextureAtlasLayout>,
    ) -> impl Bundle {
        let behavior = match self.behavior.as_str() {
            "Wandering" => Behavior::Wandering,
            "FollowAndAttack" => Behavior::FollowAndAttack,
            "Coward" => Behavior::Coward,
            _ => Behavior::Wandering,
        };

        (
            Name::new(self.name.clone()),
            Enemy::new(key.to_string()),
            Health {
                max: self.health,
                current: self.health,
            },
            Movement::new(self.speed),
            WanderState::default(),
            Damage::new(self.damage),
            AttackCooldown::new(self.attack_speed),
            behavior,
            Sprite::from_atlas_image(
                image,
                TextureAtlas {
                    layout,
                    index: self.sprite_index,
                },
            ),
            Transform::from_translation(position).with_scale(Vec3::splat(self.scale)),
        )
    }
}
