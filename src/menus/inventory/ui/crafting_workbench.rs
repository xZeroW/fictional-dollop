use bevy::prelude::*;

use crate::{
    assets::WeaponAssets,
    game::weapon_data::Weapons,
    systems::{CraftingMaterials, InventoryItem, RunInventory, SafeInventory},
};

use super::super::{
    CraftingAction, CraftingItemRef, CraftingSelection, InventoryKind, confirm_crafting_action,
    hide_inventory_item_tooltip, select_crafting_action, show_inventory_item_tooltip,
};
use super::{
    BUTTON_COLOR, CRAFT_CARD_COLOR, CRAFT_CARD_SELECTED_COLOR, CRAFT_PANEL_POS, CRAFT_PANEL_SIZE,
    CRAFT_PREVIEW_COLOR, CraftingActionUi, CraftingConfirmButtonUi, DISABLED_BUTTON_COLOR,
    DROP_PANEL_COLOR, ITEM_ICON_Z_INDEX, ITEM_SIZE, ITEM_Z_INDEX, InventoryItemUi,
    MUTED_TEXT_COLOR, SECTION_TITLE_POS, SLOT_COLOR, SLOT_SIZE, TEXT_COLOR, absolute_node,
    rarity_color,
};

const SELECTED_CARD_POS: (f32, f32) = (18.0, 58.0);
const SELECTED_CARD_SIZE: (f32, f32) = (304.0, 86.0);
const SELECTED_SLOT_POS: (f32, f32) = (128.0, 26.0);
const SELECTED_SLOT_TOOLTIP_CLEARANCE: f32 = 16.0;
const ACTION_TITLE_POS: (f32, f32) = (18.0, 158.0);
const ACTION_CARD_SIZE: (f32, f32) = (146.0, 56.0);
const ACTION_CARD_ORIGIN: (f32, f32) = (18.0, 188.0);
const ACTION_CARD_GAP: f32 = 12.0;
const PREVIEW_POS: (f32, f32) = (18.0, 320.0);
const PREVIEW_SIZE: (f32, f32) = (304.0, 62.0);
const CONFIRM_POS: (f32, f32) = (18.0, 394.0);
const CONFIRM_SIZE: (f32, f32) = (304.0, 30.0);

pub(in crate::menus::inventory) fn spawn_crafting_panel(
    ui: &mut ChildSpawnerCommands,
    run_inventory: &RunInventory,
    safe_inventory: &SafeInventory,
    crafting_selection: &CraftingSelection,
    crafting_materials: &CraftingMaterials,
    weapon_assets: &WeaponAssets,
    weapons: Option<&Weapons>,
) {
    let selected_item = current_crafting_item(crafting_selection, run_inventory, safe_inventory);

    ui.spawn((
        Name::new("Crafting Workbench"),
        absolute_node(
            CRAFT_PANEL_POS.0,
            CRAFT_PANEL_POS.1,
            CRAFT_PANEL_SIZE.0,
            CRAFT_PANEL_SIZE.1,
        ),
        BackgroundColor(DROP_PANEL_COLOR),
        Pickable::default(),
    ))
    .with_children(|ui| {
        spawn_text(
            ui,
            "Crafting Section Title",
            "Crafting Workbench",
            SECTION_TITLE_POS.0,
            SECTION_TITLE_POS.1,
            22.0,
            TEXT_COLOR,
        );
        spawn_selected_item_card(ui, selected_item, weapon_assets, weapons);
        spawn_text(
            ui,
            "Crafting Actions Title",
            "Compatible materials",
            ACTION_TITLE_POS.0,
            ACTION_TITLE_POS.1,
            17.0,
            MUTED_TEXT_COLOR,
        );
        spawn_action_cards(ui, crafting_selection.action, crafting_materials);
        spawn_crafting_preview(ui, selected_item, crafting_selection.action);
        spawn_craft_button(
            ui,
            selected_item,
            crafting_selection.action,
            crafting_materials,
        );
    });
}

fn spawn_selected_item_card(
    ui: &mut ChildSpawnerCommands,
    selected_item: Option<(CraftingItemRef, &InventoryItem)>,
    weapon_assets: &WeaponAssets,
    weapons: Option<&Weapons>,
) {
    ui.spawn((
        Name::new("Crafting Selected Item"),
        absolute_node(
            SELECTED_CARD_POS.0,
            SELECTED_CARD_POS.1,
            SELECTED_CARD_SIZE.0,
            SELECTED_CARD_SIZE.1,
        ),
        BackgroundColor(CRAFT_PREVIEW_COLOR),
        Pickable::IGNORE,
    ))
    .with_children(|ui| {
        if let Some((item_ref, item)) = selected_item {
            spawn_selected_slot(ui, item_ref, item, weapon_assets, weapons);
            spawn_text(
                ui,
                "Crafting Selected Hover Hint",
                "Hover to inspect",
                103.0,
                7.0,
                14.0,
                MUTED_TEXT_COLOR,
            );
        } else {
            spawn_text(
                ui,
                "Crafting Empty Selection",
                "No weapon selected",
                12.0,
                16.0,
                20.0,
                TEXT_COLOR,
            );
            spawn_text(
                ui,
                "Crafting Empty Selection Help",
                "Left click a weapon slot to preview crafts.",
                12.0,
                48.0,
                15.0,
                MUTED_TEXT_COLOR,
            );
        }
    });
}

fn spawn_selected_slot(
    ui: &mut ChildSpawnerCommands,
    item_ref: CraftingItemRef,
    item: &InventoryItem,
    weapon_assets: &WeaponAssets,
    weapons: Option<&Weapons>,
) {
    let panel_pos = Vec2::new(
        CRAFT_PANEL_POS.0 + SELECTED_CARD_POS.0 + SELECTED_SLOT_POS.0
            - SELECTED_SLOT_TOOLTIP_CLEARANCE,
        CRAFT_PANEL_POS.1 + SELECTED_CARD_POS.1 + SELECTED_SLOT_POS.1,
    );
    let rarity_color = rarity_color(item.rarity);

    ui.spawn((
        Name::new("Crafting Selected Item Slot"),
        InventoryItemUi {
            kind: item_ref.kind,
            index: item_ref.index,
            panel_pos,
            drag_offset: Vec2::ZERO,
        },
        absolute_node(
            SELECTED_SLOT_POS.0,
            SELECTED_SLOT_POS.1,
            SLOT_SIZE,
            SLOT_SIZE,
        ),
        ZIndex(ITEM_Z_INDEX),
        BackgroundColor(rarity_color),
        Pickable::default(),
    ))
    .observe(show_inventory_item_tooltip)
    .observe(hide_inventory_item_tooltip)
    .with_children(|ui| {
        spawn_slot_backing(ui);
        spawn_slot_icon(ui, item, weapon_assets, weapons, rarity_color);
    });
}

fn spawn_action_cards(
    ui: &mut ChildSpawnerCommands,
    selected_action: CraftingAction,
    crafting_materials: &CraftingMaterials,
) {
    for (index, action) in CraftingAction::ALL.into_iter().enumerate() {
        let col = index % 2;
        let row = index / 2;
        let x = ACTION_CARD_ORIGIN.0 + col as f32 * (ACTION_CARD_SIZE.0 + ACTION_CARD_GAP);
        let y = ACTION_CARD_ORIGIN.1 + row as f32 * (ACTION_CARD_SIZE.1 + ACTION_CARD_GAP);

        spawn_action_card(
            ui,
            action,
            selected_action == action,
            crafting_materials,
            x,
            y,
        );
    }
}

fn spawn_action_card(
    ui: &mut ChildSpawnerCommands,
    action: CraftingAction,
    selected: bool,
    crafting_materials: &CraftingMaterials,
    x: f32,
    y: f32,
) {
    let color = if selected {
        CRAFT_CARD_SELECTED_COLOR
    } else {
        CRAFT_CARD_COLOR
    };

    ui.spawn((
        Name::new(format!("{} Crafting Action", action.title())),
        Button,
        CraftingActionUi { action },
        absolute_node(x, y, ACTION_CARD_SIZE.0, ACTION_CARD_SIZE.1),
        BackgroundColor(color),
        Pickable::default(),
    ))
    .observe(select_crafting_action)
    .with_children(|ui| {
        spawn_text(
            ui,
            "Crafting Action Title",
            action.title(),
            8.0,
            6.0,
            17.0,
            TEXT_COLOR,
        );
        spawn_text(
            ui,
            "Crafting Action Material",
            format!(
                "{} {}/1",
                action.material(),
                crafting_materials.amount(action.material_kind())
            ),
            8.0,
            30.0,
            13.0,
            MUTED_TEXT_COLOR,
        );
    });
}

fn spawn_crafting_preview(
    ui: &mut ChildSpawnerCommands,
    selected_item: Option<(CraftingItemRef, &InventoryItem)>,
    action: CraftingAction,
) {
    ui.spawn((
        Name::new("Crafting Preview"),
        absolute_node(PREVIEW_POS.0, PREVIEW_POS.1, PREVIEW_SIZE.0, PREVIEW_SIZE.1),
        BackgroundColor(CRAFT_PREVIEW_COLOR),
        Pickable::IGNORE,
    ))
    .with_children(|ui| {
        spawn_text(
            ui,
            "Crafting Preview Title",
            action.description(),
            10.0,
            8.0,
            15.0,
            TEXT_COLOR,
        );
        let preview = if selected_item.is_some() {
            action.preview()
        } else {
            "Select a weapon before crafting."
        };
        spawn_text(
            ui,
            "Crafting Preview Text",
            preview,
            10.0,
            34.0,
            13.0,
            MUTED_TEXT_COLOR,
        );
    });
}

fn spawn_craft_button(
    ui: &mut ChildSpawnerCommands,
    selected_item: Option<(CraftingItemRef, &InventoryItem)>,
    action: CraftingAction,
    crafting_materials: &CraftingMaterials,
) {
    let unavailable_reason = selected_item
        .map(|(_, item)| action.unavailable_reason(item, crafting_materials))
        .unwrap_or(Some("Select a weapon"));

    if let Some(reason) = unavailable_reason {
        ui.spawn((
            Name::new("Crafting Disabled Button"),
            absolute_node(CONFIRM_POS.0, CONFIRM_POS.1, CONFIRM_SIZE.0, CONFIRM_SIZE.1),
            BackgroundColor(DISABLED_BUTTON_COLOR),
            Pickable::IGNORE,
        ))
        .with_children(|ui| {
            spawn_text(
                ui,
                "Crafting Disabled Button Text",
                reason,
                16.0,
                5.0,
                16.0,
                MUTED_TEXT_COLOR,
            );
        });
        return;
    }

    ui.spawn((
        Name::new("Crafting Confirm Button"),
        Button,
        CraftingConfirmButtonUi,
        absolute_node(CONFIRM_POS.0, CONFIRM_POS.1, CONFIRM_SIZE.0, CONFIRM_SIZE.1),
        BackgroundColor(BUTTON_COLOR),
        Pickable::default(),
    ))
    .observe(confirm_crafting_action)
    .with_children(|ui| {
        spawn_text(
            ui,
            "Crafting Confirm Button Text",
            "Craft",
            130.0,
            5.0,
            16.0,
            TEXT_COLOR,
        );
    });
}

fn current_crafting_item<'a>(
    crafting_selection: &CraftingSelection,
    run_inventory: &'a RunInventory,
    safe_inventory: &'a SafeInventory,
) -> Option<(CraftingItemRef, &'a InventoryItem)> {
    let item_ref = crafting_selection.item?;
    let item = crafting_item(item_ref, run_inventory, safe_inventory)?;

    Some((item_ref, item))
}

fn crafting_item<'a>(
    item_ref: CraftingItemRef,
    run_inventory: &'a RunInventory,
    safe_inventory: &'a SafeInventory,
) -> Option<&'a InventoryItem> {
    match item_ref.kind {
        InventoryKind::Run => run_inventory.items().get(item_ref.index),
        InventoryKind::Safe => safe_inventory.items().get(item_ref.index),
    }
}

fn spawn_slot_backing(ui: &mut ChildSpawnerCommands) {
    ui.spawn((
        Name::new("Crafting Selected Slot Backing"),
        centered_absolute_node(SLOT_SIZE - 4.0),
        ZIndex(ITEM_Z_INDEX),
        BackgroundColor(SLOT_COLOR),
        Pickable::IGNORE,
    ));
}

fn spawn_slot_icon(
    ui: &mut ChildSpawnerCommands,
    item: &InventoryItem,
    weapon_assets: &WeaponAssets,
    weapons: Option<&Weapons>,
    rarity_color: Color,
) {
    let weapon_data = weapons.and_then(|weapons| weapons.0.get(&item.item_id));
    if let Some(weapon_data) = weapon_data {
        ui.spawn((
            Name::new(format!("{} Crafting Icon", item.item_id)),
            centered_absolute_node(ITEM_SIZE),
            ZIndex(ITEM_ICON_Z_INDEX),
            ImageNode::from_atlas_image(
                weapon_assets.sprite.clone(),
                TextureAtlas {
                    layout: weapon_assets.layout.clone(),
                    index: weapon_data.weapon_sprite_index,
                },
            ),
            Pickable::IGNORE,
        ));
    } else {
        ui.spawn((
            Name::new(format!("{} Crafting Icon", item.item_id)),
            centered_absolute_node(ITEM_SIZE),
            ZIndex(ITEM_ICON_Z_INDEX),
            BackgroundColor(rarity_color),
            Pickable::IGNORE,
        ));
    }
}

fn spawn_text(
    ui: &mut ChildSpawnerCommands,
    name: &'static str,
    text: impl Into<String>,
    x: f32,
    y: f32,
    font_size: f32,
    color: Color,
) {
    ui.spawn((
        Name::new(name),
        Node {
            position_type: PositionType::Absolute,
            left: px(x),
            top: px(y),
            ..default()
        },
        Text(text.into()),
        TextFont::from_font_size(font_size),
        TextColor(color),
        Pickable::IGNORE,
    ));
}

fn centered_absolute_node(size: f32) -> Node {
    let inset = (SLOT_SIZE - size) / 2.0;
    absolute_node(inset, inset, size, size)
}
