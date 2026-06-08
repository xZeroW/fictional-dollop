mod item_transfer_menu;
mod tooltip;

use bevy::prelude::*;

use crate::components::ItemRarity;

use super::InventoryKind;

pub(super) const INVENTORY_UI_Z_INDEX: i32 = 30;
pub(super) const DRAGGED_ITEM_GLOBAL_Z_INDEX: i32 = INVENTORY_UI_Z_INDEX + 1;
pub(super) const INVENTORY_RENDER_LAYER: usize = 2;
pub(super) const ITEM_Z_INDEX: i32 = 1;
const ITEM_ICON_Z_INDEX: i32 = 2;
const TOOLTIP_Z_INDEX: i32 = 20;
pub(super) const DRAGGED_ITEM_LOCAL_Z_INDEX: i32 = 30;

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
const TOOLTIP_WIDTH: f32 = 320.0;
const TOOLTIP_HEIGHT: f32 = 320.0;
const TOOLTIP_GAP: f32 = 12.0;
const TOOLTIP_PANEL_MARGIN: f32 = 12.0;
const TOOLTIP_FRAME_PADDING: f32 = 4.0;
const TOOLTIP_CARD_PADDING: f32 = 12.0;
const TOOLTIP_SECTION_GAP: f32 = 8.0;
const TOOLTIP_BODY_GAP: f32 = 12.0;
const TOOLTIP_STAT_GAP: f32 = 4.0;
const TOOLTIP_ICON_SIZE: f32 = 54.0;
const TOOLTIP_ICON_IMAGE_SIZE: f32 = 32.0;
const TOOLTIP_ICON_BACKING_INSET: f32 = 6.0;
const TOOLTIP_TITLE_FONT_SIZE: f32 = 24.0;
const TOOLTIP_RARITY_FONT_SIZE: f32 = 19.0;
const TOOLTIP_STAT_FONT_SIZE: f32 = 18.0;
const TOOLTIP_DEBUG_FONT_SIZE: f32 = 15.0;

const PANEL_COLOR: Color = Color::srgba(0.05, 0.045, 0.075, 0.96);
const PANEL_FRAME_COLOR: Color = Color::srgba(0.14, 0.11, 0.18, 0.98);
const OVERLAY_COLOR: Color = Color::srgba(0.02, 0.0, 0.03, 0.88);
const SLOT_COLOR: Color = Color::srgba(0.08, 0.075, 0.105, 0.95);
const EMPTY_SLOT_COLOR: Color = Color::srgba(0.055, 0.05, 0.075, 0.92);
const DROP_PANEL_COLOR: Color = Color::srgba(0.025, 0.022, 0.035, 0.95);
const TOOLTIP_COLOR: Color = Color::srgba(0.015, 0.014, 0.022, 0.98);
const TOOLTIP_FRAME_COLOR: Color = Color::srgba(0.42, 0.35, 0.27, 0.98);
const TOOLTIP_SEPARATOR_COLOR: Color = Color::srgba(0.32, 0.3, 0.32, 0.95);
const TEXT_COLOR: Color = Color::srgb(0.9, 0.86, 0.78);
const MUTED_TEXT_COLOR: Color = Color::srgb(0.62, 0.58, 0.65);
const BUTTON_COLOR: Color = Color::srgb(0.33, 0.21, 0.46);

pub(super) use item_transfer_menu::spawn_item_transfer_menu_root;
pub(super) use tooltip::{
    InventoryItemTooltip, InventoryTooltipData, despawn_inventory_tooltips,
    spawn_inventory_item_tooltip,
};

#[derive(Component)]
pub(super) struct InventoryPanelUi;

#[derive(Component)]
pub(super) struct InventoryItemUi {
    pub(in crate::menus::inventory) kind: InventoryKind,
    pub(in crate::menus::inventory) index: usize,
    pub(in crate::menus::inventory) panel_pos: Vec2,
    pub(in crate::menus::inventory) drag_offset: Vec2,
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
