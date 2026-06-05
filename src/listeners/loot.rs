use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    components::{ItemDrop, ItemRarity, PickupRadius},
    game::level::LevelEntity,
    messages::EntityDiedMessage,
    screens::Screen,
    systems::MonsterProgression,
};

const BASE_DROP_CHANCE: f32 = 0.01;
const MAX_DROP_CHANCE: f32 = 0.85;
const DROP_SIZE: f32 = 12.0;

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
) {
    let drop_chance = (BASE_DROP_CHANCE * progression.reward_quantity_mult).min(MAX_DROP_CHANCE);

    for death in deaths.read() {
        if death.is_player || death.enemy_type.is_none() || rand::random::<f32>() > drop_chance {
            continue;
        }

        let rarity = roll_rarity(progression.reward_rarity_mult);
        let (item_id, display_name) = item_for_rarity(rarity);
        let position = death.position + Vec3::new(0.0, 0.0, 2.0);

        commands.entity(level_entity.0).with_children(|parent| {
            parent.spawn((
                Name::new(format!("{display_name} Drop")),
                ItemDrop {
                    item_id: item_id.to_string(),
                    rarity,
                    quantity: 1,
                },
                PickupRadius(PickupRadius::DEFAULT),
                Sprite::from_color(rarity.color(), Vec2::splat(DROP_SIZE)),
                Transform::from_translation(position),
            ));
        });
    }
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

fn item_for_rarity(rarity: ItemRarity) -> (&'static str, &'static str) {
    match rarity {
        ItemRarity::Common => ("monster_remains", "Monster Remains"),
        ItemRarity::Uncommon => ("corrupted_fang", "Corrupted Fang"),
        ItemRarity::Rare => ("gleaming_eye", "Gleaming Eye"),
        ItemRarity::Epic => ("void_shard", "Void Shard"),
        ItemRarity::Legendary => ("ancient_core", "Ancient Core"),
        ItemRarity::Mythic => ("mythic_heart", "Mythic Heart"),
    }
}
