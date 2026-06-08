use bevy::prelude::*;

use crate::{assets::WeaponAssets, game::weapon_data::Weapons, systems::InventoryItem};

use super::{
    MUTED_TEXT_COLOR, PANEL_HEIGHT, PANEL_WIDTH, SLOT_COLOR, SLOT_SIZE, TEXT_COLOR,
    TOOLTIP_BODY_GAP, TOOLTIP_CARD_PADDING, TOOLTIP_COLOR, TOOLTIP_DEBUG_FONT_SIZE,
    TOOLTIP_FRAME_COLOR, TOOLTIP_FRAME_PADDING, TOOLTIP_GAP, TOOLTIP_HEIGHT,
    TOOLTIP_ICON_BACKING_INSET, TOOLTIP_ICON_IMAGE_SIZE, TOOLTIP_ICON_SIZE, TOOLTIP_PANEL_MARGIN,
    TOOLTIP_RARITY_FONT_SIZE, TOOLTIP_SECTION_GAP, TOOLTIP_SEPARATOR_COLOR, TOOLTIP_STAT_FONT_SIZE,
    TOOLTIP_STAT_GAP, TOOLTIP_TITLE_FONT_SIZE, TOOLTIP_WIDTH, TOOLTIP_Z_INDEX, absolute_node,
    rarity_color,
};

#[derive(Component)]
pub(in crate::menus::inventory) struct InventoryItemTooltip;

pub(in crate::menus::inventory) struct InventoryTooltipData<'a> {
    item_id: &'a str,
    name: &'a str,
    rarity: crate::components::ItemRarity,
    damage: Option<f32>,
    attack_speed: Option<f32>,
    attack_range: Option<f32>,
    weapon_sprite_index: Option<usize>,
}

impl<'a> InventoryTooltipData<'a> {
    pub(in crate::menus::inventory) fn new(
        item: &'a InventoryItem,
        weapons: Option<&'a Weapons>,
    ) -> Self {
        let weapon_data = weapons.and_then(|weapons| weapons.0.get(&item.item_id));

        Self {
            item_id: item.item_id.as_str(),
            name: weapon_data
                .map(|weapon_data| weapon_data.name.as_str())
                .unwrap_or_else(|| item.item_id.as_str()),
            rarity: item.rarity,
            damage: weapon_data.map(|weapon_data| weapon_data.damage),
            attack_speed: weapon_data.map(|weapon_data| weapon_data.attack_speed),
            attack_range: weapon_data.map(|weapon_data| weapon_data.attack_range),
            weapon_sprite_index: weapon_data.map(|weapon_data| weapon_data.weapon_sprite_index),
        }
    }
}

pub(in crate::menus::inventory) fn spawn_inventory_item_tooltip(
    ui: &mut ChildSpawnerCommands,
    slot_panel_pos: Vec2,
    tooltip_data: &InventoryTooltipData,
    weapon_assets: &WeaponAssets,
) {
    let tooltip_pos = tooltip_panel_position(slot_panel_pos);

    ui.spawn((
        Name::new("Inventory Item Tooltip"),
        InventoryItemTooltip,
        absolute_node(tooltip_pos.x, tooltip_pos.y, TOOLTIP_WIDTH, TOOLTIP_HEIGHT),
        ZIndex(TOOLTIP_Z_INDEX),
        BackgroundColor(TOOLTIP_FRAME_COLOR),
        Pickable::IGNORE,
    ))
    .with_children(|ui| {
        ui.spawn((
            Name::new("Inventory Item Tooltip Card"),
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
                "Inventory Tooltip Title",
                tooltip_data.name,
                TOOLTIP_TITLE_FONT_SIZE,
                rarity_color(tooltip_data.rarity),
            );
            spawn_tooltip_separator(ui);
            spawn_tooltip_body(ui, tooltip_data, weapon_assets);

            spawn_tooltip_separator(ui);
            spawn_tooltip_text(
                ui,
                "Inventory Tooltip Item Id",
                tooltip_data.item_id,
                TOOLTIP_DEBUG_FONT_SIZE,
                MUTED_TEXT_COLOR,
            );
        });
    });
}

pub(in crate::menus::inventory) fn despawn_inventory_tooltips(
    commands: &mut Commands,
    tooltips: &Query<Entity, With<InventoryItemTooltip>>,
) {
    for tooltip in tooltips.iter() {
        commands.entity(tooltip).despawn();
    }
}

fn spawn_tooltip_body(
    ui: &mut ChildSpawnerCommands,
    tooltip_data: &InventoryTooltipData,
    weapon_assets: &WeaponAssets,
) {
    ui.spawn((
        Name::new("Inventory Tooltip Body"),
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

fn spawn_tooltip_stats(ui: &mut ChildSpawnerCommands, tooltip_data: &InventoryTooltipData) {
    ui.spawn((
        Name::new("Inventory Tooltip Stats"),
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
            "Inventory Tooltip Rarity",
            tooltip_data.rarity.label(),
            TOOLTIP_RARITY_FONT_SIZE,
            rarity_color(tooltip_data.rarity),
        );
        spawn_tooltip_text(
            ui,
            "Inventory Tooltip Damage",
            format!("Damage {}", whole_stat(tooltip_data.damage)),
            TOOLTIP_STAT_FONT_SIZE,
            TEXT_COLOR,
        );
        spawn_tooltip_text(
            ui,
            "Inventory Tooltip Attack Speed",
            format!("Attack Speed {}", decimal_stat(tooltip_data.attack_speed)),
            TOOLTIP_STAT_FONT_SIZE,
            TEXT_COLOR,
        );
        spawn_tooltip_text(
            ui,
            "Inventory Tooltip Range",
            format!("Range {}", whole_stat(tooltip_data.attack_range)),
            TOOLTIP_STAT_FONT_SIZE,
            TEXT_COLOR,
        );
    });
}

fn spawn_tooltip_icon(
    ui: &mut ChildSpawnerCommands,
    tooltip_data: &InventoryTooltipData,
    weapon_assets: &WeaponAssets,
) {
    let rarity_color = rarity_color(tooltip_data.rarity);

    ui.spawn((
        Name::new("Inventory Tooltip Icon Frame"),
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
            Name::new("Inventory Tooltip Icon Backing"),
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
                    Name::new("Inventory Tooltip Missing Icon"),
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
        Name::new("Inventory Tooltip Separator"),
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
