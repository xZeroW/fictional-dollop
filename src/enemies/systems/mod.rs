mod behavior;
mod hit_flash;
mod nameplate;
mod spawn;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;

use crate::components::AttackCooldown;
use crate::config;
use crate::{AppSystems, PausableSystems, screens::Screen};

pub use behavior::behavior;
pub use hit_flash::HitFlash;
pub use nameplate::{AttackTimeNameplate, HealthNameplate};

fn tick_attack_cooldowns(mut query: Query<&mut AttackCooldown>, time: Res<Time>) {
    for mut cooldown in query.iter_mut() {
        cooldown.timer.tick(time.delta());
    }
}

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SpawnPlugin, HitFlashPlugin));
        app.add_systems(
            Update,
            (
                behavior,
                tick_attack_cooldowns,
                nameplate::update_attack_time_nameplates,
                nameplate::update_health_nameplates,
            )
                .in_set(PausableSystems)
                .in_set(AppSystems::Update)
                .run_if(in_state(Screen::Gameplay)),
        );
        app.add_systems(
            Update,
            nameplate::update_nameplate_visibility
                .in_set(AppSystems::Update)
                .run_if(in_state(Screen::Gameplay))
                .run_if(resource_changed::<crate::config::GameSettings>),
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

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
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
                .run_if(resource_exists::<crate::enemies::EnemiesDataHandle>)
                .run_if(resource_exists::<crate::enemies::EnemySpawner>),
        );
    }
}

pub struct HitFlashPlugin;

impl Plugin for HitFlashPlugin {
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
