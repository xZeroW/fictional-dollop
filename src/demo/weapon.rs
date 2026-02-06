//! Weapon plugin and attachment to the player.

use bevy::prelude::*;
use crate::asset_tracking::LoadResource;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<WeaponAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct WeaponAssets {
    #[dependency]
    pub sprite: Handle<Image>,
    #[dependency]
    pub fire_sound: Handle<AudioSource>,
}

impl FromWorld for WeaponAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            sprite: assets.load("weapons/dagger.png"),
            fire_sound: assets.load("audio/sound_effects/step1.ogg"),
        }
    }
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

