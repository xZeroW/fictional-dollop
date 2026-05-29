use bevy::prelude::*;

use super::enemy_spatial::EnemySpatialIndex;

use crate::{
    AppSystems, PausableSystems,
    assets::WeaponAssets,
    components::{Bullet, Player},
    game::weapon_data::{WeaponData, Weapons, WeaponsHandle},
    screens::Screen,
};

pub(super) struct AutoAttackSystemsPlugin;

impl Plugin for AutoAttackSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            auto_attack
                .in_set(AppSystems::SpatialQueries)
                .in_set(PausableSystems)
                .run_if(in_state(Screen::Gameplay)),
        );
    }
}

fn auto_attack(
    mut commands: Commands,
    mut player_query: Query<(&GlobalTransform, &mut Player)>,
    spatial_index: Res<EnemySpatialIndex>,
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
            warn!("Missing weapon data for '{}'", player.weapon);
            return;
        }
    };

    let current_time = time.elapsed_secs();
    if current_time - player.last_shot_time < weapon_data.cooldown {
        return;
    }

    let Some((_, enemy_pos)) = spatial_index.nearest_enemy(player_pos, player.attack_range) else {
        return;
    };

    let direction = enemy_pos - player_pos;
    if direction.length_squared() <= f32::EPSILON {
        return;
    }

    player.last_shot_time = current_time;

    commands.spawn(bullet(&weapon_assets, weapon_data, player_pos, direction));
}

fn bullet(
    weapon_assets: &WeaponAssets,
    weapon_data: &WeaponData,
    position: Vec2,
    direction: Vec2,
) -> impl Bundle {
    (
        Name::new("Bullet"),
        Bullet::new(direction, weapon_data.velocity, weapon_data.damage),
        DespawnOnExit(Screen::Gameplay),
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
