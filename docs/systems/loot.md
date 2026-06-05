## Rarities

- Common (White)
- Uncommon (Green)
- Rare (Blue)
- Epic (Purple)
- Legendary (Orange)
- Mythic (Red)

## Base Drop Chance

Enemies have a `1%` base chance to drop loot.

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
