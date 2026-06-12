# Game Systems And Feature Plan

This document connects the current Bevy prototype to the design in `Game Idea.md`.
It separates what already exists from what should be built next, and it records a
practical direction for growing the game without losing the working foundation.

## High-Level Direction

The game is a top-down 2D pixel-art roguelite built around short timed rounds,
loot decisions, and long-term progression.

The strongest direction is:

- Keep the current timed combat and loot prototype.
- Make between-round decisions the heart of the game.
- Turn the current monster-buff prototype into a broader corruption or map-risk system.
- Build the inventory, extraction, bank, and crafting loop before adding class/passive-tree depth.
- Use themed maps to control enemy pools, loot tables, map modifiers, and long-term planning.

`Game Idea.md` says the monster progression idea is being abandoned. The current
code still implements it, and it is useful, but it should not remain the main
identity of the game. It can become a corruption layer, map modifier layer, elite
spawn pressure layer, or optional risk contract system.

## Current Project Snapshot

Already implemented or partially implemented:

- Bevy 0.18 single binary crate named `my_game`.
- Screen states for splash, title, loading, and gameplay in `src/screens/`.
- Menu states for main, pause, settings, credits, inventory, monster buff, and game over in `src/menus/`.
- Pause state and ordered `AppSystems` schedule in `src/main.rs`.
- Tiled map loading through `bevy_ecs_tiled` in `src/game/map.rs`.
- A hardcoded map, currently `assets/maps/map1.tmx`.
- Player movement through `leafwing_input_manager` in `src/game/player.rs`.
- Player health through `bevy_gauge` attributes and `Health` components.
- Automatic weapon targeting and bullet firing in `src/systems/auto_attack.rs`.
- Bullet movement, bullet lifetime, and bullet/enemy collision systems.
- Enemy spawning from data in `assets/data/enemies_data.ron`.
- Enemy visuals from `assets/data/enemies.assets.ron`.
- Enemy behavior types: wandering, follow-and-attack, and coward.
- Enemy spatial index using `kd-tree` for targeting and collision queries.
- Damage, death, collision, and loot events using Bevy messages/listeners.
- Timed waves using `WaveState`, with `WAVE_DURATION` currently set to `60.0`.
- Between-wave inventory menu after each wave.
- Run inventory and run-scoped safe inventory resources.
- Drag and right-click transfer between run inventory and safe inventory.
- Weapon loot drops with rarity tiers and pickup radius.
- Loot chance and rarity scaling from `MonsterProgression` reward multipliers.
- HUD showing health, wave timer, corruption, enemy multipliers, reward multipliers, and loot summary.
- Player death leading to a game-over menu.
- Existing docs for wave and loot systems in `docs/systems/`.

Important current limitations:

- The safe inventory is not persistent across runs yet.
- Continuing from the inventory menu clears the remaining run inventory.
- There is no extraction flow yet.
- There is no crafting system yet.
- There is no profile save/load system yet.
- There is no map theme resource or map rotation system yet.
- There is only one loaded map.
- Enemy selection is data-driven but not theme-driven.
- Weapon drops are collected, but not yet equippable from inventory.
- Rarity affects inventory color and summary, but not item stats yet.
- Weapon data has many placeholder entries with identical stats.
- There are no classes, passive trees, elite enemies, bosses, or boss maps yet.
- The current monster-buff menu conflicts with the note that monster progression is being abandoned.

## Target Game Loop

This should be the base loop before adding large progression systems:

1. Start a run from the title/menu flow.
2. Spawn into the active themed map.
3. Fight enemies during a one-minute round.
4. Pick up weapon drops, materials, currency, and theme-exclusive items.
5. End the round and enter a safe camp/menu phase.
6. Inspect run loot and decide what to bank, craft, equip, salvage, or risk.
7. Choose whether to continue, extract, or accept a corruption/map-risk modifier.
8. Rotate to a new themed map every five rounds.
9. Die and lose unbanked loot, or extract and persist selected rewards.
10. Spend long-term progress on class passives and map passives between runs.

## Design Pillars

- Risk vs reward should be visible every round.
- Loot decisions should matter more than raw enemy farming.
- Short rounds should feed long-term planning.
- The bank should create pressure through limited capacity.
- Map themes should change what the player wants and fears.
- Player power should come from a mix of run loot, permanent item progression,
  crafting, classes, and persistent passives.
- Corruption should improve rewards but change gameplay, not only multiply numbers.

## System Plan

### Run Flow And Round Phases

Current state:

- `WaveState` tracks the active wave and timer.
- When the timer finishes, gameplay pauses and `Menu::Inventory` opens.
- Inventory continue clears unbanked run loot and opens `Menu::MonsterBuff`.
- Choosing a monster buff resumes gameplay.
- Player death opens `Menu::GameOver`.

Problems to solve:

- The round flow is controlled by menus rather than an explicit run phase.
- There is no extraction, victory, or run result state.
- Death does not show lost/banked loot clearly.
- `Menu::MonsterBuff` is still central even though the design moved away from it.

Planned direction:

- Add an explicit run phase resource, for example `RunPhase` or `RunState`.
- Track phases such as combat, camp, map transition, extraction, and game over.
- Let menus display phase choices instead of owning progression rules.
- Rename or replace the inventory menu with a broader camp menu once crafting and extraction exist.
- Add an extract button that ends the run successfully and persists banked loot.
- Add a death summary that shows unbanked loot lost and banked loot kept.

Good first implementation slice:

- Add `RunOutcome` with values like `InProgress`, `Extracted`, and `Dead`.
- Add an extract button to the between-wave inventory menu.
- On extraction, transition out of gameplay and keep safe inventory through a profile/bank transfer.
- On death, show a summary before resetting gameplay.

### Loot, Items, And Inventory

Current state:

- Enemy deaths can spawn weapon drops.
- Loot rarity tiers exist in `ItemRarity`.
- Drops are picked up into `RunInventory`.
- `SafeInventory` has capacity `20` and supports drag transfer.
- Inventory UI shows weapon icons and rarity colors.

Problems to solve:

- Safe inventory is run-scoped, not a real persistent bank.
- Items only store `item_id` and `rarity`.
- Rarity does not modify item stats.
- Weapon drops cannot be equipped or inspected in detail.
- There are no material drops for crafting.
- There are no stackable items, item tags, affixes, or theme restrictions.

Planned direction:

- Split inventory concepts into temporary run inventory, run bank/stash, and persistent profile bank.
- Keep `RunInventory` for unprotected loot collected during the active round.
- Reframe the current `SafeInventory` as protected run stash or convert it into a true persistent bank after extraction.
- Add item definitions with category, rarity behavior, stackability, icon, and theme tags.
- Add materials as stackable drops so crafting can start small while still
  feeding permanent item progression.
- Add weapon inspection and equip actions before building a large gear system.
- Make rarity affect weapon stat rolls or affix counts.

Good first implementation slice:

- Add stackable material item definitions.
- Add one theme-neutral material drop alongside weapon drops.
- Add inventory item details on hover or click.
- Add equip action for weapon items and update the player's `Weapon` key.

### Bank, Extraction, And Death Penalty

Current state:

- The project has a safe inventory UI and transfer mechanics.
- Safe inventory does not survive leaving gameplay.
- Game over exists, but there is no extraction success state.

Problems to solve:

- The design requires persistent safe inventory across runs.
- Death should destroy unbanked run inventory but not safe inventory.
- Bank capacity should create meaningful choices.
- Extraction should be a strategic decision, not an automatic process.

Planned direction:

- Add a persistent profile resource with banked items, unlocks, passive points, and run stats.
- Move items from run stash to persistent bank only on extraction or explicit bank actions.
- Keep unbanked run inventory vulnerable to death.
- Show a clear death/extraction summary.
- Start with local save data before worrying about Steam inventory or market integration.

Good first implementation slice:

- Add `Profile` or `PlayerProfile` resource.
- Persist bank inventory to a small RON or JSON save file.
- Add extract action from the camp menu.
- On extraction, transfer protected items into the profile bank.
- On death, clear run inventory and leave profile bank untouched.

### Crafting And Between-Round Decisions

Current state:

- There is no crafting system yet.
- The between-wave inventory menu is already the right place to add it.

Problems to solve:

- Loot currently has no use beyond collection.
- Between-wave decisions are mostly banking and monster buff selection.
- Crafting must be useful without becoming a large content burden immediately.

Planned direction:

- Add small recipes that consume run loot or banked materials.
- Treat permanent item progression as the default crafting destination once
  extraction and the profile bank exist.
- Allow temporary run upgrades as optional consumables, but label them clearly
  and do not make them the primary crafting path.
- Use crafting to create decisions before the next round starts.

First recipes to consider:

- Salvage a weapon into material shards based on rarity.
- Spend shards to craft a bankable upgrade material.
- Spend shards to apply a small permanent upgrade to a selected weapon or gear
  item.
- Spend shards to reroll a low-rarity drop into another bankable item.
- Spend a theme-exclusive material for a map-specific permanent unlock, recipe,
  or item upgrade.
- Optionally spend shards to heal before the next round as a run-only recipe.

Good first implementation slice:

- Add a `CraftingRecipe` type.
- Add one recipe: salvage selected weapon into bankable shards.
- Add one permanent recipe: spend shards to craft an upgrade material or improve
  a selected item.
- Add one optional run-only recipe: spend shards to heal the player.
- Add a simple crafting panel to the between-wave menu.

### Map Themes And Rotation

Current state:

- The map is loaded from `assets/maps/map1.tmx`.
- The map bounds are hardcoded in `src/config.rs`.
- There is no map theme state.
- There is no random map selection every five rounds yet.

Problems to solve:

- The design requires themed maps like Forest, Crypt, Factory, and Void.
- Themes should affect drops, enemy pools, visual flavor, and mechanics.
- The current map setup cannot rotate maps or theme-specific data.

Planned direction:

- Add `MapTheme` and `ActiveMap` resources.
- Add theme definitions with map asset path, enemy table, loot table, exclusive drops, and modifiers.
- Rotate or reroll theme every five rounds.
- Start with hardcoded theme definitions before making them fully data-driven.
- Update map bounds from map data when possible instead of relying only on constants.

Theme ideas:

- Forest: beasts, poison plants, healing herbs, wood/leaf materials.
- Crypt: undead, curse modifiers, bone/relic materials, slower but tougher enemies.
- Factory: constructs, traps, scrap materials, projectile hazards.
- Void: unstable enemies, teleport events, high-rarity drops, corruption spikes.

Good first implementation slice:

- Add an `ActiveMapTheme` resource.
- Keep using `map1.tmx`, but assign it the Forest theme.
- Add a second placeholder theme that changes enemy weights and loot tags without needing a new map asset yet.
- Add HUD text for the active theme.

### Corruption, Risk Contracts, And Map Modifiers

Current state:

- `MonsterProgression` tracks corruption and enemy/reward multipliers.
- `MONSTER_BUFF_CHOICES` has three choices: health, speed, and damage risk.
- Enemy stats and loot odds consume these multipliers.
- The design note says the monster buff system is being abandoned.

Problems to solve:

- Pure stat multipliers can become invisible and repetitive.
- A mandatory monster-buff choice every round may not match the new direction.
- The risk/reward pillar still needs a mechanical home.

Planned direction:

- Keep corruption as a run pressure system, but stop framing every choice as monster evolution.
- Convert monster buffs into optional risk contracts or map modifiers.
- Let corruption trigger elites, boss invasions, dangerous map events, and better loot odds.
- Make each risk choice visibly change the next round.
- Preserve the existing multiplier fields temporarily while adding more expressive effects.

Risk contract ideas:

- Blood Moon: enemies spawn faster, but drops have higher rarity.
- Glass Hoard: enemies deal more damage, but chests or elites drop extra items.
- Cursed Floor: the map adds hazards, but theme-exclusive drops are more common.
- Hunter Mark: elite spawn chance rises, but elite drops are guaranteed.
- Unstable Portal: a mini-boss can invade, but extraction rewards are multiplied.

Good first implementation slice:

- Rename UI text away from monster evolution language.
- Add one non-stat modifier, such as increased elite chance once elites exist.
- Track selected risk contracts in the run state for summary and rewards.

### Enemies, Elites, And Bosses

Current state:

- Enemy data is loaded from RON.
- There are two enemy definitions: Green Devil and Red Devil.
- Spawn weights are supported.
- Enemy behavior supports following, wandering, and coward movement.
- Enemy stats scale with current progression multipliers.

Problems to solve:

- Enemy pools are not map-theme-specific.
- There are no elites or boss encounters.
- There are no enemy abilities beyond movement/contact damage.
- There is no run-ending goal except survival until death.

Planned direction:

- Add theme-specific enemy pools.
- Add `Elite` component with stat multiplier, visual marker, and improved drops.
- Add elite spawn chance from wave number, corruption, or map modifiers.
- Add boss waves or boss maps as run milestones.
- Keep boss behavior simple at first, then split into a boss module when needed.

Enemy content ideas:

- Forest wolf: fast melee chaser.
- Forest shambler: slow tank with better material drops.
- Crypt skeleton: basic swarm enemy.
- Crypt wraith: coward or hit-and-run enemy.
- Factory drone: ranged or explosive enemy once projectile enemies exist.
- Void leech: teleports or splits at low health.

Good first implementation slice:

- Add an `Elite` component and visual tint/scale change.
- Roll elite chance in enemy spawning after wave 3 or corruption 2.
- Give elites a guaranteed material drop and improved rarity roll.

### Player Classes And Passive Trees

Current state:

- There is one player setup.
- The player starts with `dagger_01`.
- The player has `Vitality` and health attributes.
- There is no class selection or passive tree.

Problems to solve:

- Classes are a major design goal but would be expensive before the base loop works.
- Passive trees need persistent progression and a save profile first.
- Class identity needs weapons, abilities, or passives to matter.

Planned direction:

- Defer full passive trees until extraction and profile persistence exist.
- Add a small `ClassDefinition` concept first.
- Each class should define starting weapon, base attributes, class passive, and future passive tree id.
- Use the existing `bevy_gauge` attributes to support class stats.
- Keep early passive nodes simple and testable.

Class ideas:

- Shepherd: balanced survivor, better banking/extraction economy, steady projectile weapons.
- Butcher: melee-focused, higher damage, gains value from salvaging weapons.
- Occultist: corruption-focused, better rarity odds, accepts higher map risk.
- Ranger: movement and range, safer farming, weaker bank/crafting bonuses.

Good first implementation slice:

- Add class selection with two simple classes.
- Let each class set starting weapon and one base attribute modifier.
- Add one passive point reward on extraction.

### Map Passive Tree

Current state:

- There is no map passive system yet.

Problems to solve:

- The design wants map passives to influence theme appearance, exclusive drops, and modifiers.
- This system depends on map themes and persistence.

Planned direction:

- Build map themes first.
- Add profile-backed map passive unlocks after theme rotation works.
- Keep the first map passive tree small and global.

First map passive ideas:

- Forest affinity: Forest appears more often.
- Relic hunter: theme-exclusive drops are slightly more common.
- Cartographer: see the next theme before choosing whether to extract.
- Safer roads: first modifier after a map rotation has reduced danger.
- Deep delver: higher corruption rewards after wave 5.

### Weapons And Equipment

Current state:

- Weapon data is loaded from `assets/data/weapon_data.ron`.
- The weapon asset sheet has icons for many weapon categories.
- The player has a `Weapon` component with a key.
- Auto attack uses weapon data for damage, velocity, attack speed, and attack range.
- Dropped weapons currently use the weapon icon and item id.

Problems to solve:

- Most weapon entries are placeholder clones.
- Inventory weapons cannot yet be equipped.
- There are no rarity-based stat rolls.
- There are no affixes, tooltips, or comparison UI.

Planned direction:

- Add weapon equip from inventory.
- Make weapon families meaningfully different before adding many entries.
- Add rarity stat scaling or affix count.
- Add a simple item tooltip that shows weapon stats.
- Consider separating weapon base data from dropped item instances once affixes exist.

Weapon family ideas:

- Dagger: short range, high attack speed.
- Bow: longer range, moderate speed.
- Axe: slower, higher damage.
- Sceptre: slower projectile with special effects later.
- Gun: fast projectile, lower damage, higher range.
- Codex: unusual behavior, possibly orbitals or area effects later.

Good first implementation slice:

- Change a few weapon families to different stats.
- Add inventory click action to equip a selected weapon.
- Show equipped weapon name in the HUD.

### HUD, Feedback, And UX

Current state:

- HUD shows wave, timer, health, corruption, multipliers, and loot summary.
- Inventory menu shows run and safe inventories.
- Hit flash provides enemy damage feedback.
- Game over menu has retry and title actions.

Problems to solve:

- Loot pickup feedback is minimal.
- The player cannot inspect items deeply.
- Between-wave choices need clearer hierarchy.
- Death and extraction need summaries.
- Map theme and modifiers are not shown because they do not exist yet.

Planned direction:

- Add pickup notifications.
- Add item tooltips to inventory.
- Add active map theme and active modifiers to HUD.
- Add end-of-round summary with loot gained, banked, lost, and crafted.
- Make corruption/risk effects visible before the player commits.

Good first implementation slice:

- Add a small pickup feed for item name and rarity.
- Add active weapon name to HUD.
- Add item tooltip in the inventory menu.

### Persistence And Profile

Current state:

- There is no save/load system.
- Run resources are inserted on `OnEnter(Screen::Gameplay)` and removed on exit.

Problems to solve:

- Safe inventory, unlocks, passive points, map passives, and class progression need persistence.
- Monetization or Steam market ideas should not be explored before local item persistence is trustworthy.

Planned direction:

- Start with a simple local profile save.
- Persist only what is needed: bank items, material balances, crafted item state,
  unlocked classes, passive points, and settings-like progression flags.
- Keep run inventory separate from persistent bank.
- Add versioning early if save data is expected to evolve.

Good first implementation slice:

- Add a profile resource with bank items and passive points.
- Save profile on extraction and load on startup or title entry.
- Add safe fallback behavior when the profile file is missing or malformed.

### Monetization-Sensitive Systems

Current state:

- `docs/monetization.md` says free-to-play with Steam market sellable items and a 10 percent dev fee.

Risks to avoid:

- Do not design early gameplay around market value.
- Do not make direct power trading the first economy implementation.
- Do not let monetization requirements contaminate balance before the game loop is fun.

Planned direction:

- Treat marketable items as a late-stage layer.
- Build local item identity and persistence first.
- Prefer cosmetics, account-bound crafting, or clearly separated trade rules until the gameplay economy is stable.
- If tradable gear remains a goal, design item provenance and anti-duplication rules much later.

## Recommended Milestones

### Milestone 0: Stabilize The Current Prototype

Status: mostly present.

Goal:

- Make the current wave, combat, loot, inventory, and monster-buff flow reliable and understandable.

Tasks:

- Keep `cargo check --locked` passing.
- Update docs when changing wave, loot, or inventory behavior.
- Fix any state transition issues around retry, game over, and menu cleanup.
- Add basic item inspection so the player understands drops.

### Milestone 1: Complete The Small Run Loop

Goal:

- The player can fight, loot, bank, extract, or die with clear consequences.

Tasks:

- Add run outcome state.
- Add extraction from the between-wave menu.
- Add death and extraction summaries.
- Add profile bank persistence.
- Preserve banked loot and delete unbanked loot according to the design.

Acceptance criteria:

- Dying loses unbanked run loot.
- Extracting saves protected loot.
- Restarting a run can show previously banked loot.

### Milestone 2: Make Loot Useful

Goal:

- Drops should create decisions instead of only filling inventory slots.

Tasks:

- Add item details and tooltips.
- Add weapon equip from inventory.
- Add material drops.
- Add salvage, one permanent upgrade recipe, and optionally one healing recipe.
- Make at least three weapon families mechanically different.

Acceptance criteria:

- The player can equip a dropped weapon.
- The player can convert loot into banked material, persistent item progression,
  or an explicit run-only effect.
- Rarity has a gameplay meaning beyond color.

### Milestone 3: Add Map Themes

Goal:

- The map system starts matching the design instead of using one fixed map forever.

Tasks:

- Add active map theme state.
- Add theme definitions.
- Show active theme on HUD.
- Make loot tables and enemy pools theme-aware.
- Rotate theme every five waves.

Acceptance criteria:

- The player can see the current theme.
- Different themes can change enemy selection and exclusive drop chances.
- Theme rotation happens on schedule.

### Milestone 4: Replace Monster Buffs With Risk Contracts

Goal:

- Keep the risk/reward value of corruption without centering the abandoned monster-progression idea.

Tasks:

- Rename UI language away from monster evolution.
- Add risk contracts or map modifiers.
- Tie corruption to elite chance, map hazards, boss invasions, and reward upgrades.
- Keep numeric multipliers as one effect type, not the whole system.

Acceptance criteria:

- The player chooses risk for better rewards.
- At least one risk choice visibly changes gameplay.
- The old monster-buff menu no longer defines the game's identity.

### Milestone 5: Add Elites And A Run Goal

Goal:

- Runs need spikes of danger and a satisfying endpoint.

Tasks:

- Add elite enemies.
- Add better elite drops.
- Add boss wave or boss map trigger.
- Add successful run completion after a boss or planned extraction threshold.

Acceptance criteria:

- Elites are visually distinct.
- Boss or run-ending encounters can be triggered deterministically.
- The player has a reason to push beyond only collecting more loot.

### Milestone 6: Add Meta Progression

Goal:

- Long-term progression supports repeated runs.

Tasks:

- Add class definitions and class selection.
- Add profile-backed passive points.
- Add small class passive trees.
- Add early map passive unlocks after map themes are reliable.

Acceptance criteria:

- Extracting or completing goals grants persistent progress.
- Classes feel different in starting loadout or stat identity.
- Passive unlocks are saved and loaded.

## Suggested Immediate Next Steps

1. Add item inspection and weapon equip from the inventory menu.
2. Add extraction and a persistent profile bank.
3. Add material drops plus one salvage and permanent-upgrade crafting path.
4. Add `ActiveMapTheme` with a placeholder Forest theme.
5. Reword or replace the monster-buff menu as a risk contract menu.

These steps use systems that already exist and avoid jumping straight into large
passive trees or many maps before the base game loop is complete.

## Open Design Decisions

- Should the current monster progression code be renamed into corruption now, or kept until risk contracts are implemented?
- Should the safe inventory be persistent immediately, or should extraction move selected items into a separate persistent bank?
- Should weapon drops be equippable gear, crafting ingredients, market items, or a mix?
- Should class passives be earned by extraction, boss kills, account level, or specific achievements?
- Should map rotation always happen every five rounds, or should the player sometimes choose the next theme?
- Should boss maps end the run, offer extraction, or continue into a higher-risk loop?
