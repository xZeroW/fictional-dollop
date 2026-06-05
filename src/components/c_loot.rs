use bevy::prelude::*;

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
pub struct ItemDrop {
    pub item_id: String,
    pub rarity: ItemRarity,
    pub quantity: u32,
}

#[derive(Component, Debug, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct PickupRadius(pub f32);

impl PickupRadius {
    pub const DEFAULT: f32 = 28.0;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
}

impl ItemRarity {
    pub const ALL: [Self; 6] = [
        Self::Common,
        Self::Uncommon,
        Self::Rare,
        Self::Epic,
        Self::Legendary,
        Self::Mythic,
    ];

    pub fn index(self) -> usize {
        match self {
            ItemRarity::Common => 0,
            ItemRarity::Uncommon => 1,
            ItemRarity::Rare => 2,
            ItemRarity::Epic => 3,
            ItemRarity::Legendary => 4,
            ItemRarity::Mythic => 5,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            ItemRarity::Common => "Common",
            ItemRarity::Uncommon => "Uncommon",
            ItemRarity::Rare => "Rare",
            ItemRarity::Epic => "Epic",
            ItemRarity::Legendary => "Legendary",
            ItemRarity::Mythic => "Mythic",
        }
    }

    pub fn color(self) -> Color {
        match self {
            ItemRarity::Common => Color::srgb(0.90, 0.90, 0.86),
            ItemRarity::Uncommon => Color::srgb(0.22, 0.86, 0.34),
            ItemRarity::Rare => Color::srgb(0.22, 0.46, 0.95),
            ItemRarity::Epic => Color::srgb(0.62, 0.25, 0.92),
            ItemRarity::Legendary => Color::srgb(1.00, 0.55, 0.10),
            ItemRarity::Mythic => Color::srgb(0.92, 0.08, 0.10),
        }
    }
}
