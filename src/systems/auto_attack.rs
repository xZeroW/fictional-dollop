use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    assets::WeaponAssets,
    components::{Enemy, Player},
    game::{
        weapon::bullet,
        weapon_data::{Weapons, WeaponsHandle},
    },
    screens::Screen,
};

pub(super) struct AutoAttackSystemsPlugin;

impl Plugin for AutoAttackSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            auto_attack
                .in_set(AppSystems::Update)
                .in_set(PausableSystems)
                .run_if(in_state(Screen::Gameplay)),
        );
    }
}

fn auto_attack(
    mut commands: Commands,
    mut player_query: Query<(&GlobalTransform, &mut Player)>,
    enemy_query: Query<(Entity, &GlobalTransform), With<Enemy>>,
    time: Res<Time>,
    weapon_assets: Res<WeaponAssets>,
    weapons_handle: Res<WeaponsHandle>,
    weapons_assets: Res<Assets<Weapons>>,
) {
    let (player_gt, mut player) = match player_query.single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };

    let player_pos = player_gt.translation().truncate();

    if player.weapon.is_empty() {
        return;
    }

    let weapons = match weapons_assets.get(&weapons_handle.0) {
        Some(w) => w,
        None => return,
    };

    let weapon_data = match weapons.0.get(&player.weapon) {
        Some(data) => data,
        None => {
            let default = weapons.0.get("dagger");
            match default {
                Some(d) => d,
                None => return,
            }
        }
    };

    let current_time = time.elapsed_secs();
    if current_time - player.last_shot_time < weapon_data.cooldown {
        return;
    }

    let mut nearest_enemy: Option<(Entity, Vec2)> = None;
    let mut nearest_distance_sq = player.attack_range * player.attack_range;

    for (enemy_entity, enemy_gt) in enemy_query.iter() {
        let enemy_pos = enemy_gt.translation().truncate();
        let dist_sq = player_pos.distance_squared(enemy_pos);

        if dist_sq <= nearest_distance_sq {
            nearest_distance_sq = dist_sq;
            nearest_enemy = Some((enemy_entity, enemy_pos));
        }
    }

    let Some((_, enemy_pos)) = nearest_enemy else {
        return;
    };

    let direction = enemy_pos - player_pos;
    if direction.length_squared() <= f32::EPSILON {
        return;
    }

    player.last_shot_time = current_time;

    commands.spawn(bullet(&weapon_assets, weapon_data, player_pos, direction));
}
