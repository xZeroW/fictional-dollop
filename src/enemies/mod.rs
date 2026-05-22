use bevy::prelude::*;

mod assets;
mod data;
mod systems;

pub use assets::EnemyVisualsHandle;
pub use data::Enemies;

#[derive(Resource)]
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
    pub fn init_weights(&mut self, enemies_data: &crate::enemies::Enemies) {
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

        self.enemy_keys.first().map(|(k, _)| k.as_str())
    }
}

#[derive(Resource)]
pub struct EnemiesDataHandle(pub Handle<Enemies>);

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        data::EnemyDataPlugin,
        assets::EnemyAssetsPlugin,
        systems::SystemsPlugin,
    ));
    app.add_systems(OnEnter(crate::screens::Screen::Loading), load_enemies_data);
    app.add_systems(
        OnEnter(crate::screens::Screen::Loading),
        load_enemy_visuals_data,
    );
}

fn load_enemies_data(mut commands: Commands, server: Res<AssetServer>) {
    let enemies_data_handle = server.load("data/enemies_data.ron");
    commands.insert_resource(EnemiesDataHandle(enemies_data_handle));
}

fn load_enemy_visuals_data(mut commands: Commands, server: Res<AssetServer>) {
    let enemy_visuals_handle = server.load("data/enemies.assets.ron");
    commands.insert_resource(EnemyVisualsHandle(enemy_visuals_handle));
}
