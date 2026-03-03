//! Weapon plugin and attachment to the player.

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    assets::WeaponAssets,
    demo::{
        PlayerAction,
        cursor::CursorPosition,
        player::Player,
        weapon_data::{Weapons, WeaponsHandle},
    },
};

const BULLET_LIFETIME: f32 = 2.0;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Weapon;

#[derive(Component, Debug, Clone, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct Bullet {
    pub velocity: Vec2,
    pub lifetime: Timer,
}

impl Bullet {
    pub fn new(direction: Vec2, velocity: f32) -> Self {
        Self {
            velocity: direction.normalize() * velocity,
            lifetime: Timer::from_seconds(BULLET_LIFETIME, TimerMode::Once),
        }
    }
}

#[derive(Resource, Default)]
pub struct LastShotTime {
    pub time: f32,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            update_weapon_transform,
            spawn_bullet,
            move_bullet,
            despawn_bullet,
        )
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );

    app.insert_resource(LastShotTime::default());
}

pub fn weapon(weapon_assets: &WeaponAssets) -> impl Bundle {
    (
        Name::new("Weapon"),
        Weapon,
        Sprite::from_atlas_image(
            weapon_assets.sprite.clone(),
            TextureAtlas {
                layout: weapon_assets.layout.clone(),
                index: 1,
            },
        ),
        Transform::from_translation(Vec3::new(12.0, 0.0, 1.0)).with_scale(Vec3::splat(1.0)),
    )
}

fn update_weapon_transform(
    cursor_pos: Res<CursorPosition>,
    player_query: Query<&GlobalTransform, With<Player>>,
    mut weapon_query: Query<(&mut Transform, &mut Sprite), (With<Weapon>, Without<Player>)>,
) {
    if player_query.is_empty() || weapon_query.is_empty() {
        return;
    }

    let player_gt = if let Ok(gt) = player_query.single() {
        gt
    } else {
        return;
    };
    let player_pos = player_gt.translation().truncate();

    let cursor_world = cursor_pos.0.unwrap_or(player_pos);

    let (mut weapon_transform, mut weapon_sprite) = if let Ok(v) = weapon_query.single_mut() {
        v
    } else {
        return;
    };

    let dir = cursor_world - player_pos;
    if dir.length_squared() <= f32::EPSILON {
        return;
    }

    let angle = dir.y.atan2(dir.x);
    weapon_transform.rotation = Quat::from_rotation_z(angle);

    const OFFSET: f32 = 20.0;
    const WEAPON_Z: f32 = 15.0;

    let local_x = OFFSET * angle.cos();
    let local_y = OFFSET * angle.sin();
    weapon_transform.translation = Vec3::new(local_x, local_y, WEAPON_Z);

    weapon_sprite.flip_y = local_x < 0.0;
}

pub fn bullet(
    weapon_assets: &WeaponAssets,
    weapon_data: &crate::demo::weapon_data::WeaponData,
    position: Vec2,
    direction: Vec2,
) -> impl Bundle {
    (
        Name::new("Bullet"),
        Bullet::new(direction, weapon_data.velocity),
        Sprite::from_atlas_image(
            weapon_assets.sprite.clone(),
            TextureAtlas {
                layout: weapon_assets.layout.clone(),
                index: weapon_data.sprite_index,
            },
        ),
        Transform::from_translation(position.extend(10.0))
            .with_scale(Vec3::splat(weapon_data.scale)),
    )
}

fn spawn_bullet(
    mut commands: Commands,
    action_state: Single<&ActionState<PlayerAction>>,
    cursor_pos: Res<CursorPosition>,
    player_query: Query<(&GlobalTransform, &Player)>,
    weapon_query: Query<&GlobalTransform, With<Weapon>>,
    mut last_shot: ResMut<LastShotTime>,
    time: Res<Time>,
    weapon_assets: Res<WeaponAssets>,
    weapons_handle: Res<WeaponsHandle>,
    weapons_assets: Res<Assets<Weapons>>,
) {
    if !action_state.pressed(&PlayerAction::Attack) {
        return;
    }

    let (player_gt, player) = match player_query.single() {
        Ok(v) => v,
        Err(_) => return,
    };
    let player_pos = player_gt.translation().truncate();

    let weapon_gt = match weapon_query.single() {
        Ok(gt) => gt,
        Err(_) => return,
    };
    let weapon_pos = weapon_gt.translation().truncate();

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
    if current_time - last_shot.time < weapon_data.cooldown {
        return;
    }
    last_shot.time = current_time;

    let cursor_world = cursor_pos.0.unwrap_or(player_pos);
    let direction = cursor_world - weapon_pos;

    if direction.length_squared() <= f32::EPSILON {
        return;
    }

    commands.spawn(bullet(&weapon_assets, weapon_data, weapon_pos, direction));
}

fn move_bullet(mut bullet_query: Query<(&mut Transform, &mut Bullet)>, time: Res<Time>) {
    for (mut transform, mut bullet) in &mut bullet_query {
        bullet.lifetime.tick(time.delta());
        let velocity = bullet.velocity * time.delta_secs();
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}

fn despawn_bullet(mut commands: Commands, bullet_query: Query<(Entity, &Bullet)>) {
    for (entity, bullet) in bullet_query.iter() {
        if bullet.lifetime.elapsed() >= bullet.lifetime.duration() {
            commands.entity(entity).despawn();
        }
    }
}
