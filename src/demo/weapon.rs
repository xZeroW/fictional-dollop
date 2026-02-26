//! Weapon plugin and attachment to the player.

use bevy::prelude::*;

use bevy_asset_loader::prelude::*;

use crate::demo::{player::Player, cursor::CursorPosition};
use crate::{AppSystems, PausableSystems};
use crate::screens::Screen;

#[derive(AssetCollection, Resource)]
pub struct WeaponAssets {
    #[asset(key = "weapon.sprite")]
    pub sprite: Handle<Image>,
    #[asset(key = "weapon.fire_sound")]
    pub fire_sound: Handle<AudioSource>,
}

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Loading)
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                "rons/weapon.assets.ron",
            )
            .load_collection::<WeaponAssets>()
    );

    app.add_systems(Update, update_weapon_transform
        .in_set(AppSystems::Update)
        .in_set(PausableSystems)
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Weapon;

pub fn weapon(
    weapon_assets: &WeaponAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(15), 2, 1, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    (
        Name::new("Weapon"),
        Weapon,
        Sprite::from_atlas_image(
            weapon_assets.sprite.clone(),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 1,
            },
        ),
        Transform::from_translation(Vec3::new(12.0, 0.0, 1.0)).with_scale(Vec3::splat(1.0)),
    )
}
fn update_weapon_transform(
    cursor_pos: Res<CursorPosition>,
    player_query: Query<&GlobalTransform, With<Player>>,
    mut gun_query: Query<(&mut Transform, &mut Sprite), (With<Weapon>, Without<Player>)>,
) {
    if player_query.is_empty() || gun_query.is_empty() {
        return;
    }

    let player_gt = if let Ok(gt) = player_query.single() { gt } else { return };
    let player_pos = player_gt.translation().truncate();

    let cursor_world = cursor_pos.0.unwrap_or(player_pos);

    let (mut weapon_transform, mut weapon_sprite) = if let Ok(v) = gun_query.single_mut() { v } else { return };

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
