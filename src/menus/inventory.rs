//! The between-wave inventory menu.

use bevy::{camera::ClearColorConfig, camera::visibility::RenderLayers, prelude::*};
use bevy_lunex::{RecomputeUiLayout, prelude::*};

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

const INVENTORY_CAMERA_ORDER: isize = 30;
const INVENTORY_RENDER_LAYER: usize = 2;

const ITEM_SIZE: f32 = 16.0;
const SLOT_SIZE: f32 = 48.0;
const SLOT_GAP: f32 = 6.0;

const PANEL_COLOR: Color = Color::srgba(0.05, 0.045, 0.075, 0.96);
const PANEL_FRAME_COLOR: Color = Color::srgba(0.14, 0.11, 0.18, 0.98);
const OVERLAY_COLOR: Color = Color::srgba(0.02, 0.0, 0.03, 0.88);
const SLOT_COLOR: Color = Color::srgba(0.08, 0.075, 0.105, 0.95);
const TEXT_COLOR: Color = Color::srgb(0.9, 0.86, 0.78);
const MUTED_TEXT_COLOR: Color = Color::srgb(0.62, 0.58, 0.65);
const BUTTON_COLOR: Color = Color::srgb(0.33, 0.21, 0.46);

#[derive(Component)]
struct InventoryCamera;

#[derive(Component)]
struct InventoryMenuRoot;

#[derive(Component)]
struct StaleInventoryMenuRoot {
    frames_remaining: u8,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum InventoryKind {
    Run,
    Safe,
}

#[derive(Component)]
struct InventoryDropTarget {
    kind: InventoryKind,
}

#[derive(Component)]
struct InventoryItemUi {
    kind: InventoryKind,
    index: usize,
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
            (refresh_inventory_menu, cleanup_stale_inventory_menus)
                .run_if(in_state(Menu::Inventory)),
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
        UiSourceCamera::<1>,
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
    let root = spawn_inventory_menu_root(
        &mut commands,
        &run_inventory,
        &safe_inventory,
        &weapon_assets,
        weapons,
    );

    commands.entity(camera).add_child(root);
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

    let Ok(camera) = camera.single() else {
        return;
    };

    let weapons = weapons_assets.get(&weapons_handle.0);
    let root = spawn_inventory_menu_root(
        &mut commands,
        &run_inventory,
        &safe_inventory,
        &weapon_assets,
        weapons,
    );
    commands.entity(camera).add_child(root);

    for root in roots.iter() {
        commands
            .entity(root)
            .remove::<InventoryMenuRoot>()
            .insert(StaleInventoryMenuRoot {
                frames_remaining: 1,
            });
    }
}

fn cleanup_stale_inventory_menus(
    mut commands: Commands,
    mut stale_roots: Query<(Entity, &mut StaleInventoryMenuRoot, &mut Visibility)>,
) {
    for (entity, mut stale, mut visibility) in &mut stale_roots {
        if stale.frames_remaining > 0 {
            stale.frames_remaining -= 1;
            if stale.frames_remaining == 0 {
                *visibility = Visibility::Hidden;
            }
        } else {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_inventory_menu_root(
    commands: &mut Commands,
    run_inventory: &RunInventory,
    safe_inventory: &SafeInventory,
    weapon_assets: &WeaponAssets,
    weapons: Option<&Weapons>,
) -> Entity {
    commands
        .spawn((
            Name::new("Inventory Menu"),
            InventoryMenuRoot,
            UiLayoutRoot::new_2d(),
            UiFetchFromCamera::<1>,
            RenderLayers::layer(INVENTORY_RENDER_LAYER),
            DespawnOnExit(Menu::Inventory),
        ))
        .with_children(|ui| {
            spawn_rect(
                ui,
                "Inventory Overlay",
                UiLayout::boundary().pos1(Ab(0.0)).pos2(Rl(100.0)).pack(),
                0.0,
                OVERLAY_COLOR,
            );

            ui.spawn((
                Name::new("Inventory Panel Frame"),
                UiLayout::window()
                    .pos((Rl(50.0), Rl(50.0)))
                    .anchor(Anchor::CENTER)
                    .size((Ab(900.0), Ab(590.0)))
                    .pack(),
                UiDepth::Set(2.0),
                Sprite::from_color(PANEL_FRAME_COLOR, Vec2::ONE),
                RenderLayers::layer(INVENTORY_RENDER_LAYER),
                Pickable::IGNORE,
            ))
            .with_children(|ui| {
                spawn_rect(
                    ui,
                    "Inventory Panel",
                    UiLayout::window()
                        .pos((Rl(50.0), Rl(50.0)))
                        .anchor(Anchor::CENTER)
                        .size((Ab(880.0), Ab(570.0)))
                        .pack(),
                    3.0,
                    PANEL_COLOR,
                );

                spawn_text(
                    ui,
                    "Inventory Title",
                    "Wave loot",
                    UiLayout::window()
                        .pos((Ab(30.0), Ab(28.0)))
                        .anchor(Anchor::TOP_LEFT)
                        .pack(),
                    4.0,
                    38.0,
                    TEXT_COLOR,
                );
                spawn_text(
                    ui,
                    "Inventory Help",
                    "Drag items into the safe inventory before choosing the next monster buff.",
                    UiLayout::window()
                        .pos((Ab(30.0), Ab(68.0)))
                        .anchor(Anchor::TOP_LEFT)
                        .pack(),
                    4.0,
                    20.0,
                    MUTED_TEXT_COLOR,
                );

                spawn_inventory_panel(
                    ui,
                    "Run Inventory",
                    InventoryKind::Run,
                    (30.0, 112.0),
                    (490.0, 350.0),
                    format!("Run Inventory ({})", run_inventory.items().len()),
                    run_inventory.items(),
                    weapon_assets,
                    weapons,
                );
                spawn_inventory_panel(
                    ui,
                    "Safe Inventory",
                    InventoryKind::Safe,
                    (550.0, 112.0),
                    (300.0, 350.0),
                    format!(
                        "Safe Inventory ({}/{SAFE_INVENTORY_CAPACITY})",
                        safe_inventory.items().len()
                    ),
                    safe_inventory.items(),
                    weapon_assets,
                    weapons,
                );

                ui.spawn((
                    Name::new("Inventory Continue Button"),
                    UiLayout::window()
                        .pos((Rl(50.0), Ab(520.0)))
                        .anchor(Anchor::CENTER)
                        .size((Ab(220.0), Ab(56.0)))
                        .pack(),
                    UiDepth::Set(5.0),
                    Sprite::from_color(BUTTON_COLOR, Vec2::ONE),
                    RenderLayers::layer(INVENTORY_RENDER_LAYER),
                    Pickable::default(),
                ))
                .observe(continue_to_monster_buff)
                .with_children(|ui| {
                    spawn_text(
                        ui,
                        "Inventory Continue Text",
                        "Continue",
                        UiLayout::window()
                            .pos((Rl(50.0), Rl(50.0)))
                            .anchor(Anchor::CENTER)
                            .pack(),
                        6.0,
                        26.0,
                        TEXT_COLOR,
                    );
                });
            });
        })
        .id()
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
        UiLayout::window()
            .pos((Ab(pos.0), Ab(pos.1)))
            .anchor(Anchor::TOP_LEFT)
            .size((Ab(size.0), Ab(size.1)))
            .pack(),
        UiDepth::Set(4.0),
        Sprite::from_color(Color::srgba(0.025, 0.022, 0.035, 0.95), Vec2::ONE),
        RenderLayers::layer(INVENTORY_RENDER_LAYER),
    ))
    .observe(drop_inventory_item)
    .with_children(|ui| {
        spawn_text(
            ui,
            "Inventory Section Title",
            title,
            UiLayout::window()
                .pos((Ab(16.0), Ab(20.0)))
                .anchor(Anchor::TOP_LEFT)
                .pack(),
            5.0,
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
    const COLUMNS: usize = 8;

    if items.is_empty() {
        spawn_text(
            ui,
            "Run Inventory Empty Text",
            "No drops this wave yet.",
            UiLayout::window()
                .pos((Ab(18.0), Ab(62.0)))
                .anchor(Anchor::TOP_LEFT)
                .pack(),
            5.0,
            18.0,
            MUTED_TEXT_COLOR,
        );
        return;
    }

    for (index, item) in items.iter().enumerate() {
        let col = index % COLUMNS;
        let row = index / COLUMNS;
        spawn_slot(
            ui,
            kind,
            index,
            item,
            18.0 + col as f32 * (SLOT_SIZE + SLOT_GAP),
            60.0 + row as f32 * (SLOT_SIZE + SLOT_GAP),
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
    const COLUMNS: usize = 5;

    for index in 0..SAFE_INVENTORY_CAPACITY {
        let col = index % COLUMNS;
        let row = index / COLUMNS;
        let x = 18.0 + col as f32 * (SLOT_SIZE + SLOT_GAP);
        let y = 60.0 + row as f32 * (SLOT_SIZE + SLOT_GAP);

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
        InventoryItemUi { kind, index },
        UiLayout::window()
            .pos((Ab(x), Ab(y)))
            .anchor(Anchor::TOP_LEFT)
            .size((Ab(SLOT_SIZE), Ab(SLOT_SIZE)))
            .pack(),
        UiDepth::Set(5.0),
        Sprite::from_color(rarity_color, Vec2::ONE),
        RenderLayers::layer(INVENTORY_RENDER_LAYER),
        Pickable::default(),
    ))
    .observe(shortcut_inventory_item)
    .observe(start_inventory_drag)
    .observe(drag_inventory_item)
    .observe(finish_inventory_drag)
    .with_children(|ui| {
        spawn_rect(
            ui,
            "Inventory Slot Backing",
            UiLayout::window()
                .pos((Rl(50.0), Rl(50.0)))
                .anchor(Anchor::CENTER)
                .size((Ab(SLOT_SIZE - 4.0), Ab(SLOT_SIZE - 4.0)))
                .pack(),
            6.0,
            SLOT_COLOR,
        );

        let weapon_data = weapons.and_then(|weapons| weapons.0.get(&item.item_id));
        let mut icon = ui.spawn((
            Name::new(format!("{} Icon", item.item_id)),
            UiLayout::window()
                .pos((Rl(50.0), Rl(50.0)))
                .anchor(Anchor::CENTER)
                .size((Ab(ITEM_SIZE), Ab(ITEM_SIZE)))
                .pack(),
            UiDepth::Set(8.0),
            Pickable::IGNORE,
            RenderLayers::layer(INVENTORY_RENDER_LAYER),
        ));

        if let Some(weapon_data) = weapon_data {
            icon.insert(Sprite::from_atlas_image(
                weapon_assets.sprite.clone(),
                TextureAtlas {
                    layout: weapon_assets.layout.clone(),
                    index: weapon_data.weapon_sprite_index,
                },
            ));
        } else {
            icon.insert(Sprite::from_color(rarity_color, Vec2::ONE));
        }
    });
}

fn spawn_empty_slot(ui: &mut ChildSpawnerCommands, x: f32, y: f32) {
    ui.spawn((
        Name::new("Empty Safe Inventory Slot"),
        UiLayout::window()
            .pos((Ab(x), Ab(y)))
            .anchor(Anchor::TOP_LEFT)
            .size((Ab(SLOT_SIZE), Ab(SLOT_SIZE)))
            .pack(),
        UiDepth::Set(5.0),
        Sprite::from_color(Color::srgba(0.055, 0.05, 0.075, 0.92), Vec2::ONE),
        RenderLayers::layer(INVENTORY_RENDER_LAYER),
        Pickable::IGNORE,
    ));
}

fn spawn_rect(
    ui: &mut ChildSpawnerCommands,
    name: &'static str,
    layout: UiLayout,
    depth: f32,
    color: Color,
) {
    ui.spawn((
        Name::new(name),
        layout,
        UiDepth::Set(depth),
        Sprite::from_color(color, Vec2::ONE),
        RenderLayers::layer(INVENTORY_RENDER_LAYER),
        Pickable::IGNORE,
    ));
}

fn spawn_text(
    ui: &mut ChildSpawnerCommands,
    name: &'static str,
    text: impl Into<String>,
    layout: UiLayout,
    depth: f32,
    font_size: f32,
    color: Color,
) {
    ui.spawn((
        Name::new(name),
        layout,
        UiTextSize::from(Ab(font_size)),
        UiDepth::Set(depth),
        Text2d::new(text),
        TextFont::from_font_size(font_size),
        TextColor(color),
        RenderLayers::layer(INVENTORY_RENDER_LAYER),
        Pickable::IGNORE,
    ));
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
    mut items: Query<&mut Transform, With<InventoryItemUi>>,
) {
    let Ok(mut transform) = items.get_mut(drag.event_target()) else {
        return;
    };

    transform.translation.x += drag.delta.x;
    transform.translation.y -= drag.delta.y;
    transform.translation.z = 30.0;
}

fn start_inventory_drag(
    drag: On<Pointer<DragStart>>,
    mut items: Query<&mut Pickable, With<InventoryItemUi>>,
) {
    let Ok(mut pickable) = items.get_mut(drag.event_target()) else {
        return;
    };

    pickable.is_hoverable = false;
    pickable.should_block_lower = false;
}

fn finish_inventory_drag(
    drag: On<Pointer<DragEnd>>,
    mut drag_state: ResMut<InventoryDragState>,
    mut items: Query<&mut Pickable, With<InventoryItemUi>>,
    mut commands: Commands,
) {
    if drag_state.successful_drop {
        drag_state.successful_drop = false;
        return;
    }

    if let Ok(mut pickable) = items.get_mut(drag.event_target()) {
        *pickable = Pickable::default();
    }

    commands.trigger(RecomputeUiLayout);
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

    let moved = match (item.kind, target.kind) {
        (InventoryKind::Run, InventoryKind::Safe) => {
            move_run_item_to_safe(&mut run_inventory, &mut safe_inventory, item.index)
        }
        (InventoryKind::Safe, InventoryKind::Run) => {
            move_safe_item_to_run(&mut run_inventory, &mut safe_inventory, item.index)
        }
        _ => false,
    };

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

    let moved = match item.kind {
        InventoryKind::Run => {
            move_run_item_to_safe(&mut run_inventory, &mut safe_inventory, item.index)
        }
        InventoryKind::Safe => {
            move_safe_item_to_run(&mut run_inventory, &mut safe_inventory, item.index)
        }
    };

    click.propagate(false);

    if moved {
        commands.insert_resource(InventoryUiDirty);
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
