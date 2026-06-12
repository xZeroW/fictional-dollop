use bevy::prelude::*;
use bevy_gauge::prelude::*;

use crate::{
    AppSystems,
    components::{Player, Weapon},
    screens::Screen,
};

use super::inventory::{InventoryItem, SAFE_INVENTORY_CAPACITY, SafeInventory};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum EquipmentSlot {
    MainHand,
}

#[derive(Resource, Default, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct Equipment {
    main_hand: Option<InventoryItem>,
    synced_main_hand: Option<InventoryItem>,
    base_weapon: Option<String>,
}

impl Equipment {
    pub fn item(&self, slot: EquipmentSlot) -> Option<&InventoryItem> {
        match slot {
            EquipmentSlot::MainHand => self.main_hand.as_ref(),
        }
    }

    fn synced_item(&self, slot: EquipmentSlot) -> Option<&InventoryItem> {
        match slot {
            EquipmentSlot::MainHand => self.synced_main_hand.as_ref(),
        }
    }

    fn equip(&mut self, slot: EquipmentSlot, item: InventoryItem) -> Option<InventoryItem> {
        match slot {
            EquipmentSlot::MainHand => self.main_hand.replace(item),
        }
    }

    fn take(&mut self, slot: EquipmentSlot) -> Option<InventoryItem> {
        match slot {
            EquipmentSlot::MainHand => self.main_hand.take(),
        }
    }

    fn mark_synced(&mut self, slot: EquipmentSlot) {
        match slot {
            EquipmentSlot::MainHand => self.synced_main_hand = self.main_hand.clone(),
        }
    }

    fn needs_sync(&self, slot: EquipmentSlot) -> bool {
        self.item(slot) != self.synced_item(slot)
    }
}

pub fn move_safe_item_to_equipment(
    safe_inventory: &mut SafeInventory,
    equipment: &mut Equipment,
    index: usize,
    slot: EquipmentSlot,
) -> bool {
    let Some(item) = safe_inventory.take(index) else {
        return false;
    };

    if let Some(previous_item) = equipment.equip(slot, item)
        && safe_inventory.try_push(previous_item).is_err()
    {
        unreachable!("taking an item from safe inventory must leave room for replaced equipment");
    }

    true
}

pub fn move_equipment_item_to_safe(
    equipment: &mut Equipment,
    safe_inventory: &mut SafeInventory,
    slot: EquipmentSlot,
) -> bool {
    if safe_inventory.is_full() {
        return false;
    }

    let Some(item) = equipment.take(slot) else {
        return false;
    };

    if let Err(item) = safe_inventory.try_push(item) {
        equipment.equip(slot, item);
        false
    } else {
        true
    }
}

pub(super) struct EquipmentSystemsPlugin;

impl Plugin for EquipmentSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Equipment>();
        app.register_type::<EquipmentSlot>();
        app.add_systems(OnEnter(Screen::Gameplay), reset_equipment);
        app.add_systems(OnExit(Screen::Gameplay), remove_equipment);
        app.add_systems(
            Update,
            sync_equipment_to_player
                .in_set(AppSystems::Update)
                .run_if(in_state(Screen::Gameplay)),
        );
    }
}

fn reset_equipment(mut commands: Commands) {
    commands.insert_resource(Equipment::default());
}

fn remove_equipment(mut commands: Commands) {
    commands.remove_resource::<Equipment>();
}

fn sync_equipment_to_player(
    mut equipment: ResMut<Equipment>,
    mut attributes: AttributesMut,
    mut player: Query<(Entity, &mut Weapon), With<Player>>,
) {
    if !equipment.needs_sync(EquipmentSlot::MainHand) {
        return;
    }

    let Ok((player, mut weapon)) = player.single_mut() else {
        return;
    };

    if equipment.base_weapon.is_none() {
        equipment.base_weapon = Some(weapon.key.clone());
    }

    if let Some(old_item) = equipment.synced_main_hand.clone() {
        item_attribute_modifiers(&old_item).remove(player, &mut attributes);
    }

    if let Some(new_item) = equipment.main_hand.clone() {
        item_attribute_modifiers(&new_item).apply(player, &mut attributes);
        weapon.equip(new_item.item_id);
    } else if let Some(base_weapon) = equipment.base_weapon.clone() {
        weapon.equip(base_weapon);
    }

    equipment.mark_synced(EquipmentSlot::MainHand);
}

fn item_attribute_modifiers(item: &InventoryItem) -> ModifierSet {
    let mut modifiers = ModifierSet::new();

    for affix in &item.affixes {
        if let Some((attribute, value)) = affix.attribute_modifier() {
            modifiers.add(attribute, value);
        }
    }

    modifiers
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        components::ItemRarity,
        systems::crafting::{CraftingAffix, CraftingAffixKind},
    };

    fn item(id: usize) -> InventoryItem {
        InventoryItem::new(format!("item_{id}"), ItemRarity::Common)
    }

    fn stat_item(id: usize, kind: CraftingAffixKind, value: u8) -> InventoryItem {
        let mut item = item(id);
        item.affixes.push(CraftingAffix {
            kind,
            tier: 1,
            value,
        });
        item
    }

    #[test]
    fn safe_item_moves_to_main_hand() {
        let mut equipment = Equipment::default();
        let mut safe_inventory = SafeInventory::default();
        safe_inventory.try_push(item(1)).unwrap();

        assert!(move_safe_item_to_equipment(
            &mut safe_inventory,
            &mut equipment,
            0,
            EquipmentSlot::MainHand,
        ));

        assert!(safe_inventory.items().is_empty());
        assert_eq!(
            equipment.item(EquipmentSlot::MainHand).unwrap().item_id,
            "item_1"
        );
    }

    #[test]
    fn replacing_main_hand_returns_previous_item_to_safe_inventory() {
        let mut equipment = Equipment::default();
        equipment.equip(EquipmentSlot::MainHand, item(1));
        let mut safe_inventory = SafeInventory::default();
        safe_inventory.try_push(item(2)).unwrap();

        assert!(move_safe_item_to_equipment(
            &mut safe_inventory,
            &mut equipment,
            0,
            EquipmentSlot::MainHand,
        ));

        assert_eq!(
            equipment.item(EquipmentSlot::MainHand).unwrap().item_id,
            "item_2"
        );
        assert_eq!(safe_inventory.items()[0].item_id, "item_1");
    }

    #[test]
    fn equipped_item_moves_back_to_safe_inventory() {
        let mut equipment = Equipment::default();
        equipment.equip(EquipmentSlot::MainHand, item(1));
        let mut safe_inventory = SafeInventory::default();

        assert!(move_equipment_item_to_safe(
            &mut equipment,
            &mut safe_inventory,
            EquipmentSlot::MainHand,
        ));

        assert!(equipment.item(EquipmentSlot::MainHand).is_none());
        assert_eq!(safe_inventory.items()[0].item_id, "item_1");
    }

    #[test]
    fn unequip_fails_when_safe_inventory_is_full() {
        let mut equipment = Equipment::default();
        equipment.equip(EquipmentSlot::MainHand, item(100));
        let mut safe_inventory = SafeInventory::default();
        for index in 0..SAFE_INVENTORY_CAPACITY {
            safe_inventory.try_push(item(index)).unwrap();
        }

        assert!(!move_equipment_item_to_safe(
            &mut equipment,
            &mut safe_inventory,
            EquipmentSlot::MainHand,
        ));

        assert_eq!(
            equipment.item(EquipmentSlot::MainHand).unwrap().item_id,
            "item_100"
        );
        assert_eq!(safe_inventory.items().len(), SAFE_INVENTORY_CAPACITY);
    }

    #[test]
    fn item_attribute_modifiers_only_include_attribute_affixes() {
        let strength_item = stat_item(1, CraftingAffixKind::Strength, 7);

        assert_eq!(item_attribute_modifiers(&strength_item).len(), 1);
        assert_eq!(item_attribute_modifiers(&item(2)).len(), 0);
    }
}
