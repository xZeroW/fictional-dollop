//! The between-wave inventory menu.

mod ui;

use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

use crate::{
    domain::inventory::{
        InventoryItem, RunInventory, SafeInventory, move_run_item_to_safe, move_safe_item_to_run,
    },
    infrastructure::{
        assets::WeaponAssets,
        weapon::{Weapons, WeaponsHandle},
    },
    menus::{
        Menu,
        item_tooltip::{ItemTooltip, ItemTooltipData, despawn_item_tooltips, spawn_item_tooltip},
    },
};

use ui::{
    DRAGGED_ITEM_GLOBAL_Z_INDEX, DRAGGED_ITEM_LOCAL_Z_INDEX, ITEM_Z_INDEX, InventoryItemUi,
    InventoryKind, InventoryPanelUi,
};

#[derive(Component)]
struct InventoryMenuRoot;

impl InventoryKind {
    fn shortcut_target(self) -> Self {
        match self {
            Self::Run => Self::Safe,
            Self::Safe => Self::Run,
        }
    }
}

#[derive(Component)]
struct InventoryDropTarget {
    kind: InventoryKind,
}

#[derive(Resource)]
struct InventoryUiDirty;

#[derive(Resource, Default)]
#[auto_init_resource(plugin = ui::InventoryMenuPlugin)]
struct InventoryDragState {
    primary_drag: bool,
    successful_drop: bool,
}

pub(crate) use ui::InventoryMenuPlugin;

fn show_inventory_item_tooltip(
    over: On<Pointer<Over>>,
    mut commands: Commands,
    items: Query<&InventoryItemUi>,
    panels: Query<Entity, With<InventoryPanelUi>>,
    tooltips: Query<Entity, With<ItemTooltip>>,
    run_inventory: Res<RunInventory>,
    safe_inventory: Res<SafeInventory>,
    weapon_assets: Res<WeaponAssets>,
    weapons_handle: Res<WeaponsHandle>,
    weapons_assets: Res<Assets<Weapons>>,
) {
    let Ok(item_ui) = items.get(over.event_target()) else {
        return;
    };
    let Some(item) = inventory_item(item_ui.kind, item_ui.index, &run_inventory, &safe_inventory)
    else {
        return;
    };
    let Ok(panel) = panels.single() else {
        return;
    };

    let weapons = weapons_assets.get(&weapons_handle.0);
    let tooltip_data = ItemTooltipData::new(item, weapons);
    let slot_panel_pos = item_ui.panel_pos;

    despawn_item_tooltips(&mut commands, &tooltips);
    commands.entity(panel).with_children(|ui| {
        spawn_item_tooltip(ui, slot_panel_pos, &tooltip_data, &weapon_assets);
    });
}

fn hide_inventory_item_tooltip(
    _: On<Pointer<Out>>,
    mut commands: Commands,
    tooltips: Query<Entity, With<ItemTooltip>>,
) {
    despawn_item_tooltips(&mut commands, &tooltips);
}

fn inventory_item<'a>(
    kind: InventoryKind,
    index: usize,
    run_inventory: &'a RunInventory,
    safe_inventory: &'a SafeInventory,
) -> Option<&'a InventoryItem> {
    match kind {
        InventoryKind::Run => run_inventory.items().get(index),
        InventoryKind::Safe => safe_inventory.items().get(index),
    }
}

fn drag_inventory_item(
    drag: On<Pointer<Drag>>,
    drag_state: Res<InventoryDragState>,
    mut items: Query<(&mut UiTransform, &mut InventoryItemUi)>,
) {
    if !drag_state.primary_drag {
        return;
    }

    let Ok((mut transform, mut item)) = items.get_mut(drag.event_target()) else {
        return;
    };

    item.drag_offset.x += drag.delta.x;
    item.drag_offset.y += drag.delta.y;
    transform.translation = Val2::px(item.drag_offset.x, item.drag_offset.y);
}

fn start_inventory_drag(
    drag: On<Pointer<DragStart>>,
    mut drag_state: ResMut<InventoryDragState>,
    mut items: Query<(&mut Pickable, &mut ZIndex), With<InventoryItemUi>>,
    tooltips: Query<Entity, With<ItemTooltip>>,
    mut commands: Commands,
) {
    if drag.button != PointerButton::Primary {
        drag_state.primary_drag = false;
        return;
    }

    let Ok((mut pickable, mut z_index)) = items.get_mut(drag.event_target()) else {
        return;
    };

    drag_state.primary_drag = true;
    pickable.is_hoverable = false;
    pickable.should_block_lower = false;
    *z_index = ZIndex(DRAGGED_ITEM_LOCAL_Z_INDEX);
    despawn_item_tooltips(&mut commands, &tooltips);
    commands
        .entity(drag.event_target())
        .insert(GlobalZIndex(DRAGGED_ITEM_GLOBAL_Z_INDEX));
}

fn finish_inventory_drag(
    drag: On<Pointer<DragEnd>>,
    mut drag_state: ResMut<InventoryDragState>,
    mut commands: Commands,
    mut items: Query<(
        &mut Pickable,
        &mut UiTransform,
        &mut ZIndex,
        &mut InventoryItemUi,
    )>,
) {
    if drag_state.successful_drop {
        drag_state.successful_drop = false;
        drag_state.primary_drag = false;
        return;
    }

    if !drag_state.primary_drag {
        return;
    }

    if let Ok((mut pickable, mut transform, mut z_index, mut item)) =
        items.get_mut(drag.event_target())
    {
        *pickable = Pickable::default();
        *transform = UiTransform::default();
        *z_index = ZIndex(ITEM_Z_INDEX);
        item.drag_offset = Vec2::ZERO;
        drag_state.primary_drag = false;
        commands
            .entity(drag.event_target())
            .remove::<GlobalZIndex>();
    }
}

fn drop_inventory_item(
    mut drop: On<Pointer<DragDrop>>,
    targets: Query<&InventoryDropTarget>,
    items: Query<&InventoryItemUi>,
    mut run_inventory: ResMut<RunInventory>,
    mut safe_inventory: ResMut<SafeInventory>,
    mut drag_state: ResMut<InventoryDragState>,
    mut commands: Commands,
) {
    if drag_state.successful_drop {
        drop.propagate(false);
        return;
    }
    if !drag_state.primary_drag {
        drop.propagate(false);
        return;
    }

    let Ok(target) = targets.get(drop.event_target()) else {
        return;
    };
    let Ok(item) = items.get(drop.dropped) else {
        return;
    };

    let moved = move_inventory_item(
        item.kind,
        target.kind,
        item.index,
        &mut run_inventory,
        &mut safe_inventory,
    );

    drop.propagate(false);

    if moved {
        drag_state.successful_drop = true;
        commands.insert_resource(InventoryUiDirty);
    }
}

fn shortcut_inventory_item(
    mut click: On<Pointer<Click>>,
    items: Query<&InventoryItemUi>,
    mut run_inventory: ResMut<RunInventory>,
    mut safe_inventory: ResMut<SafeInventory>,
    mut commands: Commands,
) {
    if click.button != PointerButton::Secondary {
        return;
    }

    let Ok(item) = items.get(click.event_target()) else {
        return;
    };

    let moved = move_inventory_item(
        item.kind,
        item.kind.shortcut_target(),
        item.index,
        &mut run_inventory,
        &mut safe_inventory,
    );

    click.propagate(false);

    if moved {
        commands.insert_resource(InventoryUiDirty);
    }
}

fn move_inventory_item(
    source: InventoryKind,
    target: InventoryKind,
    index: usize,
    run_inventory: &mut RunInventory,
    safe_inventory: &mut SafeInventory,
) -> bool {
    match (source, target) {
        (InventoryKind::Run, InventoryKind::Safe) => {
            move_run_item_to_safe(run_inventory, safe_inventory, index)
        }
        (InventoryKind::Safe, InventoryKind::Run) => {
            move_safe_item_to_run(run_inventory, safe_inventory, index)
        }
        _ => false,
    }
}

fn continue_to_equipment_menu(
    _: On<Pointer<Click>>,
    mut run_inventory: ResMut<RunInventory>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    run_inventory.clear();
    next_menu.set(Menu::Equipment);
}
