use bevy::prelude::*;

use crate::{
    assets::WeaponAssets,
    game::weapon_data::Weapons,
    systems::{CraftingAffix, InventoryItem},
};

const PANEL_WIDTH: f32 = 1260.0;
const PANEL_HEIGHT: f32 = 570.0;
const SLOT_SIZE: f32 = 48.0;
const TOOLTIP_Z_INDEX: i32 = 20;
const TOOLTIP_WIDTH: f32 = 320.0;
const TOOLTIP_HEIGHT: f32 = 370.0;
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

const SLOT_COLOR: Color = Color::srgba(0.08, 0.075, 0.105, 0.95);
const TOOLTIP_COLOR: Color = Color::srgba(0.015, 0.014, 0.022, 0.98);
const TOOLTIP_FRAME_COLOR: Color = Color::srgba(0.42, 0.35, 0.27, 0.98);
const TOOLTIP_SEPARATOR_COLOR: Color = Color::srgba(0.32, 0.3, 0.32, 0.95);
const TEXT_COLOR: Color = Color::srgb(0.9, 0.86, 0.78);
const MUTED_TEXT_COLOR: Color = Color::srgb(0.62, 0.58, 0.65);

#[derive(Component)]
pub(in crate::menus) struct ItemTooltip;

pub(in crate::menus) struct ItemTooltipData<'a> {
    item_id: &'a str,
    name: &'a str,
    rarity: crate::components::ItemRarity,
    quality: u8,
    affixes: &'a [CraftingAffix],
    damage: Option<f32>,
    attack_speed: Option<f32>,
    attack_range: Option<f32>,
    weapon_sprite_index: Option<usize>,
}

impl<'a> ItemTooltipData<'a> {
    pub(in crate::menus) fn new(item: &'a InventoryItem, weapons: Option<&'a Weapons>) -> Self {
        let weapon_data = weapons.and_then(|weapons| weapons.0.get(&item.item_id));

        Self {
            item_id: item.item_id.as_str(),
            name: weapon_data
                .map(|weapon_data| weapon_data.name.as_str())
                .unwrap_or_else(|| item.item_id.as_str()),
            rarity: item.rarity,
            quality: item.quality,
            affixes: item.affixes.as_slice(),
            damage: weapon_data.map(|weapon_data| weapon_data.damage),
            attack_speed: weapon_data.map(|weapon_data| weapon_data.attack_speed),
            attack_range: weapon_data.map(|weapon_data| weapon_data.attack_range),
            weapon_sprite_index: weapon_data.map(|weapon_data| weapon_data.weapon_sprite_index),
        }
    }
}

pub(in crate::menus) fn spawn_item_tooltip(
    ui: &mut ChildSpawnerCommands,
    slot_panel_pos: Vec2,
    tooltip_data: &ItemTooltipData,
    weapon_assets: &WeaponAssets,
) {
    let tooltip_pos = tooltip_panel_position(slot_panel_pos);

    ui.spawn((
        Name::new("Item Tooltip"),
        ItemTooltip,
        absolute_node(tooltip_pos.x, tooltip_pos.y, TOOLTIP_WIDTH, TOOLTIP_HEIGHT),
        ZIndex(TOOLTIP_Z_INDEX),
        BackgroundColor(TOOLTIP_FRAME_COLOR),
        Pickable::IGNORE,
    ))
    .with_children(|ui| {
        ui.spawn((
            Name::new("Item Tooltip Card"),
            Node {
                padding: UiRect::all(px(TOOLTIP_CARD_PADDING)),
                flex_direction: FlexDirection::Column,
                row_gap: px(TOOLTIP_SECTION_GAP),
                ..absolute_node(
                    TOOLTIP_FRAME_PADDING,
                    TOOLTIP_FRAME_PADDING,
                    TOOLTIP_WIDTH - TOOLTIP_FRAME_PADDING * 2.0,
                    TOOLTIP_HEIGHT - TOOLTIP_FRAME_PADDING * 2.0,
                )
            },
            BackgroundColor(TOOLTIP_COLOR),
            Pickable::IGNORE,
        ))
        .with_children(|ui| {
            spawn_tooltip_text(
                ui,
                "Item Tooltip Title",
                tooltip_data.name,
                TOOLTIP_TITLE_FONT_SIZE,
                rarity_color(tooltip_data.rarity),
            );
            spawn_tooltip_separator(ui);
            spawn_tooltip_body(ui, tooltip_data, weapon_assets);
            spawn_tooltip_separator(ui);
            spawn_tooltip_details(ui, tooltip_data);
        });
    });
}

pub(in crate::menus) fn despawn_item_tooltips(
    commands: &mut Commands,
    tooltips: &Query<Entity, With<ItemTooltip>>,
) {
    for tooltip in tooltips.iter() {
        commands.entity(tooltip).despawn();
    }
}

fn spawn_tooltip_body(
    ui: &mut ChildSpawnerCommands,
    tooltip_data: &ItemTooltipData,
    weapon_assets: &WeaponAssets,
) {
    ui.spawn((
        Name::new("Item Tooltip Body"),
        Node {
            width: percent(100),
            flex_direction: FlexDirection::Row,
            column_gap: px(TOOLTIP_BODY_GAP),
            align_items: AlignItems::FlexStart,
            ..default()
        },
        Pickable::IGNORE,
    ))
    .with_children(|ui| {
        spawn_tooltip_icon(ui, tooltip_data, weapon_assets);
        spawn_tooltip_stats(ui, tooltip_data);
    });
}

fn spawn_tooltip_stats(ui: &mut ChildSpawnerCommands, tooltip_data: &ItemTooltipData) {
    ui.spawn((
        Name::new("Item Tooltip Stats"),
        Node {
            flex_direction: FlexDirection::Column,
            row_gap: px(TOOLTIP_STAT_GAP),
            ..default()
        },
        Pickable::IGNORE,
    ))
    .with_children(|ui| {
        spawn_tooltip_text(
            ui,
            "Item Tooltip Rarity",
            tooltip_data.rarity.label(),
            TOOLTIP_RARITY_FONT_SIZE,
            rarity_color(tooltip_data.rarity),
        );
        spawn_tooltip_text(
            ui,
            "Item Tooltip Quality",
            format!("Quality +{}%", tooltip_data.quality),
            TOOLTIP_STAT_FONT_SIZE,
            TEXT_COLOR,
        );
        spawn_tooltip_text(
            ui,
            "Item Tooltip Damage",
            format!("Damage {}", whole_stat(tooltip_data.damage)),
            TOOLTIP_STAT_FONT_SIZE,
            TEXT_COLOR,
        );
        spawn_tooltip_text(
            ui,
            "Item Tooltip Attack Speed",
            format!("Attack Speed {}", decimal_stat(tooltip_data.attack_speed)),
            TOOLTIP_STAT_FONT_SIZE,
            TEXT_COLOR,
        );
        spawn_tooltip_text(
            ui,
            "Item Tooltip Range",
            format!("Range {}", whole_stat(tooltip_data.attack_range)),
            TOOLTIP_STAT_FONT_SIZE,
            TEXT_COLOR,
        );
    });
}

fn spawn_tooltip_details(ui: &mut ChildSpawnerCommands, tooltip_data: &ItemTooltipData) {
    ui.spawn((
        Name::new("Item Tooltip Details"),
        Node {
            flex_direction: FlexDirection::Column,
            row_gap: px(TOOLTIP_STAT_GAP),
            ..default()
        },
        Pickable::IGNORE,
    ))
    .with_children(|ui| {
        if !tooltip_data.affixes.is_empty() {
            spawn_tooltip_text(
                ui,
                "Item Tooltip Affixes Title",
                "Crafted affixes",
                TOOLTIP_DEBUG_FONT_SIZE,
                MUTED_TEXT_COLOR,
            );

            for affix in tooltip_data.affixes {
                spawn_tooltip_text(
                    ui,
                    "Item Tooltip Crafted Affix",
                    affix.label(),
                    TOOLTIP_DEBUG_FONT_SIZE,
                    TEXT_COLOR,
                );
            }
        }

        spawn_tooltip_text(
            ui,
            "Item Tooltip Item Id",
            tooltip_data.item_id,
            TOOLTIP_DEBUG_FONT_SIZE,
            MUTED_TEXT_COLOR,
        );
    });
}

fn spawn_tooltip_icon(
    ui: &mut ChildSpawnerCommands,
    tooltip_data: &ItemTooltipData,
    weapon_assets: &WeaponAssets,
) {
    let rarity_color = rarity_color(tooltip_data.rarity);

    ui.spawn((
        Name::new("Item Tooltip Icon Frame"),
        Node {
            width: px(TOOLTIP_ICON_SIZE),
            height: px(TOOLTIP_ICON_SIZE),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(rarity_color),
        Pickable::IGNORE,
    ))
    .with_children(|ui| {
        ui.spawn((
            Name::new("Item Tooltip Icon Backing"),
            Node {
                width: px(TOOLTIP_ICON_SIZE - TOOLTIP_ICON_BACKING_INSET),
                height: px(TOOLTIP_ICON_SIZE - TOOLTIP_ICON_BACKING_INSET),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(SLOT_COLOR),
            Pickable::IGNORE,
        ))
        .with_children(|ui| {
            if let Some(sprite_index) = tooltip_data.weapon_sprite_index {
                ui.spawn((
                    Name::new(format!("{} Tooltip Icon", tooltip_data.item_id)),
                    Node {
                        width: px(TOOLTIP_ICON_IMAGE_SIZE),
                        height: px(TOOLTIP_ICON_IMAGE_SIZE),
                        ..default()
                    },
                    ImageNode::from_atlas_image(
                        weapon_assets.sprite.clone(),
                        TextureAtlas {
                            layout: weapon_assets.layout.clone(),
                            index: sprite_index,
                        },
                    ),
                    Pickable::IGNORE,
                ));
            } else {
                ui.spawn((
                    Name::new("Item Tooltip Missing Icon"),
                    Node {
                        width: px(TOOLTIP_ICON_IMAGE_SIZE),
                        height: px(TOOLTIP_ICON_IMAGE_SIZE),
                        ..default()
                    },
                    BackgroundColor(rarity_color),
                    Pickable::IGNORE,
                ));
            }
        });
    });
}

fn spawn_tooltip_text(
    ui: &mut ChildSpawnerCommands,
    name: &'static str,
    text: impl Into<String>,
    font_size: f32,
    color: Color,
) {
    ui.spawn((
        Name::new(name),
        Text(text.into()),
        TextFont::from_font_size(font_size),
        TextColor(color),
        Pickable::IGNORE,
    ));
}

fn spawn_tooltip_separator(ui: &mut ChildSpawnerCommands) {
    ui.spawn((
        Name::new("Item Tooltip Separator"),
        Node {
            width: percent(100),
            height: px(1),
            ..default()
        },
        BackgroundColor(TOOLTIP_SEPARATOR_COLOR),
        Pickable::IGNORE,
    ));
}

fn tooltip_panel_position(slot_panel_pos: Vec2) -> Vec2 {
    let slot_center_x = slot_panel_pos.x + SLOT_SIZE / 2.0;
    let place_right = slot_center_x <= PANEL_WIDTH / 2.0;
    let mut x = if place_right {
        slot_panel_pos.x + SLOT_SIZE + TOOLTIP_GAP
    } else {
        slot_panel_pos.x - TOOLTIP_WIDTH - TOOLTIP_GAP
    };

    x = x.clamp(
        TOOLTIP_PANEL_MARGIN,
        PANEL_WIDTH - TOOLTIP_WIDTH - TOOLTIP_PANEL_MARGIN,
    );

    if place_right && x < slot_panel_pos.x + SLOT_SIZE + TOOLTIP_GAP {
        x = slot_panel_pos.x + SLOT_SIZE + TOOLTIP_GAP;
    } else if !place_right && x + TOOLTIP_WIDTH > slot_panel_pos.x - TOOLTIP_GAP {
        x = slot_panel_pos.x - TOOLTIP_WIDTH - TOOLTIP_GAP;
    }

    Vec2::new(x, (PANEL_HEIGHT - TOOLTIP_HEIGHT) / 2.0)
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

fn whole_stat(value: Option<f32>) -> String {
    value
        .map(|value| format!("{value:.0}"))
        .unwrap_or_else(|| "--".to_string())
}

fn decimal_stat(value: Option<f32>) -> String {
    value
        .map(|value| format!("{value:.2}"))
        .unwrap_or_else(|| "--".to_string())
}
