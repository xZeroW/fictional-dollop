use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    components::{ItemDrop, PickupRadius, Player},
    screens::Screen,
    systems::RunInventory,
};

pub(super) struct LootSystemsPlugin;

impl Plugin for LootSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            pickup_loot
                .in_set(AppSystems::SpatialQueries)
                .in_set(PausableSystems)
                .run_if(in_state(Screen::Gameplay)),
        );
    }
}

fn pickup_loot(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    drops: Query<(Entity, &Transform, &ItemDrop, &PickupRadius), Without<Player>>,
    mut inventory: ResMut<RunInventory>,
) {
    let Ok(player_transform) = player.single() else {
        return;
    };

    let player_pos = player_transform.translation.truncate();

    for (entity, transform, drop, radius) in drops.iter() {
        let drop_pos = transform.translation.truncate();
        let pickup_radius = radius.0.max(0.0);

        if player_pos.distance_squared(drop_pos) > pickup_radius * pickup_radius {
            continue;
        }

        inventory.add_item(drop.item_id.clone(), drop.rarity, drop.quantity);
        commands.entity(entity).despawn();
    }
}
