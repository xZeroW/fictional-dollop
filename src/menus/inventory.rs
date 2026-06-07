//! The between-wave inventory menu.

use bevy::{camera::ClearColorConfig, camera::visibility::RenderLayers, prelude::*};

use crate::{
    assets::WeaponAssets,
    components::ItemRarity,
    game::weapon_data::{Weapons, WeaponsHandle},
    menus::Menu,
    systems::{
        InventoryItem, RunInventory, SAFE_INVENTORY_CAPACITY, SafeInventory, move_run_item_to_safe,
        move_safe_item_to_run,
    },
};

const INVENTORY_UI_Z_INDEX: i32 = 30;
const INVENTORY_CAMERA_ORDER: isize = INVENTORY_UI_Z_INDEX as isize;
const DRAGGED_ITEM_GLOBAL_Z_INDEX: i32 = INVENTORY_UI_Z_INDEX + 1;
const INVENTORY_RENDER_LAYER: usize = 2;
const ITEM_Z_INDEX: i32 = 1;
const ITEM_ICON_Z_INDEX: i32 = 2;
const DRAGGED_ITEM_LOCAL_Z_INDEX: i32 = 30;

const ITEM_SIZE: f32 = 16.0;
const SLOT_SIZE: f32 = 48.0;
const SLOT_GAP: f32 = 6.0;
const RUN_SLOT_COLUMNS: usize = 8;
const SAFE_SLOT_COLUMNS: usize = 5;

const PANEL_WIDTH: f32 = 880.0;
const PANEL_HEIGHT: f32 = 570.0;
const PANEL_FRAME_PADDING: f32 = 10.0;
const CONTINUE_BUTTON_WIDTH: f32 = 220.0;
const CONTINUE_BUTTON_HEIGHT: f32 = 56.0;
const CONTINUE_BUTTON_CENTER_Y: f32 = 520.0;
const RUN_PANEL_POS: (f32, f32) = (30.0, 112.0);
const RUN_PANEL_SIZE: (f32, f32) = (490.0, 350.0);
const SAFE_PANEL_POS: (f32, f32) = (550.0, 112.0);
const SAFE_PANEL_SIZE: (f32, f32) = (300.0, 350.0);
const TITLE_POS: (f32, f32) = (30.0, 28.0);
const HELP_POS: (f32, f32) = (30.0, 68.0);
const SECTION_TITLE_POS: (f32, f32) = (16.0, 20.0);
const SLOT_ORIGIN: (f32, f32) = (18.0, 60.0);
const EMPTY_RUN_TEXT_POS: (f32, f32) = (18.0, 62.0);

const PANEL_COLOR: Color = Color::srgba(0.05, 0.045, 0.075, 0.96);
const PANEL_FRAME_COLOR: Color = Color::srgba(0.14, 0.11, 0.18, 0.98);
const OVERLAY_COLOR: Color = Color::srgba(0.02, 0.0, 0.03, 0.88);
const SLOT_COLOR: Color = Color::srgba(0.08, 0.075, 0.105, 0.95);
const EMPTY_SLOT_COLOR: Color = Color::srgba(0.055, 0.05, 0.075, 0.92);
const DROP_PANEL_COLOR: Color = Color::srgba(0.025, 0.022, 0.035, 0.95);
const TEXT_COLOR: Color = Color::srgb(0.9, 0.86, 0.78);
const MUTED_TEXT_COLOR: Color = Color::srgb(0.62, 0.58, 0.65);
const BUTTON_COLOR: Color = Color::srgb(0.33, 0.21, 0.46);
const DEBUG_BORDER_THICKNESS: f32 = 2.0;

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

    fn debug_border_color(self) -> Color {
        match self {
            Self::Run => Color::srgb(0.1, 0.75, 1.0),
            Self::Safe => Color::srgb(1.0, 0.35, 0.9),
        }
    }
}

#[derive(Component)]
struct InventoryDropTarget {
    kind: InventoryKind,
}

#[derive(Component)]
struct InventoryItemUi {
    kind: InventoryKind,
    index: usize,
    drag_offset: Vec2,
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
    spawn_inventory_menu_root(
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
    spawn_inventory_menu_root(
        &mut commands,
        camera,
        &run_inventory,
        &safe_inventory,
        &weapon_assets,
        weapons,
    );
}

fn spawn_inventory_menu_root(
    commands: &mut Commands,
    camera: Entity,
    run_inventory: &RunInventory,
    safe_inventory: &SafeInventory,
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
        "Drag items into the safe inventory before choosing the next monster buff.",
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
        Node {
            border: UiRect::all(px(DEBUG_BORDER_THICKNESS)),
            ..absolute_node(pos.0, pos.1, size.0, size.1)
        },
        BackgroundColor(DROP_PANEL_COLOR),
        BorderColor::all(kind.debug_border_color()),
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
            InventoryKind::Run => spawn_run_slots(ui, kind, items, weapon_assets, weapons),
            InventoryKind::Safe => spawn_safe_slots(ui, kind, items, weapon_assets, weapons),
        }
    });
}

fn spawn_run_slots(
    ui: &mut ChildSpawnerCommands,
    kind: InventoryKind,
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
        spawn_slot(
            ui,
            kind,
            index,
            item,
            SLOT_ORIGIN.0 + col as f32 * (SLOT_SIZE + SLOT_GAP),
            SLOT_ORIGIN.1 + row as f32 * (SLOT_SIZE + SLOT_GAP),
            weapon_assets,
            weapons,
        );
    }
}

fn spawn_safe_slots(
    ui: &mut ChildSpawnerCommands,
    kind: InventoryKind,
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
            spawn_slot(ui, kind, index, item, x, y, weapon_assets, weapons);
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
            drag_offset: Vec2::ZERO,
        },
        absolute_node(x, y, SLOT_SIZE, SLOT_SIZE),
        UiTransform::default(),
        ZIndex(ITEM_Z_INDEX),
        BackgroundColor(rarity_color),
        Pickable::default(),
    ))
    .observe(shortcut_inventory_item)
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
                (PANEL_WIDTH - CONTINUE_BUTTON_WIDTH) / 2.0,
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

fn absolute_node(x: f32, y: f32, width: f32, height: f32) -> Node {
    Node {
        position_type: PositionType::Absolute,
        left: px(x),
        top: px(y),
        width: px(width),
        height: px(height),
        ..default()
    }
}

fn centered_absolute_node(size: f32) -> Node {
    let inset = (SLOT_SIZE - size) / 2.0;
    absolute_node(inset, inset, size, size)
}

fn rarity_color(rarity: ItemRarity) -> Color {
    match rarity {
        ItemRarity::Common => Color::srgb(0.48, 0.5, 0.54),
        ItemRarity::Uncommon => Color::srgb(0.24, 0.66, 0.31),
        ItemRarity::Rare => Color::srgb(0.22, 0.42, 0.9),
        ItemRarity::Epic => Color::srgb(0.58, 0.27, 0.86),
        ItemRarity::Legendary => Color::srgb(0.96, 0.58, 0.17),
        ItemRarity::Mythic => Color::srgb(0.92, 0.25, 0.32),
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
    mut commands: Commands,
) {
    let Ok((mut pickable, mut z_index)) = items.get_mut(drag.event_target()) else {
        return;
    };

    pickable.is_hoverable = false;
    pickable.should_block_lower = false;
    *z_index = ZIndex(DRAGGED_ITEM_LOCAL_Z_INDEX);
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
