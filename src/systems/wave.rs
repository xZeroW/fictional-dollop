use bevy::prelude::*;

use super::enemy_spatial::EnemySpatialIndex;

use crate::{
    AppSystems, PausableSystems, Pause,
    components::{Bullet, Enemy},
    config::WAVE_DURATION,
    enemies::EnemySpawner,
    menus::Menu,
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
        app.add_systems(OnEnter(Menu::MonsterBuff), cleanup_wave_entities);
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
    time: Res<Time>,
    mut wave_state: ResMut<WaveState>,
    mut next_menu: ResMut<NextState<Menu>>,
    mut next_pause: ResMut<NextState<Pause>>,
) {
    wave_state.timer.tick(time.delta());

    if !wave_state.timer.just_finished() {
        return;
    }

    wave_state.current_wave += 1;
    next_pause.set(Pause(true));
    next_menu.set(Menu::MonsterBuff);
    info!(
        "Wave {:?} ready after monster evolution.",
        wave_state.current_wave
    );
}

fn cleanup_wave_entities(
    mut commands: Commands,
    enemies: Query<Entity, With<Enemy>>,
    bullets: Query<Entity, With<Bullet>>,
    mut spawner: Option<ResMut<EnemySpawner>>,
    mut spatial_index: ResMut<EnemySpatialIndex>,
) {
    for enemy in enemies.iter() {
        commands.entity(enemy).despawn();
    }
    for bullet in bullets.iter() {
        commands.entity(bullet).despawn();
    }

    if let Some(ref mut spawner) = spawner {
        spawner.spawned_count = 0;
    }
    spatial_index.clear();
}
