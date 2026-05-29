use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems, components::Enemy, config::WAVE_DURATION, enemies::EnemySpawner,
    screens::Screen,
};

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct WaveState {
    pub current_wave: u32,
    pub timer: Timer,
}

impl Default for WaveState {
    fn default() -> Self {
        Self {
            current_wave: 1,
            timer: Timer::from_seconds(WAVE_DURATION, TimerMode::Repeating),
        }
    }
}

pub(super) struct WaveSystemsPlugin;

impl Plugin for WaveSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<WaveState>();
        app.add_systems(OnEnter(Screen::Gameplay), reset_wave_state);
        app.add_systems(OnExit(Screen::Gameplay), remove_wave_state);
        app.add_systems(
            Update,
            advance_wave_timer
                .in_set(PausableSystems)
                .in_set(AppSystems::WaveTransitions)
                .run_if(in_state(Screen::Gameplay)),
        );
    }
}

fn reset_wave_state(mut commands: Commands) {
    commands.insert_resource(WaveState::default());
}

fn remove_wave_state(mut commands: Commands) {
    commands.remove_resource::<WaveState>();
}

fn advance_wave_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut wave_state: ResMut<WaveState>,
    mut spawner: Option<ResMut<EnemySpawner>>,
    enemies: Query<Entity, With<Enemy>>,
) {
    wave_state.timer.tick(time.delta());

    if !wave_state.timer.just_finished() {
        return;
    }

    for enemy in enemies.iter() {
        commands.entity(enemy).despawn();
    }

    if let Some(ref mut spawner) = spawner {
        spawner.spawned_count = 0;
    }

    wave_state.current_wave += 1;
    println!("Wave {:?} started!", wave_state.current_wave)
}
