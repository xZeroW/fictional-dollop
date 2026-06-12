use bevy::prelude::*;

use super::enemy_spatial::EnemySpatialIndex;

use crate::{
    AppSystems, PausableSystems,
    assets::WeaponAssets,
    components::{Bullet, Player, Weapon},
    game::{
        attributes::{ATTACK_DAMAGE, ATTACK_DAMAGE_BASE},
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

    weapon.set_attack_speed(weapon_data.attack_speed);
    weapon.attack_timer.tick(time.delta());
    if !weapon.attack_timer.is_finished() {
        return;
    }

    let Some((_, enemy_pos)) = spatial_index.nearest_enemy(player_pos, weapon_data.attack_range)
    else {
        return;
    };

    let direction = enemy_pos - player_pos;
    if direction.length_squared() <= f32::EPSILON {
        return;
    }

    weapon.attack_timer.reset();

    attributes.set_base(player, ATTACK_DAMAGE_BASE, weapon_data.damage);
    let damage = attributes.evaluate(player, ATTACK_DAMAGE);

    commands.spawn(bullet(
        &weapon_assets,
        weapon_data,
        player_pos,
        direction,
        damage,
    ));
}

fn bullet(
    weapon_assets: &WeaponAssets,
    weapon_data: &WeaponData,
    position: Vec2,
    direction: Vec2,
    damage: f32,
) -> impl Bundle {
    (
        Name::new("Bullet"),
        Bullet::new(direction, weapon_data.velocity, damage),
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
