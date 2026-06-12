## Rarities

- Common (White)
- Uncommon (Green)
- Rare (Blue)
- Epic (Purple)
- Legendary (Orange)
- Mythic (Red)

## Base Drop Chance

Enemies have a `10%` base chance to drop loot.

`MonsterProgression.reward_quantity_mult` scales this chance and caps it at
`85%`.

Base rarity weights after a drop succeeds:

- Common: `6200`
- Uncommon: `2500`
- Rare: `900`
- Epic: `300`
- Legendary: `90`
- Mythic: `10`

`MonsterProgression.reward_rarity_mult` scales uncommon and higher weights with
larger scaling on higher rarities, then the final weights are normalized by the
roll.

## Weapon Drops

Enemy loot drops are weapon items. After the drop chance and rarity rolls pass,
the loot listener selects one random entry from `assets/data/weapon_data.ron`.

The drop stores the selected weapon key in `ItemDrop.item_id`, so duplicate keys
are still separate item instances in `RunInventory`. Loot is intentionally not
stackable.

When extraction/profile persistence exists, extracted weapon instances should be
the permanent item objects stored in the profile bank, including rarity and any
future rolls or upgrades.

Drop sprites use the weapon atlas:

- Image: `assets/sprites/weapons/weapons.png`
- Layout: `16x16`, `11` columns, `13` rows
- Atlas indexing: Bevy row-major order, `index = row * 11 + column`
- Data field: `WeaponData.weapon_sprite_index`

Projectile sprites are separate from weapon sprites. Bullets use
`weapon.bullet_sprite` / `weapon.bullet_layout` from
`assets/data/weapon.assets.ron` and `WeaponData.bullet_sprite_index`, so changing
weapon item art does not change projectile art.
