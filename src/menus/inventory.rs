//! The between-wave inventory menu.

mod ui;

use bevy::{camera::ClearColorConfig, camera::visibility::RenderLayers, prelude::*};

use crate::{
    assets::WeaponAssets,
    game::weapon_data::{Weapons, WeaponsHandle},
    menus::Menu,
    systems::{
        InventoryItem, RunInventory, SafeInventory, move_run_item_to_safe, move_safe_item_to_run,
    },
};

use ui::{
    DRAGGED_ITEM_GLOBAL_Z_INDEX, DRAGGED_ITEM_LOCAL_Z_INDEX, INVENTORY_RENDER_LAYER, ITEM_Z_INDEX,
    InventoryItemTooltip, InventoryItemUi, InventoryPanelUi, InventoryTooltipData,
    despawn_inventory_tooltips, spawn_inventory_item_tooltip, spawn_item_transfer_menu_root,
};

const INVENTORY_CAMERA_ORDER: isize = ui::INVENTORY_UI_Z_INDEX as isize;

#[derive(Component)]
struct InventoryCamera;

#[derive(Component)]
struct InventoryMenuRoot;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum InventoryKind {
    Run,
    Safe,
}

impl InventoryKind {
    fn opposite(self) -> Self {
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
struct InventoryDragState {
    successful_drop: bool,
}

pub(super) struct InventoryMenuPlugin;

impl Plugin for InventoryMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InventoryDragState>();
        app.add_systems(Startup, spawn_inventory_camera);
        app.add_systems(OnEnter(Menu::Inventory), spawn_inventory_menu);
        app.add_systems(
            Update,
            refresh_inventory_menu.run_if(in_state(Menu::Inventory)),
        );
    }
}

fn spawn_inventory_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Inventory Camera"),
        InventoryCamera,
        Camera2d,
        Camera {
            order: INVENTORY_CAMERA_ORDER,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        Msaa::Off,
        RenderLayers::layer(INVENTORY_RENDER_LAYER),
    ));
}

fn spawn_inventory_menu(
    mut commands: Commands,
    camera: Query<Entity, With<InventoryCamera>>,
    run_inventory: Res<RunInventory>,
    safe_inventory: Res<SafeInventory>,
    weapon_assets: Res<WeaponAssets>,
    weapons_handle: Res<WeaponsHandle>,
    weapons_assets: Res<Assets<Weapons>>,
) {
    let Ok(camera) = camera.single() else {
        return;
    };

    let weapons = weapons_assets.get(&weapons_handle.0);
    spawn_item_transfer_menu_root(
        &mut commands,
        camera,
        &run_inventory,
        &safe_inventory,
        &weapon_assets,
        weapons,
    );
}

fn refresh_inventory_menu(
    mut commands: Commands,
    dirty: Option<Res<InventoryUiDirty>>,
    roots: Query<Entity, With<InventoryMenuRoot>>,
    camera: Query<Entity, With<InventoryCamera>>,
    run_inventory: Res<RunInventory>,
    safe_inventory: Res<SafeInventory>,
    weapon_assets: Res<WeaponAssets>,
    weapons_handle: Res<WeaponsHandle>,
    weapons_assets: Res<Assets<Weapons>>,
) {
    if dirty.is_none() {
        return;
    }

    commands.remove_resource::<InventoryUiDirty>();

    for root in roots.iter() {
        commands.entity(root).despawn();
    }

    let Ok(camera) = camera.single() else {
        return;
    };

    let weapons = weapons_assets.get(&weapons_handle.0);
    spawn_item_transfer_menu_root(
        &mut commands,
        camera,
        &run_inventory,
        &safe_inventory,
        &weapon_assets,
        weapons,
    );
}

fn show_inventory_item_tooltip(
    over: On<Pointer<Over>>,
    mut commands: Commands,
    items: Query<&InventoryItemUi>,
    panels: Query<Entity, With<InventoryPanelUi>>,
    tooltips: Query<Entity, With<InventoryItemTooltip>>,
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
    let tooltip_data = InventoryTooltipData::new(item, weapons);
    let slot_panel_pos = item_ui.panel_pos;

    despawn_inventory_tooltips(&mut commands, &tooltips);
    commands.entity(panel).with_children(|ui| {
        spawn_inventory_item_tooltip(ui, slot_panel_pos, &tooltip_data, &weapon_assets);
    });
}

fn hide_inventory_item_tooltip(
    _: On<Pointer<Out>>,
    mut commands: Commands,
    tooltips: Query<Entity, With<InventoryItemTooltip>>,
) {
    despawn_inventory_tooltips(&mut commands, &tooltips);
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
    mut items: Query<(&mut UiTransform, &mut InventoryItemUi)>,
) {
    let Ok((mut transform, mut item)) = items.get_mut(drag.event_target()) else {
        return;
    };

    item.drag_offset.x += drag.delta.x;
    item.drag_offset.y += drag.delta.y;
    transform.translation = Val2::px(item.drag_offset.x, item.drag_offset.y);
}

fn start_inventory_drag(
    drag: On<Pointer<DragStart>>,
    mut items: Query<(&mut Pickable, &mut ZIndex), With<InventoryItemUi>>,
    tooltips: Query<Entity, With<InventoryItemTooltip>>,
    mut commands: Commands,
) {
    let Ok((mut pickable, mut z_index)) = items.get_mut(drag.event_target()) else {
        return;
    };

    pickable.is_hoverable = false;
    pickable.should_block_lower = false;
    *z_index = ZIndex(DRAGGED_ITEM_LOCAL_Z_INDEX);
    despawn_inventory_tooltips(&mut commands, &tooltips);
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
        return;
    }

    if let Ok((mut pickable, mut transform, mut z_index, mut item)) =
        items.get_mut(drag.event_target())
    {
        *pickable = Pickable::default();
        *transform = UiTransform::default();
        *z_index = ZIndex(ITEM_Z_INDEX);
        item.drag_offset = Vec2::ZERO;
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
        item.kind.opposite(),
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

fn continue_to_monster_buff(
    _: On<Pointer<Click>>,
    mut run_inventory: ResMut<RunInventory>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    run_inventory.clear();
    next_menu.set(Menu::MonsterBuff);
}
