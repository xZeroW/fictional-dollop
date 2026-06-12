use bevy::prelude::*;

use crate::{
    components::ItemRarity,
    game::attributes::{DEXTERITY, INTELLIGENCE, STRENGTH, VITALITY},
    screens::Screen,
};

use super::inventory::InventoryItem;

const MAX_ITEM_QUALITY: u8 = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum CraftingMaterial {
    Whetstone,
    Ember,
    ReforgeOre,
    Essence,
}

impl CraftingMaterial {
    pub const ALL: [Self; 4] = [
        Self::Whetstone,
        Self::Ember,
        Self::ReforgeOre,
        Self::Essence,
    ];

    pub fn index(self) -> usize {
        match self {
            Self::Whetstone => 0,
            Self::Ember => 1,
            Self::ReforgeOre => 2,
            Self::Essence => 3,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Whetstone => "Whetstone",
            Self::Ember => "Ember",
            Self::ReforgeOre => "Reforge Ore",
            Self::Essence => "Essence",
        }
    }
}

#[derive(Resource, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct CraftingMaterials {
    stacks: [u32; CraftingMaterial::ALL.len()],
}

impl Default for CraftingMaterials {
    fn default() -> Self {
        Self::starter()
    }
}

impl CraftingMaterials {
    pub fn starter() -> Self {
        Self {
            stacks: [999; CraftingMaterial::ALL.len()],
        }
    }

    pub fn amount(&self, material: CraftingMaterial) -> u32 {
        self.stacks[material.index()]
    }

    pub fn can_spend(&self, material: CraftingMaterial, amount: u32) -> bool {
        self.amount(material) >= amount
    }

    pub fn spend(&mut self, material: CraftingMaterial, amount: u32) -> bool {
        if !self.can_spend(material, amount) {
            return false;
        }

        self.stacks[material.index()] -= amount;
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum CraftingAffixKind {
    Damage,
    AttackSpeed,
    Range,
    ProjectileSpeed,
    CriticalChance,
    Strength,
    Dexterity,
    Intelligence,
    Vitality,
}

impl CraftingAffixKind {
    const ALL: [Self; 9] = [
        Self::Damage,
        Self::AttackSpeed,
        Self::Range,
        Self::ProjectileSpeed,
        Self::CriticalChance,
        Self::Strength,
        Self::Dexterity,
        Self::Intelligence,
        Self::Vitality,
    ];

    fn random_available(used: &[CraftingAffixKind]) -> Option<Self> {
        let available = Self::ALL
            .into_iter()
            .filter(|kind| !used.contains(kind))
            .collect::<Vec<_>>();

        if available.is_empty() {
            return None;
        }

        Some(available[rand::random_range(0..available.len())])
    }

    fn label(self) -> &'static str {
        match self {
            Self::Damage => "Damage",
            Self::AttackSpeed => "Attack Speed",
            Self::Range => "Range",
            Self::ProjectileSpeed => "Projectile Speed",
            Self::CriticalChance => "Critical Chance",
            Self::Strength => "Strength",
            Self::Dexterity => "Dexterity",
            Self::Intelligence => "Intelligence",
            Self::Vitality => "Vitality",
        }
    }

    fn is_percent(self) -> bool {
        matches!(
            self,
            Self::Damage
                | Self::AttackSpeed
                | Self::Range
                | Self::ProjectileSpeed
                | Self::CriticalChance
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct CraftingAffix {
    pub kind: CraftingAffixKind,
    pub tier: u8,
    pub value: u8,
}

impl CraftingAffix {
    fn random(kind: CraftingAffixKind) -> Self {
        let tier = roll_affix_tier();
        let min_value = tier * 5;
        let max_value = min_value + 5;

        Self {
            kind,
            tier,
            value: rand::random_range(min_value..=max_value),
        }
    }

    pub fn label(self) -> String {
        if self.kind.is_percent() {
            format!("T{} {} +{}%", self.tier, self.kind.label(), self.value)
        } else {
            format!("T{} +{} {}", self.tier, self.value, self.kind.label())
        }
    }

    pub(crate) fn attribute_modifier(self) -> Option<(&'static str, f32)> {
        let attribute = match self.kind {
            CraftingAffixKind::Strength => STRENGTH,
            CraftingAffixKind::Dexterity => DEXTERITY,
            CraftingAffixKind::Intelligence => INTELLIGENCE,
            CraftingAffixKind::Vitality => VITALITY,
            _ => return None,
        };

        Some((attribute, self.value as f32))
    }
}

impl InventoryItem {
    pub fn affix_capacity(&self) -> usize {
        match self.rarity {
            ItemRarity::Common => 0,
            ItemRarity::Uncommon => 1,
            ItemRarity::Rare => 2,
            ItemRarity::Epic => 3,
            ItemRarity::Legendary => 4,
            ItemRarity::Mythic => 5,
        }
    }

    pub fn can_improve_quality(&self) -> bool {
        self.quality < MAX_ITEM_QUALITY
    }

    pub fn can_add_affix(&self) -> bool {
        self.affixes.len() < self.affix_capacity()
            && self.affixes.len() < CraftingAffixKind::ALL.len()
    }

    pub fn can_reforge_affix(&self) -> bool {
        !self.affixes.is_empty()
    }

    pub fn can_guarantee_affix(&self) -> bool {
        self.can_add_affix() || self.can_reforge_affix()
    }

    pub fn improve_quality(&mut self) -> bool {
        if !self.can_improve_quality() {
            return false;
        }

        self.quality = (self.quality + 2).min(MAX_ITEM_QUALITY);
        true
    }

    pub fn add_random_affix(&mut self) -> bool {
        if !self.can_add_affix() {
            return false;
        }

        let used = self.affix_kinds();
        let Some(kind) = CraftingAffixKind::random_available(&used) else {
            return false;
        };

        self.affixes.push(CraftingAffix::random(kind));
        true
    }

    pub fn reforge_random_affix(&mut self) -> bool {
        if !self.can_reforge_affix() {
            return false;
        }

        let index = rand::random_range(0..self.affixes.len());
        self.reforge_affix_at(index, None);
        true
    }

    pub fn guarantee_speed_affix(&mut self) -> bool {
        if !self.can_guarantee_affix() {
            return false;
        }

        if let Some(index) = self
            .affixes
            .iter()
            .position(|affix| affix.kind == CraftingAffixKind::AttackSpeed)
        {
            self.affixes[index] = CraftingAffix::random(CraftingAffixKind::AttackSpeed);
            return true;
        }

        if self.can_add_affix() {
            self.affixes
                .push(CraftingAffix::random(CraftingAffixKind::AttackSpeed));
            return true;
        }

        let index = rand::random_range(0..self.affixes.len());
        self.affixes[index] = CraftingAffix::random(CraftingAffixKind::AttackSpeed);
        true
    }

    fn affix_kinds(&self) -> Vec<CraftingAffixKind> {
        self.affixes.iter().map(|affix| affix.kind).collect()
    }

    fn reforge_affix_at(&mut self, index: usize, forced_kind: Option<CraftingAffixKind>) {
        let used = self
            .affixes
            .iter()
            .enumerate()
            .filter_map(|(other_index, affix)| (other_index != index).then_some(affix.kind))
            .collect::<Vec<_>>();
        let kind = forced_kind
            .or_else(|| CraftingAffixKind::random_available(&used))
            .unwrap_or(self.affixes[index].kind);

        self.affixes[index] = CraftingAffix::random(kind);
    }
}

pub(super) struct CraftingSystemsPlugin;

impl Plugin for CraftingSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Gameplay), reset_crafting_materials);
        app.add_systems(OnExit(Screen::Gameplay), remove_crafting_materials);
    }
}

fn reset_crafting_materials(mut commands: Commands) {
    commands.insert_resource(CraftingMaterials::default());
}

fn remove_crafting_materials(mut commands: Commands) {
    commands.remove_resource::<CraftingMaterials>();
}

fn roll_affix_tier() -> u8 {
    match rand::random_range(0..100) {
        0..=59 => 1,
        60..=86 => 2,
        87..=97 => 3,
        _ => 4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item_with_rarity(rarity: ItemRarity) -> InventoryItem {
        InventoryItem::new("weapon", rarity)
    }

    fn affix(kind: CraftingAffixKind) -> CraftingAffix {
        CraftingAffix {
            kind,
            tier: 1,
            value: 10,
        }
    }

    #[test]
    fn starter_crafting_materials_are_large_testing_stacks() {
        let materials = CraftingMaterials::starter();

        for material in CraftingMaterial::ALL {
            assert_eq!(materials.amount(material), 999);
        }
    }

    #[test]
    fn crafting_material_spend_checks_available_amount() {
        let mut materials = CraftingMaterials::starter();

        assert!(materials.can_spend(CraftingMaterial::Ember, 999));
        assert!(!materials.can_spend(CraftingMaterial::Ember, 1000));
        assert!(materials.spend(CraftingMaterial::Ember, 2));
        assert_eq!(materials.amount(CraftingMaterial::Ember), 997);
        assert!(!materials.spend(CraftingMaterial::Ember, 998));
        assert_eq!(materials.amount(CraftingMaterial::Ember), 997);
    }

    #[test]
    fn affix_capacity_increases_with_rarity() {
        let cases = [
            (ItemRarity::Common, 0),
            (ItemRarity::Uncommon, 1),
            (ItemRarity::Rare, 2),
            (ItemRarity::Epic, 3),
            (ItemRarity::Legendary, 4),
            (ItemRarity::Mythic, 5),
        ];

        for (rarity, capacity) in cases {
            assert_eq!(item_with_rarity(rarity).affix_capacity(), capacity);
        }
    }

    #[test]
    fn common_items_cannot_add_affixes() {
        let mut item = item_with_rarity(ItemRarity::Common);

        assert!(!item.can_add_affix());
        assert!(!item.add_random_affix());
        assert!(item.affixes.is_empty());
    }

    #[test]
    fn add_random_affix_stops_at_rarity_capacity() {
        let mut item = item_with_rarity(ItemRarity::Rare);

        assert!(item.add_random_affix());
        assert!(item.add_random_affix());
        assert!(!item.add_random_affix());
        assert_eq!(item.affixes.len(), 2);
    }

    #[test]
    fn mythic_items_stop_at_affix_capacity() {
        let mut item = item_with_rarity(ItemRarity::Mythic);

        for _ in 0..item.affix_capacity() {
            assert!(item.add_random_affix());
        }

        assert!(!item.add_random_affix());
        assert_eq!(item.affixes.len(), 5);
    }

    #[test]
    fn quality_improvement_caps_at_twenty() {
        let mut item = item_with_rarity(ItemRarity::Rare);

        for _ in 0..10 {
            assert!(item.improve_quality());
        }

        assert_eq!(item.quality, 20);
        assert!(!item.can_improve_quality());
        assert!(!item.improve_quality());
        assert_eq!(item.quality, 20);
    }

    #[test]
    fn guarantee_speed_affix_adds_speed_when_slot_is_open() {
        let mut item = item_with_rarity(ItemRarity::Uncommon);

        assert!(item.guarantee_speed_affix());
        assert_eq!(item.affixes.len(), 1);
        assert_eq!(item.affixes[0].kind, CraftingAffixKind::AttackSpeed);
    }

    #[test]
    fn guarantee_speed_affix_replaces_existing_speed_affix() {
        let mut item = item_with_rarity(ItemRarity::Uncommon);
        item.affixes.push(affix(CraftingAffixKind::Damage));

        assert!(item.guarantee_speed_affix());
        assert_eq!(item.affixes.len(), 1);
        assert_eq!(item.affixes[0].kind, CraftingAffixKind::AttackSpeed);
    }

    #[test]
    fn reforge_requires_existing_affix_and_preserves_affix_count() {
        let mut item = item_with_rarity(ItemRarity::Rare);

        assert!(!item.reforge_random_affix());

        item.affixes.push(affix(CraftingAffixKind::Damage));
        item.affixes.push(affix(CraftingAffixKind::Range));

        assert!(item.reforge_random_affix());
        assert_eq!(item.affixes.len(), 2);
        for affix in &item.affixes {
            assert!((1..=4).contains(&affix.tier));
            assert!((5..=25).contains(&affix.value));
        }
    }

    #[test]
    fn attribute_affixes_are_labeled_as_flat_stats() {
        assert_eq!(
            affix(CraftingAffixKind::Strength).label(),
            "T1 +10 Strength"
        );
    }

    #[test]
    fn attribute_affixes_produce_flat_attribute_modifiers() {
        assert_eq!(
            affix(CraftingAffixKind::Intelligence).attribute_modifier(),
            Some((INTELLIGENCE, 10.0))
        );
        assert_eq!(affix(CraftingAffixKind::Damage).attribute_modifier(), None);
    }
}
