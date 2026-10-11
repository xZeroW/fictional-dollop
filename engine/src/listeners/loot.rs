use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    assets::WeaponAssets,
    components::{ItemDrop, ItemRarity, PickupRadius},
    game::{
        level::LevelEntity,
        weapon_data::{WeaponData, Weapons, WeaponsHandle},
    },
    messages::EntityDiedMessage,
    screens::Screen,
    systems::MonsterProgression,
};

const BASE_DROP_CHANCE: f32 = 0.1;
const MAX_DROP_CHANCE: f32 = 0.85;
const DROP_SCALE: f32 = 2.0;

pub struct LootListenerPlugin;

impl Plugin for LootListenerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_loot_from_enemy_deaths
                .in_set(AppSystems::DeathEvents)
                .in_set(PausableSystems)
                .run_if(in_state(Screen::Gameplay)),
        );
    }
}

fn spawn_loot_from_enemy_deaths(
    mut commands: Commands,
    mut deaths: MessageReader<EntityDiedMessage>,
    progression: Res<MonsterProgression>,
    level_entity: Res<LevelEntity>,
    weapon_assets: Res<WeaponAssets>,
    weapons_handle: Res<WeaponsHandle>,
    weapons_assets: Res<Assets<Weapons>>,
) {
    let Some(weapons) = weapons_assets.get(&weapons_handle.0) else {
        return;
    };

    let drop_chance = (BASE_DROP_CHANCE * progression.reward_quantity_mult).min(MAX_DROP_CHANCE);

    for death in deaths.read() {
        if death.is_player || death.enemy_type.is_none() || rand::random::<f32>() > drop_chance {
            continue;
        }

        let rarity = roll_rarity(progression.reward_rarity_mult);
        let Some((item_id, weapon_data)) = select_weapon_drop(weapons) else {
            continue;
        };
        let position = death.position + Vec3::new(0.0, 0.0, 2.0);

        commands.entity(level_entity.0).with_children(|parent| {
            parent.spawn(weapon_drop(
                &weapon_assets,
                item_id,
                weapon_data,
                rarity,
                position,
            ));
        });
    }
}

fn select_weapon_drop(weapons: &Weapons) -> Option<(&str, &WeaponData)> {
    if weapons.0.is_empty() {
        return None;
    }

    let index = rand::random_range(0..weapons.0.len());
    weapons
        .0
        .iter()
        .nth(index)
        .map(|(key, weapon_data)| (key.as_str(), weapon_data))
}

fn weapon_drop(
    weapon_assets: &WeaponAssets,
    item_id: &str,
    weapon_data: &WeaponData,
    rarity: ItemRarity,
    position: Vec3,
) -> impl Bundle {
    (
        Name::new(format!("{} Drop", weapon_data.name)),
        ItemDrop {
            item_id: item_id.to_string(),
            rarity,
        },
        PickupRadius(PickupRadius::DEFAULT),
        Sprite::from_atlas_image(
            weapon_assets.sprite.clone(),
            TextureAtlas {
                layout: weapon_assets.layout.clone(),
                index: weapon_data.weapon_sprite_index,
            },
        ),
        Transform::from_translation(position).with_scale(Vec3::splat(DROP_SCALE)),
    )
}

fn roll_rarity(rarity_mult: f32) -> ItemRarity {
    let rarity_mult = rarity_mult.max(1.0);
    let weights = [
        (ItemRarity::Common, 6200.0),
        (ItemRarity::Uncommon, 2500.0 * rarity_mult.powf(0.35)),
        (ItemRarity::Rare, 900.0 * rarity_mult.powf(0.75)),
        (ItemRarity::Epic, 300.0 * rarity_mult.powf(1.10)),
        (ItemRarity::Legendary, 90.0 * rarity_mult.powf(1.50)),
        (ItemRarity::Mythic, 10.0 * rarity_mult.powf(2.00)),
    ];
    let total_weight = weights.iter().map(|(_, weight)| *weight).sum::<f32>();
    let mut pick = rand::random::<f32>() * total_weight;

    for (rarity, weight) in weights {
        if pick <= weight {
            return rarity;
        }

        pick -= weight;
    }

    ItemRarity::Common
}
