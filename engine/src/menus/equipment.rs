//! The between-wave equipment and crafting menu.

use bevy::prelude::*;

use crate::{
    Pause,
    assets::WeaponAssets,
    components::{Health, Player, Weapon},
    game::weapon_data::{Weapons, WeaponsHandle},
    menus::{
        Menu, MenuCamera,
        item_tooltip::{ItemTooltip, ItemTooltipData, despawn_item_tooltips, spawn_item_tooltip},
    },
    systems::{
        CraftingMaterial, CraftingMaterials, Equipment, EquipmentSlot, InventoryItem,
        SAFE_INVENTORY_CAPACITY, SafeInventory, move_equipment_item_to_safe,
        move_safe_item_to_equipment,
    },
};

const EQUIPMENT_UI_Z_INDEX: i32 = 30;
const DRAGGED_ITEM_GLOBAL_Z_INDEX: i32 = EQUIPMENT_UI_Z_INDEX + 1;
const ITEM_Z_INDEX: i32 = 1;
const ITEM_ICON_Z_INDEX: i32 = 2;
const DRAGGED_ITEM_LOCAL_Z_INDEX: i32 = 30;

const PANEL_WIDTH: f32 = 1260.0;
const PANEL_HEIGHT: f32 = 650.0;
const PANEL_FRAME_PADDING: f32 = 10.0;
const TITLE_POS: (f32, f32) = (30.0, 28.0);
const HELP_POS: (f32, f32) = (30.0, 68.0);
const STATS_PANEL_POS: (f32, f32) = (30.0, 112.0);
const STATS_PANEL_SIZE: (f32, f32) = (280.0, 430.0);
const CRAFT_PANEL_POS: (f32, f32) = (340.0, 112.0);
const CRAFT_PANEL_SIZE: (f32, f32) = (430.0, 430.0);
const EQUIPMENT_PANEL_POS: (f32, f32) = (800.0, 112.0);
const EQUIPMENT_PANEL_SIZE: (f32, f32) = (430.0, 150.0);
const SAFE_PANEL_POS: (f32, f32) = (800.0, 282.0);
const SAFE_PANEL_SIZE: (f32, f32) = (430.0, 260.0);
const SECTION_TITLE_POS: (f32, f32) = (16.0, 20.0);
const SLOT_SIZE: f32 = 48.0;
const ITEM_SIZE: f32 = 16.0;
const SLOT_GAP: f32 = 6.0;
const SAFE_SLOT_COLUMNS: usize = 7;
const SAFE_SLOT_ORIGIN: (f32, f32) = (18.0, 62.0);
const MAIN_HAND_SLOT_POS: (f32, f32) = (24.0, 72.0);
const CONTINUE_BUTTON_WIDTH: f32 = 220.0;
const CONTINUE_BUTTON_HEIGHT: f32 = 56.0;
const CONTINUE_BUTTON_CENTER_X: f32 = 630.0;
const CONTINUE_BUTTON_CENTER_Y: f32 = 605.0;

const PANEL_COLOR: Color = Color::srgba(0.05, 0.045, 0.075, 0.96);
const PANEL_FRAME_COLOR: Color = Color::srgba(0.14, 0.11, 0.18, 0.98);
const OVERLAY_COLOR: Color = Color::srgba(0.02, 0.0, 0.03, 0.88);
const CARD_COLOR: Color = Color::srgba(0.025, 0.022, 0.035, 0.95);
const SLOT_COLOR: Color = Color::srgba(0.08, 0.075, 0.105, 0.95);
const EMPTY_SLOT_COLOR: Color = Color::srgba(0.055, 0.05, 0.075, 0.92);
const CRAFT_CARD_COLOR: Color = Color::srgba(0.09, 0.075, 0.12, 0.96);
const CRAFT_CARD_SELECTED_COLOR: Color = Color::srgba(0.25, 0.16, 0.34, 0.98);
const CRAFT_PREVIEW_COLOR: Color = Color::srgba(0.055, 0.048, 0.075, 0.98);
const DISABLED_BUTTON_COLOR: Color = Color::srgba(0.16, 0.15, 0.18, 0.95);
const TEXT_COLOR: Color = Color::srgb(0.9, 0.86, 0.78);
const MUTED_TEXT_COLOR: Color = Color::srgb(0.62, 0.58, 0.65);
const BUTTON_COLOR: Color = Color::srgb(0.33, 0.21, 0.46);

#[derive(Component)]
struct EquipmentMenuRoot;

#[derive(Component)]
struct EquipmentPanelUi;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
enum EquipmentItemKind {
    Safe,
    Equipment(EquipmentSlot),
}

#[derive(Component)]
struct EquipmentDropTarget {
    kind: EquipmentItemKind,
}

#[derive(Component)]
struct EquipmentItemUi {
    kind: EquipmentItemKind,
    index: usize,
    panel_pos: Vec2,
    drag_offset: Vec2,
}

#[derive(Component)]
struct CraftingActionUi {
    action: CraftingAction,
}

#[derive(Component)]
struct CraftingConfirmButtonUi;

#[derive(Resource)]
struct EquipmentMenuDirty;

#[derive(Resource, Default)]
struct EquipmentDragState {
    primary_drag: bool,
    successful_drop: bool,
}

#[derive(Resource)]
struct CraftingSelection {
    item_index: Option<usize>,
    action: CraftingAction,
}

impl Default for CraftingSelection {
    fn default() -> Self {
        Self {
            item_index: None,
            action: CraftingAction::Improve,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CraftingAction {
    Improve,
    Add,
    Reroll,
    Guarantee,
}

impl CraftingAction {
    const ALL: [Self; 4] = [Self::Improve, Self::Add, Self::Reroll, Self::Guarantee];

    fn title(self) -> &'static str {
        match self {
            Self::Improve => "Improve",
            Self::Add => "Add",
            Self::Reroll => "Reroll",
            Self::Guarantee => "Guarantee",
        }
    }

    fn material(self) -> CraftingMaterial {
        match self {
            Self::Improve => CraftingMaterial::Whetstone,
            Self::Add => CraftingMaterial::Ember,
            Self::Reroll => CraftingMaterial::ReforgeOre,
            Self::Guarantee => CraftingMaterial::Essence,
        }
    }

    fn description(self) -> &'static str {
        match self {
            Self::Improve => "Safe quality upgrade",
            Self::Add => "Add a random affix",
            Self::Reroll => "Replace one affix",
            Self::Guarantee => "Force an affix family",
        }
    }

    fn preview(self) -> &'static str {
        match self {
            Self::Improve => "Quality +2% up to tier cap.",
            Self::Add => "Rolls one random affix if a slot is open.",
            Self::Reroll => "Replaces one affix.",
            Self::Guarantee => "Essence guarantees attack speed.",
        }
    }

    fn unavailable_reason(
        self,
        item: &InventoryItem,
        materials: &CraftingMaterials,
    ) -> Option<&'static str> {
        if !materials.can_spend(self.material(), 1) {
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

pub(super) struct EquipmentMenuPlugin;

impl Plugin for EquipmentMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EquipmentDragState>();
        app.init_resource::<CraftingSelection>();
        app.add_systems(OnEnter(Menu::Equipment), spawn_equipment_menu);
        app.add_systems(
            Update,
            refresh_equipment_menu.run_if(in_state(Menu::Equipment)),
        );
    }
}

fn spawn_equipment_menu(
    mut commands: Commands,
    camera: Query<Entity, With<MenuCamera>>,
    safe_inventory: Res<SafeInventory>,
    equipment: Res<Equipment>,
    crafting_selection: Res<CraftingSelection>,
    crafting_materials: Res<CraftingMaterials>,
    weapon_assets: Res<WeaponAssets>,
    weapons_handle: Res<WeaponsHandle>,
    weapons_assets: Res<Assets<Weapons>>,
    player: Query<(&Health, &Weapon), With<Player>>,
) {
    let Ok(camera) = camera.single() else {
        return;
    };

    let weapons = weapons_assets.get(&weapons_handle.0);
    spawn_equipment_menu_root(
        &mut commands,
        camera,
        &safe_inventory,
        &equipment,
        &crafting_selection,
        &crafting_materials,
        &weapon_assets,
        weapons,
        player.iter().next(),
    );
}

fn refresh_equipment_menu(
    mut commands: Commands,
    dirty: Option<Res<EquipmentMenuDirty>>,
    roots: Query<Entity, With<EquipmentMenuRoot>>,
    camera: Query<Entity, With<MenuCamera>>,
    safe_inventory: Res<SafeInventory>,
    equipment: Res<Equipment>,
    crafting_selection: Res<CraftingSelection>,
    crafting_materials: Res<CraftingMaterials>,
    weapon_assets: Res<WeaponAssets>,
    weapons_handle: Res<WeaponsHandle>,
    weapons_assets: Res<Assets<Weapons>>,
    player: Query<(&Health, &Weapon), With<Player>>,
) {
    if dirty.is_none() {
        return;
    }

    commands.remove_resource::<EquipmentMenuDirty>();

    for root in roots.iter() {
        commands.entity(root).despawn();
    }

    let Ok(camera) = camera.single() else {
        return;
    };

    let weapons = weapons_assets.get(&weapons_handle.0);
    spawn_equipment_menu_root(
        &mut commands,
        camera,
        &safe_inventory,
        &equipment,
        &crafting_selection,
        &crafting_materials,
        &weapon_assets,
        weapons,
        player.iter().next(),
    );
}

fn spawn_equipment_menu_root(
    commands: &mut Commands,
    camera: Entity,
    safe_inventory: &SafeInventory,
    equipment: &Equipment,
    crafting_selection: &CraftingSelection,
    crafting_materials: &CraftingMaterials,
    weapon_assets: &WeaponAssets,
    weapons: Option<&Weapons>,
    player: Option<(&Health, &Weapon)>,
) -> Entity {
    commands
        .spawn((
            Name::new("Equipment Menu"),
            EquipmentMenuRoot,
            Node {
                position_type: PositionType::Absolute,
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            UiTargetCamera(camera),
            GlobalZIndex(EQUIPMENT_UI_Z_INDEX),
            BackgroundColor(OVERLAY_COLOR),
            Pickable {
                should_block_lower: true,
                is_hoverable: false,
            },
            DespawnOnExit(Menu::Equipment),
        ))
        .with_children(|ui| {
            ui.spawn((
                Name::new("Equipment Panel Frame"),
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
                    Name::new("Equipment Panel"),
                    EquipmentPanelUi,
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
                    spawn_text(
                        ui,
                        "Equipment Title",
                        "Character",
                        TITLE_POS.0,
                        TITLE_POS.1,
                        38.0,
                        TEXT_COLOR,
                    );
                    spawn_text(
                        ui,
                        "Equipment Help",
                        "Craft safe weapons, equip a main hand, then continue to the next wave.",
                        HELP_POS.0,
                        HELP_POS.1,
                        20.0,
                        MUTED_TEXT_COLOR,
                    );
                    spawn_stats_panel(ui, equipment, player);
                    spawn_crafting_panel(
                        ui,
                        safe_inventory,
                        crafting_selection,
                        crafting_materials,
                        weapon_assets,
                        weapons,
                    );
                    spawn_equipment_panel(ui, equipment, weapon_assets, weapons);
                    spawn_safe_inventory_panel(ui, safe_inventory, weapon_assets, weapons);
                    spawn_continue_button(ui);
                });
            });
        })
        .id()
}

fn spawn_stats_panel(
    ui: &mut ChildSpawnerCommands,
    equipment: &Equipment,
    player: Option<(&Health, &Weapon)>,
) {
    let stats = equipped_attribute_totals(equipment);
    let (health_text, weapon_text) = player
        .map(|(health, weapon)| {
            (
                format!("Health {:.0} / {:.0}", health.current, health.max),
                format!("Weapon {}", weapon.key),
            )
        })
        .unwrap_or(("Health -- / --".to_string(), "Weapon --".to_string()));

    ui.spawn((
        Name::new("Character Stats Panel"),
        absolute_node(
            STATS_PANEL_POS.0,
            STATS_PANEL_POS.1,
            STATS_PANEL_SIZE.0,
            STATS_PANEL_SIZE.1,
        ),
        BackgroundColor(CARD_COLOR),
        Pickable::IGNORE,
    ))
    .with_children(|ui| {
        spawn_text(
            ui,
            "Stats Title",
            "Stats",
            SECTION_TITLE_POS.0,
            SECTION_TITLE_POS.1,
            22.0,
            TEXT_COLOR,
        );
        spawn_text(
            ui,
            "Stats Health",
            health_text,
            18.0,
            70.0,
            18.0,
            TEXT_COLOR,
        );
        spawn_text(
            ui,
            "Stats Weapon",
            weapon_text,
            18.0,
            100.0,
            16.0,
            MUTED_TEXT_COLOR,
        );
        spawn_text(
            ui,
            "Stats Strength",
            format!("Strength {:.0}", stats.strength),
            18.0,
            150.0,
            18.0,
            TEXT_COLOR,
        );
        spawn_text(
            ui,
            "Stats Dexterity",
            format!("Dexterity {:.0}", stats.dexterity),
            18.0,
            182.0,
            18.0,
            TEXT_COLOR,
        );
        spawn_text(
            ui,
            "Stats Intelligence",
            format!("Intelligence {:.0}", stats.intelligence),
            18.0,
            214.0,
            18.0,
            TEXT_COLOR,
        );
        spawn_text(
            ui,
            "Stats Vitality",
            format!("Vitality {:.0}", stats.vitality),
            18.0,
            246.0,
            18.0,
            TEXT_COLOR,
        );
        spawn_text(
            ui,
            "Stats Damage Bonus",
            format!("Attack damage +{:.0}%", stats.strength),
            18.0,
            300.0,
            16.0,
            MUTED_TEXT_COLOR,
        );
        spawn_text(
            ui,
            "Stats Health Bonus",
            format!("Max health +{:.0}", stats.vitality * 10.0),
            18.0,
            326.0,
            16.0,
            MUTED_TEXT_COLOR,
        );
    });
}

fn spawn_crafting_panel(
    ui: &mut ChildSpawnerCommands,
    safe_inventory: &SafeInventory,
    crafting_selection: &CraftingSelection,
    crafting_materials: &CraftingMaterials,
    weapon_assets: &WeaponAssets,
    weapons: Option<&Weapons>,
) {
    let selected_item = crafting_selection
        .item_index
        .and_then(|index| safe_inventory.items().get(index).map(|item| (index, item)));

    ui.spawn((
        Name::new("Crafting Workbench"),
        absolute_node(
            CRAFT_PANEL_POS.0,
            CRAFT_PANEL_POS.1,
            CRAFT_PANEL_SIZE.0,
            CRAFT_PANEL_SIZE.1,
        ),
        BackgroundColor(CARD_COLOR),
        Pickable::default(),
    ))
    .with_children(|ui| {
        spawn_text(
            ui,
            "Crafting Title",
            "Crafting",
            SECTION_TITLE_POS.0,
            SECTION_TITLE_POS.1,
            22.0,
            TEXT_COLOR,
        );
        spawn_selected_item_card(ui, selected_item, weapon_assets, weapons);
        spawn_text(
            ui,
            "Crafting Actions Title",
            "Materials",
            18.0,
            158.0,
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
    selected_item: Option<(usize, &InventoryItem)>,
    weapon_assets: &WeaponAssets,
    weapons: Option<&Weapons>,
) {
    ui.spawn((
        Name::new("Crafting Selected Item"),
        absolute_node(18.0, 58.0, 394.0, 86.0),
        BackgroundColor(CRAFT_PREVIEW_COLOR),
        Pickable::IGNORE,
    ))
    .with_children(|ui| {
        if let Some((index, item)) = selected_item {
            spawn_item_slot(
                ui,
                EquipmentItemKind::Safe,
                index,
                item,
                Vec2::new(
                    CRAFT_PANEL_POS.0 + 18.0 + 173.0,
                    CRAFT_PANEL_POS.1 + 58.0 + 26.0,
                ),
                173.0,
                26.0,
                weapon_assets,
                weapons,
            );
            spawn_text(
                ui,
                "Crafting Selected Hint",
                "Selected safe weapon",
                12.0,
                8.0,
                15.0,
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
                "Left click a safe inventory weapon.",
                12.0,
                48.0,
                15.0,
                MUTED_TEXT_COLOR,
            );
        }
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
        let x = 18.0 + col as f32 * 200.0;
        let y = 188.0 + row as f32 * 68.0;
        let color = if selected_action == action {
            CRAFT_CARD_SELECTED_COLOR
        } else {
            CRAFT_CARD_COLOR
        };

        ui.spawn((
            Name::new(format!("{} Crafting Action", action.title())),
            Button,
            CraftingActionUi { action },
            absolute_node(x, y, 182.0, 56.0),
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
                    action.material().label(),
                    crafting_materials.amount(action.material())
                ),
                8.0,
                30.0,
                13.0,
                MUTED_TEXT_COLOR,
            );
        });
    }
}

fn spawn_crafting_preview(
    ui: &mut ChildSpawnerCommands,
    selected_item: Option<(usize, &InventoryItem)>,
    action: CraftingAction,
) {
    ui.spawn((
        Name::new("Crafting Preview"),
        absolute_node(18.0, 320.0, 394.0, 62.0),
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
    selected_item: Option<(usize, &InventoryItem)>,
    action: CraftingAction,
    crafting_materials: &CraftingMaterials,
) {
    let unavailable_reason = selected_item
        .map(|(_, item)| action.unavailable_reason(item, crafting_materials))
        .unwrap_or(Some("Select a weapon"));

    if let Some(reason) = unavailable_reason {
        ui.spawn((
            Name::new("Crafting Disabled Button"),
            absolute_node(18.0, 394.0, 394.0, 30.0),
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
        absolute_node(18.0, 394.0, 394.0, 30.0),
        BackgroundColor(BUTTON_COLOR),
        Pickable::default(),
    ))
    .observe(confirm_crafting_action)
    .with_children(|ui| {
        spawn_text(
            ui,
            "Crafting Confirm Button Text",
            "Craft",
            178.0,
            5.0,
            16.0,
            TEXT_COLOR,
        );
    });
}

fn spawn_equipment_panel(
    ui: &mut ChildSpawnerCommands,
    equipment: &Equipment,
    weapon_assets: &WeaponAssets,
    weapons: Option<&Weapons>,
) {
    let kind = EquipmentItemKind::Equipment(EquipmentSlot::MainHand);
    ui.spawn((
        Name::new("Main Hand Panel"),
        EquipmentDropTarget { kind },
        absolute_node(
            EQUIPMENT_PANEL_POS.0,
            EQUIPMENT_PANEL_POS.1,
            EQUIPMENT_PANEL_SIZE.0,
            EQUIPMENT_PANEL_SIZE.1,
        ),
        BackgroundColor(CARD_COLOR),
        Pickable::default(),
    ))
    .observe(drop_equipment_item)
    .with_children(|ui| {
        spawn_text(
            ui,
            "Equipment Section Title",
            "Equipment",
            SECTION_TITLE_POS.0,
            SECTION_TITLE_POS.1,
            22.0,
            TEXT_COLOR,
        );
        spawn_text(
            ui,
            "Main Hand Label",
            "Main hand",
            92.0,
            85.0,
            18.0,
            MUTED_TEXT_COLOR,
        );

        if let Some(item) = equipment.item(EquipmentSlot::MainHand) {
            spawn_item_slot(
                ui,
                kind,
                0,
                item,
                Vec2::new(
                    EQUIPMENT_PANEL_POS.0 + MAIN_HAND_SLOT_POS.0,
                    EQUIPMENT_PANEL_POS.1 + MAIN_HAND_SLOT_POS.1,
                ),
                MAIN_HAND_SLOT_POS.0,
                MAIN_HAND_SLOT_POS.1,
                weapon_assets,
                weapons,
            );
        } else {
            spawn_empty_slot(ui, MAIN_HAND_SLOT_POS.0, MAIN_HAND_SLOT_POS.1);
        }
    });
}

fn spawn_safe_inventory_panel(
    ui: &mut ChildSpawnerCommands,
    safe_inventory: &SafeInventory,
    weapon_assets: &WeaponAssets,
    weapons: Option<&Weapons>,
) {
    ui.spawn((
        Name::new("Safe Inventory Panel"),
        EquipmentDropTarget {
            kind: EquipmentItemKind::Safe,
        },
        absolute_node(
            SAFE_PANEL_POS.0,
            SAFE_PANEL_POS.1,
            SAFE_PANEL_SIZE.0,
            SAFE_PANEL_SIZE.1,
        ),
        BackgroundColor(CARD_COLOR),
        Pickable::default(),
    ))
    .observe(drop_equipment_item)
    .with_children(|ui| {
        spawn_text(
            ui,
            "Safe Inventory Title",
            format!(
                "Safe Inventory ({}/{SAFE_INVENTORY_CAPACITY})",
                safe_inventory.items().len()
            ),
            SECTION_TITLE_POS.0,
            SECTION_TITLE_POS.1,
            22.0,
            TEXT_COLOR,
        );

        for index in 0..SAFE_INVENTORY_CAPACITY {
            let col = index % SAFE_SLOT_COLUMNS;
            let row = index / SAFE_SLOT_COLUMNS;
            let x = SAFE_SLOT_ORIGIN.0 + col as f32 * (SLOT_SIZE + SLOT_GAP);
            let y = SAFE_SLOT_ORIGIN.1 + row as f32 * (SLOT_SIZE + SLOT_GAP);

            if let Some(item) = safe_inventory.items().get(index) {
                spawn_item_slot(
                    ui,
                    EquipmentItemKind::Safe,
                    index,
                    item,
                    Vec2::new(SAFE_PANEL_POS.0 + x, SAFE_PANEL_POS.1 + y),
                    x,
                    y,
                    weapon_assets,
                    weapons,
                );
            } else {
                spawn_empty_slot(ui, x, y);
            }
        }
    });
}

fn spawn_item_slot(
    ui: &mut ChildSpawnerCommands,
    kind: EquipmentItemKind,
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
        Name::new(format!("Equipment Item Slot {}", index + 1)),
        EquipmentItemUi {
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
    .observe(show_equipment_item_tooltip)
    .observe(hide_equipment_item_tooltip)
    .observe(shortcut_equipment_item)
    .observe(start_equipment_drag)
    .observe(drag_equipment_item)
    .observe(finish_equipment_drag)
    .with_children(|ui| {
        ui.spawn((
            Name::new("Equipment Slot Backing"),
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
        Name::new("Empty Equipment Slot"),
        absolute_node(x, y, SLOT_SIZE, SLOT_SIZE),
        BackgroundColor(EMPTY_SLOT_COLOR),
        Pickable::IGNORE,
    ));
}

fn spawn_continue_button(ui: &mut ChildSpawnerCommands) {
    ui.spawn((
        Name::new("Equipment Continue Button"),
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
    .observe(continue_to_next_wave)
    .with_children(|ui| {
        ui.spawn((
            Name::new("Equipment Continue Text"),
            Text("Continue".to_string()),
            TextFont::from_font_size(26.0),
            TextColor(TEXT_COLOR),
            Pickable::IGNORE,
        ));
    });
}

fn select_crafting_item(
    mut click: On<Pointer<Click>>,
    items: Query<&EquipmentItemUi>,
    mut crafting_selection: ResMut<CraftingSelection>,
    mut commands: Commands,
) {
    if click.button != PointerButton::Primary {
        return;
    }

    let Ok(item) = items.get(click.event_target()) else {
        return;
    };
    if item.kind != EquipmentItemKind::Safe {
        return;
    }

    crafting_selection.item_index = Some(item.index);
    click.propagate(false);
    commands.insert_resource(EquipmentMenuDirty);
}

fn select_crafting_action(
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
    commands.insert_resource(EquipmentMenuDirty);
}

fn confirm_crafting_action(
    mut click: On<Pointer<Click>>,
    buttons: Query<&CraftingConfirmButtonUi>,
    mut safe_inventory: ResMut<SafeInventory>,
    mut crafting_materials: ResMut<CraftingMaterials>,
    crafting_selection: Res<CraftingSelection>,
    mut commands: Commands,
) {
    if click.button != PointerButton::Primary || buttons.get(click.event_target()).is_err() {
        return;
    }

    let Some(index) = crafting_selection.item_index else {
        return;
    };
    let action = crafting_selection.action;
    let Some(item) = safe_inventory.item_mut(index) else {
        return;
    };

    if action
        .unavailable_reason(item, &crafting_materials)
        .is_some()
        || !action.apply(item)
    {
        return;
    }
    crafting_materials.spend(action.material(), 1);

    click.propagate(false);
    commands.insert_resource(EquipmentMenuDirty);
}

fn show_equipment_item_tooltip(
    over: On<Pointer<Over>>,
    mut commands: Commands,
    items: Query<&EquipmentItemUi>,
    panels: Query<Entity, With<EquipmentPanelUi>>,
    tooltips: Query<Entity, With<ItemTooltip>>,
    safe_inventory: Res<SafeInventory>,
    equipment: Res<Equipment>,
    weapon_assets: Res<WeaponAssets>,
    weapons_handle: Res<WeaponsHandle>,
    weapons_assets: Res<Assets<Weapons>>,
) {
    let Ok(item_ui) = items.get(over.event_target()) else {
        return;
    };
    let Some(item) = equipment_menu_item(item_ui.kind, item_ui.index, &safe_inventory, &equipment)
    else {
        return;
    };
    let Ok(panel) = panels.single() else {
        return;
    };

    let weapons = weapons_assets.get(&weapons_handle.0);
    let tooltip_data = ItemTooltipData::new(item, weapons);

    despawn_item_tooltips(&mut commands, &tooltips);
    commands.entity(panel).with_children(|ui| {
        spawn_item_tooltip(ui, item_ui.panel_pos, &tooltip_data, &weapon_assets);
    });
}

fn hide_equipment_item_tooltip(
    _: On<Pointer<Out>>,
    mut commands: Commands,
    tooltips: Query<Entity, With<ItemTooltip>>,
) {
    despawn_item_tooltips(&mut commands, &tooltips);
}

fn equipment_menu_item<'a>(
    kind: EquipmentItemKind,
    index: usize,
    safe_inventory: &'a SafeInventory,
    equipment: &'a Equipment,
) -> Option<&'a InventoryItem> {
    match kind {
        EquipmentItemKind::Safe => safe_inventory.items().get(index),
        EquipmentItemKind::Equipment(slot) => equipment.item(slot),
    }
}

fn drag_equipment_item(
    drag: On<Pointer<Drag>>,
    drag_state: Res<EquipmentDragState>,
    mut items: Query<(&mut UiTransform, &mut EquipmentItemUi)>,
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

fn start_equipment_drag(
    drag: On<Pointer<DragStart>>,
    mut drag_state: ResMut<EquipmentDragState>,
    mut items: Query<(&mut Pickable, &mut ZIndex), With<EquipmentItemUi>>,
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

fn finish_equipment_drag(
    drag: On<Pointer<DragEnd>>,
    mut drag_state: ResMut<EquipmentDragState>,
    mut commands: Commands,
    mut items: Query<(
        &mut Pickable,
        &mut UiTransform,
        &mut ZIndex,
        &mut EquipmentItemUi,
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

fn drop_equipment_item(
    mut drop: On<Pointer<DragDrop>>,
    targets: Query<&EquipmentDropTarget>,
    items: Query<&EquipmentItemUi>,
    mut safe_inventory: ResMut<SafeInventory>,
    mut equipment: ResMut<Equipment>,
    mut crafting_selection: ResMut<CraftingSelection>,
    mut drag_state: ResMut<EquipmentDragState>,
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

    let moved = move_equipment_menu_item(
        item.kind,
        target.kind,
        item.index,
        &mut safe_inventory,
        &mut equipment,
    );

    drop.propagate(false);

    if moved {
        crafting_selection.item_index = None;
        drag_state.successful_drop = true;
        commands.insert_resource(EquipmentMenuDirty);
    }
}

fn shortcut_equipment_item(
    mut click: On<Pointer<Click>>,
    items: Query<&EquipmentItemUi>,
    mut safe_inventory: ResMut<SafeInventory>,
    mut equipment: ResMut<Equipment>,
    mut crafting_selection: ResMut<CraftingSelection>,
    mut commands: Commands,
) {
    if click.button != PointerButton::Secondary {
        return;
    }

    let Ok(item) = items.get(click.event_target()) else {
        return;
    };
    let target = match item.kind {
        EquipmentItemKind::Safe => EquipmentItemKind::Equipment(EquipmentSlot::MainHand),
        EquipmentItemKind::Equipment(_) => EquipmentItemKind::Safe,
    };

    let moved = move_equipment_menu_item(
        item.kind,
        target,
        item.index,
        &mut safe_inventory,
        &mut equipment,
    );

    click.propagate(false);

    if moved {
        crafting_selection.item_index = None;
        commands.insert_resource(EquipmentMenuDirty);
    }
}

fn move_equipment_menu_item(
    source: EquipmentItemKind,
    target: EquipmentItemKind,
    index: usize,
    safe_inventory: &mut SafeInventory,
    equipment: &mut Equipment,
) -> bool {
    match (source, target) {
        (EquipmentItemKind::Safe, EquipmentItemKind::Equipment(slot)) => {
            move_safe_item_to_equipment(safe_inventory, equipment, index, slot)
        }
        (EquipmentItemKind::Equipment(slot), EquipmentItemKind::Safe) => {
            move_equipment_item_to_safe(equipment, safe_inventory, slot)
        }
        _ => false,
    }
}

fn continue_to_next_wave(
    _: On<Pointer<Click>>,
    mut next_menu: ResMut<NextState<Menu>>,
    mut next_pause: ResMut<NextState<Pause>>,
) {
    next_menu.set(Menu::None);
    next_pause.set(Pause(false));
}

#[derive(Default)]
struct EquippedAttributeTotals {
    strength: f32,
    dexterity: f32,
    intelligence: f32,
    vitality: f32,
}

fn equipped_attribute_totals(equipment: &Equipment) -> EquippedAttributeTotals {
    let mut stats = EquippedAttributeTotals::default();

    if let Some(item) = equipment.item(EquipmentSlot::MainHand) {
        for affix in &item.affixes {
            let Some((attribute, value)) = affix.attribute_modifier() else {
                continue;
            };

            match attribute {
                "Strength" => stats.strength += value,
                "Dexterity" => stats.dexterity += value,
                "Intelligence" => stats.intelligence += value,
                "Vitality" => stats.vitality += value,
                _ => {}
            }
        }
    }

    stats
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

fn rarity_color(rarity: crate::components::ItemRarity) -> Color {
    match rarity {
        crate::components::ItemRarity::Common => Color::srgb(0.48, 0.5, 0.54),
        crate::components::ItemRarity::Uncommon => Color::srgb(0.24, 0.66, 0.31),
        crate::components::ItemRarity::Rare => Color::srgb(0.22, 0.42, 0.9),
        crate::components::ItemRarity::Epic => Color::srgb(0.58, 0.27, 0.86),
        crate::components::ItemRarity::Legendary => Color::srgb(0.96, 0.58, 0.17),
        crate::components::ItemRarity::Mythic => Color::srgb(0.92, 0.25, 0.32),
    }
}
