mod behavior;
mod spawn;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;

use crate::components::AttackCooldown;
use crate::game::config;
use crate::{AppSystems, PausableSystems, screens::Screen};

pub use behavior::behavior;

fn tick_attack_cooldowns(mut query: Query<&mut AttackCooldown>, time: Res<Time>) {
    for mut cooldown in query.iter_mut() {
        cooldown.timer.tick(time.delta());
    }
}

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SpawnPlugin);
        app.add_systems(
            Update,
            (behavior, tick_attack_cooldowns)
                .in_set(PausableSystems)
                .in_set(AppSystems::Update)
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
