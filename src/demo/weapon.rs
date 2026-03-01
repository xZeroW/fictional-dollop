//! Weapon plugin and attachment to the player.

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use bevy_asset_loader::prelude::*;

use crate::demo::{cursor::CursorPosition, player::Player, PlayerAction};
use crate::screens::Screen;
use crate::{AppSystems, PausableSystems};

#[derive(AssetCollection, Resource)]
pub struct WeaponAssets {
    #[asset(key = "weapon.sprite")]
    pub sprite: Handle<Image>,
    #[asset(key = "weapon.layout")]
    pub layout: Handle<TextureAtlasLayout>,
    #[asset(key = "weapon.fire_sound")]
    pub fire_sound: Handle<AudioSource>,
}

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
    pub fn new(direction: Vec2) -> Self {
        Self {
            velocity: direction.normalize() * 600.0,
            lifetime: Timer::from_seconds(2.0, TimerMode::Once),
        }
    }
}

#[derive(Resource)]
pub struct BulletSpawnTimer {
    pub timer: Timer,
}

impl Default for BulletSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.2, TimerMode::Once),
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Loading)
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>("rons/weapon.assets.ron")
            .load_collection::<WeaponAssets>(),
    );

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

    app.insert_resource(BulletSpawnTimer::default());
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

pub fn bullet(weapon_assets: &WeaponAssets, position: Vec2, direction: Vec2) -> impl Bundle {
    (
        Name::new("Bullet"),
        Bullet::new(direction),
        Sprite::from_atlas_image(
            weapon_assets.sprite.clone(),
            TextureAtlas {
                layout: weapon_assets.layout.clone(),
                index: 0,
            },
        ),
        Transform::from_translation(position.extend(10.0)).with_scale(Vec3::splat(1.5)),
    )
}

fn spawn_bullet(
    mut commands: Commands,
    action_state: Single<&ActionState<PlayerAction>>,
    cursor_pos: Res<CursorPosition>,
    player_query: Query<&GlobalTransform, With<Player>>,
    weapon_query: Query<&GlobalTransform, With<Weapon>>,
    mut spawn_timer: ResMut<BulletSpawnTimer>,
    time: Res<Time>,
    weapon_assets: Res<WeaponAssets>,
) {
    spawn_timer.timer.tick(time.delta());

    if !action_state.pressed(&PlayerAction::Attack) {
        return;
    }

    if spawn_timer.timer.elapsed() < spawn_timer.timer.duration() {
        return;
    }

    spawn_timer.timer.reset();

    let player_gt = match player_query.single() {
        Ok(gt) => gt,
        Err(_) => return,
    };
    let player_pos = player_gt.translation().truncate();

    let weapon_gt = match weapon_query.single() {
        Ok(gt) => gt,
        Err(_) => return,
    };
    let weapon_pos = weapon_gt.translation().truncate();

    let cursor_world = cursor_pos.0.unwrap_or(player_pos);
    let direction = cursor_world - weapon_pos;

    if direction.length_squared() <= f32::EPSILON {
        return;
    }

    commands.spawn(bullet(&weapon_assets, weapon_pos, direction));
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
