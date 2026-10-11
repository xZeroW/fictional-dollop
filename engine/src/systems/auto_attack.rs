use bevy::prelude::*;

use super::enemy_spatial::EnemySpatialIndex;

use crate::{
    AppSystems, PausableSystems,
    assets::WeaponAssets,
    components::{Bullet, Player, Weapon},
    game::{
        attributes::{
            ATTACK_DAMAGE, ATTACK_DAMAGE_BASE, ATTACK_RANGE, ATTACK_RANGE_BASE, ATTACK_SPEED,
            ATTACK_SPEED_BASE, CRITICAL_CHANCE, PROJECTILE_SPEED, PROJECTILE_SPEED_BASE,
        },
        weapon_data::{WeaponData, Weapons, WeaponsHandle},
    },
    screens::Screen,
};
use bevy_gauge::prelude::*;

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
    mut attributes: AttributesMut,
    mut player_query: Query<(Entity, &Transform, &mut Weapon), With<Player>>,
    spatial_index: Res<EnemySpatialIndex>,
    time: Res<Time>,
    weapon_assets: Res<WeaponAssets>,
    weapons_handle: Res<WeaponsHandle>,
    weapons_assets: Res<Assets<Weapons>>,
) {
    let (player, player_gt, mut weapon) = match player_query.single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };

    let player_pos = player_gt.translation.truncate();

    let weapons = match weapons_assets.get(&weapons_handle.0) {
        Some(w) => w,
        None => return,
    };

    let weapon_data = match weapons.0.get(&weapon.key) {
        Some(data) => data,
        None => {
            warn!("Missing weapon data for '{}'", weapon.key);
            return;
        }
    };

    attributes.set_base(player, ATTACK_SPEED_BASE, weapon_data.attack_speed);
    let attack_speed = attributes.evaluate(player, ATTACK_SPEED);
    weapon.set_attack_speed(attack_speed);

    weapon.attack_timer.tick(time.delta());
    if !weapon.attack_timer.is_finished() {
        return;
    }

    attributes.set_base(player, ATTACK_RANGE_BASE, weapon_data.attack_range);
    let attack_range = attributes.evaluate(player, ATTACK_RANGE);
    let Some((_, enemy_pos)) = spatial_index.nearest_enemy(player_pos, attack_range) else {
        return;
    };

    let direction = enemy_pos - player_pos;
    if direction.length_squared() <= f32::EPSILON {
        return;
    }

    weapon.attack_timer.reset();

    attributes.set_base(player, ATTACK_DAMAGE_BASE, weapon_data.damage);
    let mut damage = attributes.evaluate(player, ATTACK_DAMAGE);
    let critical_chance = attributes.evaluate(player, CRITICAL_CHANCE).clamp(0.0, 1.0);
    if rand::random::<f32>() < critical_chance {
        damage *= 2.0;
    }

    attributes.set_base(player, PROJECTILE_SPEED_BASE, weapon_data.velocity);
    let projectile_speed = attributes.evaluate(player, PROJECTILE_SPEED);

    commands.spawn(bullet(
        &weapon_assets,
        weapon_data,
        player_pos,
        direction,
        projectile_speed,
        damage,
    ));
}

fn bullet(
    weapon_assets: &WeaponAssets,
    weapon_data: &WeaponData,
    position: Vec2,
    direction: Vec2,
    velocity: f32,
    damage: f32,
) -> impl Bundle {
    (
        Name::new("Bullet"),
        Bullet::new(direction, velocity, damage),
        DespawnOnExit(Screen::Gameplay),
        Sprite::from_atlas_image(
            weapon_assets.bullet_sprite.clone(),
            TextureAtlas {
                layout: weapon_assets.bullet_layout.clone(),
                index: weapon_data.bullet_sprite_index,
            },
        ),
        Transform::from_translation(position.extend(10.0)).with_scale(Vec3::splat(2.0)),
    )
}
