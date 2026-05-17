//! Weapon plugin and attachment to the player.

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    assets::WeaponAssets,
    components::Player,
    game::{
        PlayerAction,
        weapon_data::{Weapons, WeaponsHandle},
    },
    libs::cursor::CursorPosition,
};

const BULLET_LIFETIME: f32 = 2.0;

#[derive(Resource, Default)]
pub struct AvailableWeapons {
    pub weapons: Vec<String>,
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
    pub fn new(direction: Vec2, velocity: f32) -> Self {
        Self {
            velocity: direction.normalize() * velocity,
            lifetime: Timer::from_seconds(BULLET_LIFETIME, TimerMode::Once),
        }
    }
}

pub fn weapon(
    weapon_assets: &WeaponAssets,
    weapon_data: &crate::game::weapon_data::WeaponData,
) -> impl Bundle {
    (
        Name::new("Weapon"),
        Weapon,
        Sprite::from_atlas_image(
            weapon_assets.sprite.clone(),
            TextureAtlas {
                layout: weapon_assets.layout.clone(),
                index: weapon_data.weapon_sprite_index,
            },
        ),
        Transform::from_translation(Vec3::new(12.0, 0.0, 1.0)).with_scale(Vec3::splat(1.0)),
    )
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            update_weapon_transform,
            switch_weapon,
            update_weapon_switch_timer,
            spawn_bullet,
            move_bullet,
            despawn_bullet,
        )
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );

    app.init_resource::<AvailableWeapons>();
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

fn switch_weapon(
    mut commands: Commands,
    action_state: Single<&ActionState<PlayerAction>>,
    mut player_query: Query<(Entity, &mut Player)>,
    available_weapons: Res<AvailableWeapons>,
    weapons_handle: Res<WeaponsHandle>,
    weapons_assets: Res<Assets<Weapons>>,
    weapon_assets: Res<WeaponAssets>,
) {
    if !action_state.just_pressed(&PlayerAction::SwitchWeapon) {
        return;
    }

    if available_weapons.weapons.is_empty() {
        return;
    }

    let (player_entity, mut player) = match player_query.single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };

    if player.switching_weapon {
        return;
    }

    player.switching_weapon = true;

    let current_index = available_weapons
        .weapons
        .iter()
        .position(|w| w == &player.weapon)
        .unwrap_or(0);
    let next_index = (current_index + 1) % available_weapons.weapons.len();
    let new_weapon = &available_weapons.weapons[next_index];

    let weapons = match weapons_assets.get(&weapons_handle.0) {
        Some(w) => w,
        None => return,
    };

    let weapon_data = match weapons.0.get(new_weapon) {
        Some(data) => data,
        None => return,
    };

    if let Some(old_weapon) = player.weapon_entity {
        if commands.get_entity(old_weapon).is_ok() {
            commands.entity(old_weapon).despawn();
        }
    }

    let new_weapon_entity = commands.spawn(weapon(&weapon_assets, weapon_data)).id();

    commands.entity(player_entity).add_child(new_weapon_entity);

    let mut new_player = player.clone();
    new_player.weapon = new_weapon.to_string();
    new_player.weapon_entity = Some(new_weapon_entity);
    new_player.switching_weapon = true;
    new_player.switch_timer = Timer::from_seconds(3.0, TimerMode::Once);
    new_player.can_shoot_timer = Timer::from_seconds(0.2, TimerMode::Once);
    commands.entity(player_entity).insert(new_player);
}

fn update_weapon_switch_timer(mut player_query: Query<&mut Player>, time: Res<Time>) {
    for mut player in &mut player_query {
        if player.switching_weapon {
            player.switch_timer.tick(time.delta());

            if player.switch_timer.just_finished() {
                player.switching_weapon = false;
                player.switch_timer.reset();
            }
        }

        if player.can_shoot_timer.elapsed_secs() < 0.2 {
            player.can_shoot_timer.tick(time.delta());
        }
    }
}

pub fn bullet(
    weapon_assets: &WeaponAssets,
    weapon_data: &crate::game::weapon_data::WeaponData,
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
                index: weapon_data.bullet_sprite_index,
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
    player_query: Query<(Entity, &GlobalTransform, &Player)>,
    weapon_query: Query<&GlobalTransform, With<Weapon>>,
    time: Res<Time>,
    weapon_assets: Res<WeaponAssets>,
    weapons_handle: Res<WeaponsHandle>,
    weapons_assets: Res<Assets<Weapons>>,
) {
    if !action_state.pressed(&PlayerAction::Attack) {
        return;
    }

    let (player_entity, player_gt, player) = match player_query.single() {
        Ok(v) => v,
        Err(_) => return,
    };

    if player.can_shoot_timer.elapsed_secs() < 0.2 {
        return;
    }

    let player_pos = player_gt.translation().truncate();
    let player_weapon = player.weapon.clone();
    let last_shot_time = player.last_shot_time;

    if player_weapon.is_empty() {
        return;
    }

    let weapon_gt = match weapon_query.single() {
        Ok(gt) => gt,
        Err(_) => return,
    };
    let weapon_pos = weapon_gt.translation().truncate();

    let weapons = match weapons_assets.get(&weapons_handle.0) {
        Some(w) => w,
        None => return,
    };

    let weapon_data = match weapons.0.get(&player_weapon) {
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
    if current_time - last_shot_time < weapon_data.cooldown {
        return;
    }

    let new_player = Player {
        weapon: player_weapon,
        weapon_entity: player.weapon_entity,
        last_shot_time: current_time,
        switching_weapon: player.switching_weapon,
        switch_timer: player.switch_timer.clone(),
        can_shoot_timer: player.can_shoot_timer.clone(),
    };
    commands.entity(player_entity).insert(new_player);

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
