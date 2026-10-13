//! Enemy data loaded from .ron files.

use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use std::collections::HashMap;

use crate::domain::progression::MonsterProgression;
use crate::domain::{
    combat::{AttackTimer, Damage},
    entities::{Behavior, Enemy, Health, Movement, WanderState},
};

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct Enemies(pub HashMap<String, EnemyData>);

#[derive(Resource)]
pub struct EnemiesDataHandle(pub Handle<Enemies>);

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct EnemySpawner {
    pub spawned_count: usize,
    pub enemy_keys: Vec<(String, f32)>,
    total_spawn_weight: f32,
}

impl Default for EnemySpawner {
    fn default() -> Self {
        Self {
            spawned_count: 0,
            enemy_keys: vec![],
            total_spawn_weight: 0.0,
        }
    }
}

impl EnemySpawner {
    pub fn init_weights(&mut self, enemies_data: &Enemies) {
        self.enemy_keys.clear();
        self.total_spawn_weight = 0.0;
        for (key, data) in &enemies_data.0 {
            self.enemy_keys.push((key.clone(), data.spawn_rate));
            self.total_spawn_weight += data.spawn_rate;
        }
    }

    pub fn select_enemy_key(&self) -> Option<&str> {
        if self.enemy_keys.is_empty() || self.total_spawn_weight <= 0.0 {
            return None;
        }

        let pick = rand::random::<f32>() * self.total_spawn_weight;
        let mut accum = 0.0;

        for (key, weight) in &self.enemy_keys {
            accum += *weight;
            if pick <= accum {
                return Some(key.as_str());
            }
        }

        self.enemy_keys.first().map(|(key, _)| key.as_str())
    }
}

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
        progression: &MonsterProgression,
    ) -> impl Bundle {
        let behavior = match self.behavior.as_str() {
            "Wandering" => Behavior::Wandering,
            "FollowAndAttack" => Behavior::FollowAndAttack,
            "Coward" => Behavior::Coward,
            _ => Behavior::Wandering,
        };
        let health = self.health * progression.enemy_health_mult;
        let damage = self.damage * progression.enemy_damage_mult;
        let speed = self.speed * progression.enemy_speed_mult;

        (
            Name::new(self.name.clone()),
            Enemy::new(key.to_string()),
            Health {
                max: health,
                current: health,
            },
            Movement::new(speed),
            WanderState::default(),
            Damage::new(damage),
            AttackTimer::new(self.attack_speed),
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
