use bevy::prelude::*;

use crate::{
    assets::WeaponAssets,
    game::weapon_data::Weapons,
    menus::Menu,
    systems::{
        CraftingMaterials, InventoryItem, RunInventory, SAFE_INVENTORY_CAPACITY, SafeInventory,
    },
};

use super::super::{
    CraftingSelection, InventoryDropTarget, InventoryKind, InventoryMenuRoot,
    continue_to_monster_buff, drag_inventory_item, drop_inventory_item, finish_inventory_drag,
    select_crafting_item, shortcut_inventory_item, start_inventory_drag,
};
use super::{
    BUTTON_COLOR, CONTINUE_BUTTON_CENTER_X, CONTINUE_BUTTON_CENTER_Y, CONTINUE_BUTTON_HEIGHT,
    CONTINUE_BUTTON_WIDTH, DROP_PANEL_COLOR, EMPTY_RUN_TEXT_POS, EMPTY_SLOT_COLOR, HELP_POS,
    INVENTORY_UI_Z_INDEX, ITEM_ICON_Z_INDEX, ITEM_SIZE, ITEM_Z_INDEX, InventoryItemUi,
    InventoryPanelUi, MUTED_TEXT_COLOR, OVERLAY_COLOR, PANEL_COLOR, PANEL_FRAME_COLOR,
    PANEL_FRAME_PADDING, PANEL_HEIGHT, PANEL_WIDTH, RUN_PANEL_POS, RUN_PANEL_SIZE,
    RUN_SLOT_COLUMNS, SAFE_PANEL_POS, SAFE_PANEL_SIZE, SAFE_SLOT_COLUMNS, SECTION_TITLE_POS,
    SLOT_COLOR, SLOT_GAP, SLOT_ORIGIN, SLOT_SIZE, TEXT_COLOR, TITLE_POS, absolute_node,
    rarity_color, spawn_crafting_panel,
};

pub(in crate::menus::inventory) fn spawn_item_transfer_menu_root(
    commands: &mut Commands,
    camera: Entity,
    run_inventory: &RunInventory,
    safe_inventory: &SafeInventory,
    crafting_selection: &CraftingSelection,
    crafting_materials: &CraftingMaterials,
    weapon_assets: &WeaponAssets,
    weapons: Option<&Weapons>,
) -> Entity {
    commands
        .spawn((
            Name::new("Inventory Menu"),
            InventoryMenuRoot,
            Node {
                position_type: PositionType::Absolute,
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            UiTargetCamera(camera),
            GlobalZIndex(INVENTORY_UI_Z_INDEX),
            BackgroundColor(OVERLAY_COLOR),
            Pickable {
                should_block_lower: true,
                is_hoverable: false,
            },
            DespawnOnExit(Menu::Inventory),
        ))
        .with_children(|ui| {
            ui.spawn((
                Name::new("Inventory Panel Frame"),
                Node {
                    position_type: PositionType::Relative,
                    width: px(PANEL_WIDTH + PANEL_FRAME_PADDING * 2.0),
                    height: px(PANEL_HEIGHT + PANEL_FRAME_PADDING * 2.0),
                    ..default()
                },
                BackgroundColor(PANEL_FRAME_COLOR),
                Pickable::IGNORE,
            ))
            .with_children(|ui| {
                ui.spawn((
                    Name::new("Inventory Panel"),
                    InventoryPanelUi,
                    absolute_node(
                        PANEL_FRAME_PADDING,
                        PANEL_FRAME_PADDING,
                        PANEL_WIDTH,
                        PANEL_HEIGHT,
                    ),
                    BackgroundColor(PANEL_COLOR),
                    Pickable::IGNORE,
                ))
                .with_children(|ui| {
                    spawn_inventory_content(
                        ui,
                        run_inventory,
                        safe_inventory,
                        crafting_selection,
                        crafting_materials,
                        weapon_assets,
                        weapons,
                    );
                });
            });
        })
        .id()
}

fn spawn_inventory_content(
    ui: &mut ChildSpawnerCommands,
    run_inventory: &RunInventory,
    safe_inventory: &SafeInventory,
    crafting_selection: &CraftingSelection,
    crafting_materials: &CraftingMaterials,
    weapon_assets: &WeaponAssets,
    weapons: Option<&Weapons>,
) {
    spawn_text(
        ui,
        "Inventory Title",
        "Wave loot",
        TITLE_POS.0,
        TITLE_POS.1,
        38.0,
        TEXT_COLOR,
    );
    spawn_text(
        ui,
        "Inventory Help",
        "Drag items into safe inventory. Left click a weapon to preview craft options.",
        HELP_POS.0,
        HELP_POS.1,
        20.0,
        MUTED_TEXT_COLOR,
    );

    spawn_inventory_panel(
        ui,
        "Run Inventory",
        InventoryKind::Run,
        RUN_PANEL_POS,
        RUN_PANEL_SIZE,
        format!("Run Inventory ({})", run_inventory.items().len()),
        run_inventory.items(),
        weapon_assets,
        weapons,
    );
    spawn_inventory_panel(
        ui,
        "Safe Inventory",
        InventoryKind::Safe,
        SAFE_PANEL_POS,
        SAFE_PANEL_SIZE,
        format!(
            "Safe Inventory ({}/{SAFE_INVENTORY_CAPACITY})",
            safe_inventory.items().len()
        ),
        safe_inventory.items(),
        weapon_assets,
        weapons,
    );

    spawn_crafting_panel(
        ui,
        run_inventory,
        safe_inventory,
        crafting_selection,
        crafting_materials,
        weapon_assets,
        weapons,
    );

    spawn_continue_button(ui);
}

fn spawn_inventory_panel(
    ui: &mut ChildSpawnerCommands,
    name: &'static str,
    kind: InventoryKind,
    pos: (f32, f32),
    size: (f32, f32),
    title: String,
    items: &[InventoryItem],
    weapon_assets: &WeaponAssets,
    weapons: Option<&Weapons>,
) {
    ui.spawn((
        Name::new(name),
        InventoryDropTarget { kind },
        absolute_node(pos.0, pos.1, size.0, size.1),
        BackgroundColor(DROP_PANEL_COLOR),
        Pickable::default(),
    ))
    .observe(drop_inventory_item)
    .with_children(|ui| {
        spawn_text(
            ui,
            "Inventory Section Title",
            title,
            SECTION_TITLE_POS.0,
            SECTION_TITLE_POS.1,
            22.0,
            TEXT_COLOR,
        );

        match kind {
            InventoryKind::Run => spawn_run_slots(ui, kind, pos, items, weapon_assets, weapons),
            InventoryKind::Safe => spawn_safe_slots(ui, kind, pos, items, weapon_assets, weapons),
        }
    });
}

fn spawn_run_slots(
    ui: &mut ChildSpawnerCommands,
    kind: InventoryKind,
    panel_pos: (f32, f32),
    items: &[InventoryItem],
    weapon_assets: &WeaponAssets,
    weapons: Option<&Weapons>,
) {
    if items.is_empty() {
        spawn_text(
            ui,
            "Run Inventory Empty Text",
            "No drops this wave yet.",
            EMPTY_RUN_TEXT_POS.0,
            EMPTY_RUN_TEXT_POS.1,
            18.0,
            MUTED_TEXT_COLOR,
        );
        return;
    }

    for (index, item) in items.iter().enumerate() {
        let col = index % RUN_SLOT_COLUMNS;
        let row = index / RUN_SLOT_COLUMNS;
        let x = SLOT_ORIGIN.0 + col as f32 * (SLOT_SIZE + SLOT_GAP);
        let y = SLOT_ORIGIN.1 + row as f32 * (SLOT_SIZE + SLOT_GAP);
        spawn_slot(
            ui,
            kind,
            index,
            item,
            Vec2::new(panel_pos.0 + x, panel_pos.1 + y),
            x,
            y,
            weapon_assets,
            weapons,
        );
    }
}

fn spawn_safe_slots(
    ui: &mut ChildSpawnerCommands,
    kind: InventoryKind,
    panel_pos: (f32, f32),
    items: &[InventoryItem],
    weapon_assets: &WeaponAssets,
    weapons: Option<&Weapons>,
) {
    for index in 0..SAFE_INVENTORY_CAPACITY {
        let col = index % SAFE_SLOT_COLUMNS;
        let row = index / SAFE_SLOT_COLUMNS;
        let x = SLOT_ORIGIN.0 + col as f32 * (SLOT_SIZE + SLOT_GAP);
        let y = SLOT_ORIGIN.1 + row as f32 * (SLOT_SIZE + SLOT_GAP);

        if let Some(item) = items.get(index) {
            spawn_slot(
                ui,
                kind,
                index,
                item,
                Vec2::new(panel_pos.0 + x, panel_pos.1 + y),
                x,
                y,
                weapon_assets,
                weapons,
            );
        } else {
            spawn_empty_slot(ui, x, y);
        }
    }
}

fn spawn_slot(
    ui: &mut ChildSpawnerCommands,
    kind: InventoryKind,
    index: usize,
    item: &InventoryItem,
    panel_pos: Vec2,
    x: f32,
    y: f32,
    weapon_assets: &WeaponAssets,
    weapons: Option<&Weapons>,
) {
    let rarity_color = rarity_color(item.rarity);
    ui.spawn((
        Name::new(format!("Inventory Slot {}", index + 1)),
        InventoryItemUi {
            kind,
            index,
            panel_pos,
            drag_offset: Vec2::ZERO,
        },
        absolute_node(x, y, SLOT_SIZE, SLOT_SIZE),
        UiTransform::default(),
        ZIndex(ITEM_Z_INDEX),
        BackgroundColor(rarity_color),
        Pickable::default(),
    ))
    .observe(select_crafting_item)
    .observe(shortcut_inventory_item)
    .observe(super::super::show_inventory_item_tooltip)
    .observe(super::super::hide_inventory_item_tooltip)
    .observe(start_inventory_drag)
    .observe(drag_inventory_item)
    .observe(finish_inventory_drag)
    .with_children(|ui| {
        ui.spawn((
            Name::new("Inventory Slot Backing"),
            centered_absolute_node(SLOT_SIZE - 4.0),
            ZIndex(ITEM_Z_INDEX),
            BackgroundColor(SLOT_COLOR),
            Pickable::IGNORE,
        ));

        let weapon_data = weapons.and_then(|weapons| weapons.0.get(&item.item_id));
        if let Some(weapon_data) = weapon_data {
            ui.spawn((
                Name::new(format!("{} Icon", item.item_id)),
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
                Name::new(format!("{} Icon", item.item_id)),
                centered_absolute_node(ITEM_SIZE),
                ZIndex(ITEM_ICON_Z_INDEX),
                BackgroundColor(rarity_color),
                Pickable::IGNORE,
            ));
        }
    });
}

fn spawn_empty_slot(ui: &mut ChildSpawnerCommands, x: f32, y: f32) {
    ui.spawn((
        Name::new("Empty Safe Inventory Slot"),
        absolute_node(x, y, SLOT_SIZE, SLOT_SIZE),
        BackgroundColor(EMPTY_SLOT_COLOR),
        Pickable::IGNORE,
    ));
}

fn spawn_continue_button(ui: &mut ChildSpawnerCommands) {
    ui.spawn((
        Name::new("Inventory Continue Button"),
        Button,
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..absolute_node(
                CONTINUE_BUTTON_CENTER_X - CONTINUE_BUTTON_WIDTH / 2.0,
                CONTINUE_BUTTON_CENTER_Y - CONTINUE_BUTTON_HEIGHT / 2.0,
                CONTINUE_BUTTON_WIDTH,
                CONTINUE_BUTTON_HEIGHT,
            )
        },
        BackgroundColor(BUTTON_COLOR),
        Pickable::default(),
    ))
    .observe(continue_to_monster_buff)
    .with_children(|ui| {
        ui.spawn((
            Name::new("Inventory Continue Text"),
            Text("Continue".to_string()),
            TextFont::from_font_size(26.0),
            TextColor(TEXT_COLOR),
            Pickable::IGNORE,
        ));
    });
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
