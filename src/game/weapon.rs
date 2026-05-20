//! Auto-attack system - shoots nearest enemy within range automatically.

use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    assets::WeaponAssets,
    components::{Enemy, Player},
    game::weapon_data::{Weapons, WeaponsHandle},
    screens::Screen,
};

const BULLET_LIFETIME: f32 = 2.0;

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

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            auto_attack.run_if(in_state(Screen::Gameplay)),
            move_bullet.run_if(in_state(Screen::Gameplay)),
            despawn_bullet.run_if(in_state(Screen::Gameplay)),
        )
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

fn auto_attack(
    mut commands: Commands,
    player_query: Query<(Entity, &GlobalTransform, &Player)>,
    enemy_query: Query<(Entity, &GlobalTransform), With<Enemy>>,
    time: Res<Time>,
    weapon_assets: Res<WeaponAssets>,
    weapons_handle: Res<WeaponsHandle>,
    weapons_assets: Res<Assets<Weapons>>,
) {
    let (player_entity, player_gt, player) = match player_query.single() {
        Ok(v) => v,
        Err(_) => return,
    };

    let player_pos = player_gt.translation().truncate();
    let player_weapon = player.weapon.clone();

    if player_weapon.is_empty() {
        return;
    }

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

    let new_player = Player {
        weapon: player_weapon,
        attack_range: player.attack_range,
        last_shot_time: current_time,
    };
    commands.entity(player_entity).insert(new_player);

    commands.spawn(bullet(&weapon_assets, weapon_data, player_pos, direction));
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
