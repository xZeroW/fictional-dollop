use bevy::prelude::*;

use crate::systems::{
    CraftingMaterial, CraftingMaterials, InventoryItem, RunInventory, SafeInventory,
};

use super::{InventoryKind, InventoryUiDirty, ui::CraftingActionUi, ui::CraftingConfirmButtonUi};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::menus::inventory) struct CraftingItemRef {
    pub(in crate::menus::inventory) kind: InventoryKind,
    pub(in crate::menus::inventory) index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::menus::inventory) enum CraftingAction {
    Improve,
    Add,
    Reroll,
    Guarantee,
}

impl CraftingAction {
    pub(in crate::menus::inventory) const ALL: [Self; 4] =
        [Self::Improve, Self::Add, Self::Reroll, Self::Guarantee];

    pub(in crate::menus::inventory) fn title(self) -> &'static str {
        match self {
            Self::Improve => "Improve",
            Self::Add => "Add",
            Self::Reroll => "Reroll",
            Self::Guarantee => "Guarantee",
        }
    }

    pub(in crate::menus::inventory) fn material(self) -> &'static str {
        self.material_kind().label()
    }

    pub(in crate::menus::inventory) fn material_kind(self) -> CraftingMaterial {
        match self {
            Self::Improve => CraftingMaterial::Whetstone,
            Self::Add => CraftingMaterial::Ember,
            Self::Reroll => CraftingMaterial::ReforgeOre,
            Self::Guarantee => CraftingMaterial::Essence,
        }
    }

    pub(in crate::menus::inventory) fn description(self) -> &'static str {
        match self {
            Self::Improve => "Safe quality upgrade",
            Self::Add => "Add a random affix",
            Self::Reroll => "Replace one affix",
            Self::Guarantee => "Force an affix family",
        }
    }

    pub(in crate::menus::inventory) fn preview(self) -> &'static str {
        match self {
            Self::Improve => "Quality +2% up to tier cap.",
            Self::Add => "Rolls one random affix if a slot is open.",
            Self::Reroll => "Replaces one unsealed affix.",
            Self::Guarantee => "Essence guarantees the affix family.",
        }
    }

    pub(in crate::menus::inventory) fn unavailable_reason(
        self,
        item: &InventoryItem,
        materials: &CraftingMaterials,
    ) -> Option<&'static str> {
        if !materials.can_spend(self.material_kind(), 1) {
            return Some("Missing material");
        }

        match self {
            Self::Improve if !item.can_improve_quality() => Some("Quality is already maxed"),
            Self::Add if !item.can_add_affix() => Some("No open affix slot"),
            Self::Reroll if !item.can_reforge_affix() => Some("No affix to reroll"),
            Self::Guarantee if !item.can_guarantee_affix() => Some("No craftable affix slot"),
            _ => None,
        }
    }

    fn apply(self, item: &mut InventoryItem) -> bool {
        match self {
            Self::Improve => item.improve_quality(),
            Self::Add => item.add_random_affix(),
            Self::Reroll => item.reforge_random_affix(),
            Self::Guarantee => item.guarantee_speed_affix(),
        }
    }
}

#[derive(Resource)]
pub(in crate::menus::inventory) struct CraftingSelection {
    pub(in crate::menus::inventory) item: Option<CraftingItemRef>,
    pub(in crate::menus::inventory) action: CraftingAction,
}

impl Default for CraftingSelection {
    fn default() -> Self {
        Self {
            item: None,
            action: CraftingAction::Improve,
        }
    }
}

pub(in crate::menus::inventory) fn select_crafting_item(
    mut click: On<Pointer<Click>>,
    items: Query<&super::ui::InventoryItemUi>,
    mut crafting_selection: ResMut<CraftingSelection>,
    mut commands: Commands,
) {
    if click.button != PointerButton::Primary {
        return;
    }

    let Ok(item) = items.get(click.event_target()) else {
        return;
    };

    crafting_selection.item = Some(CraftingItemRef {
        kind: item.kind,
        index: item.index,
    });
    click.propagate(false);
    commands.insert_resource(InventoryUiDirty);
}

pub(in crate::menus::inventory) fn select_crafting_action(
    mut click: On<Pointer<Click>>,
    actions: Query<&CraftingActionUi>,
    mut crafting_selection: ResMut<CraftingSelection>,
    mut commands: Commands,
) {
    if click.button != PointerButton::Primary {
        return;
    }

    let Ok(action) = actions.get(click.event_target()) else {
        return;
    };

    crafting_selection.action = action.action;
    click.propagate(false);
    commands.insert_resource(InventoryUiDirty);
}

pub(in crate::menus::inventory) fn confirm_crafting_action(
    mut click: On<Pointer<Click>>,
    buttons: Query<&CraftingConfirmButtonUi>,
    mut run_inventory: ResMut<RunInventory>,
    mut safe_inventory: ResMut<SafeInventory>,
    mut crafting_materials: ResMut<CraftingMaterials>,
    crafting_selection: Res<CraftingSelection>,
    mut commands: Commands,
) {
    if click.button != PointerButton::Primary || buttons.get(click.event_target()).is_err() {
        return;
    }

    let Some(item_ref) = crafting_selection.item else {
        return;
    };
    let action = crafting_selection.action;
    let Some(item) = inventory_item_mut(item_ref, &mut run_inventory, &mut safe_inventory) else {
        return;
    };

    if action
        .unavailable_reason(item, &crafting_materials)
        .is_some()
    {
        return;
    }
    if !action.apply(item) {
        return;
    }
    crafting_materials.spend(action.material_kind(), 1);

    click.propagate(false);
    commands.insert_resource(InventoryUiDirty);
}

fn inventory_item_mut<'a>(
    item_ref: CraftingItemRef,
    run_inventory: &'a mut RunInventory,
    safe_inventory: &'a mut SafeInventory,
) -> Option<&'a mut InventoryItem> {
    match item_ref.kind {
        InventoryKind::Run => run_inventory.item_mut(item_ref.index),
        InventoryKind::Safe => safe_inventory.item_mut(item_ref.index),
    }
}
