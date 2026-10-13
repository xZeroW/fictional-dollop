use crate::Screen;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_auto_plugin::prelude::*;
use std::time::Duration;

use crate::domain::combat::AttackTimer;
use crate::infrastructure::config;
use crate::{AppSystems, PausableSystems};

use super::data::{EnemiesDataHandle, EnemySpawner};

pub use behavior::behavior;
pub use hit_flash::HitFlash;

mod behavior;
mod hit_flash;
mod nameplate;
mod spawn;

fn tick_attack_cooldowns(mut query: Query<&mut AttackTimer>, time: Res<Time>) {
    for mut cooldown in query.iter_mut() {
        cooldown.timer.tick(time.delta());
    }
}

#[derive(AutoPlugin)]
pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    #[auto_plugin]
    fn build(&self, app: &mut App) {
        app.add_plugins((SpawnPlugin, HitFlashPlugin, nameplate::NameplatePlugin));
        app.add_systems(
            Update,
            (behavior, tick_attack_cooldowns)
                .in_set(PausableSystems)
                .in_set(AppSystems::Update)
                .run_if(in_state(Screen::Gameplay)),
        );
        app.add_systems(
            Update,
            behavior::prevent_enemy_player_overlap
                .in_set(PausableSystems)
                .in_set(AppSystems::ResolveContacts)
                .run_if(in_state(Screen::Gameplay)),
        );
    }
}

#[derive(AutoPlugin)]
pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    #[auto_plugin]
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn::spawn_enemies
                .in_set(PausableSystems)
                .in_set(AppSystems::Update)
                .run_if(in_state(Screen::Gameplay))
                .run_if(on_timer(Duration::from_secs_f32(
                    config::ENEMY_SPAWN_INTERVAL,
                )))
                .run_if(resource_exists::<EnemiesDataHandle>)
                .run_if(resource_exists::<EnemySpawner>),
        );
    }
}

#[derive(AutoPlugin)]
pub struct HitFlashPlugin;

impl Plugin for HitFlashPlugin {
    #[auto_plugin]
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            hit_flash::update_hit_flash
                .in_set(PausableSystems)
                .in_set(AppSystems::Update)
                .run_if(in_state(Screen::Gameplay)),
        );
        app.add_systems(
            Update,
            hit_flash::tick_hit_flash
                .in_set(PausableSystems)
                .in_set(AppSystems::TickTimers)
                .run_if(in_state(Screen::Gameplay)),
        );
    }
}
