# Base Game Systems Roadmap

This roadmap focuses on new systems needed to turn the current combat, wave,
enemy, and monster corruption prototype into a base game loop.

The goal is not full content breadth. The goal is a complete playable skeleton:
enter a run, survive waves, choose monster risk, gain loot, make between-wave
decisions, die or extract, and keep extracted item and account progress.

## Current Foundation

Already present:

- Player movement, attacks, bullets, damage, health, death, and game over.
- Enemy spawning, movement behavior, collision, damage, and death messages.
- Timed waves with between-wave monster buff choices.
- Run-scoped `MonsterProgression` with corruption and enemy/reward multipliers.
- Tiled map loading and level-owned entity cleanup.
- Menus, screen states, pause state, and HUD.

## Target Base Game Loop

1. Start a run.
2. Spawn into a themed map.
3. Fight enemies during a timed wave.
4. Enemies drop loot.
5. Finish the wave.
6. Choose a monster buff.
7. Manage loot, craft upgrades or gear, and decide what to bank between waves.
8. Continue, extract, or die.
9. Persist extracted loot, crafted items, unlocks, and account progress.

## Recommended Build Order

1. Loot drop system.
2. Run inventory system.
3. Between-wave reward/inventory menu.
4. Extraction and death-loss rules.
5. Persistent profile and bank system.
6. Minimal crafting system.
7. Map theme and map rotation system.
8. Elite enemy system.
9. Boss or run-ending encounter system.
10. Base-game HUD and feedback pass.

## 1. Loot Drop System

This should be the next new system because it completes the risk/reward promise
of monster corruption.

Purpose:

- Convert enemy deaths into item drops.
- Consume `MonsterProgression.reward_quantity_mult` and
  `MonsterProgression.reward_rarity_mult`.
- Provide visible rewards during the active wave.

Suggested shape:

- Add `src/systems/loot.rs` for drop resources and item pickup behavior.
- Add `src/listeners/loot.rs` to react to `EntityDiedMessage` for enemies.
- Add `LootDropMessage` if drop creation needs to stay decoupled from death
  handling.
- Add `ItemDrop`, `PickupRadius`, or similar components under `src/components/`.
- Parent spawned drops under `LevelEntity` so they clean up with the level.

Minimum data:

- Item id.
- Display name.
- Rarity.
- Quantity or stack count.
- (Optional) Map theme tag for later filtering.

Acceptance criteria:

- Enemies don't have guaranteed loot drops.
- Loot quantity or chance improves when reward quantity increases.
- Rarity odds improve when reward rarity increases.
- Picked-up loot is removed from the world and added to run inventory. // Needs Run Inventory System

## 2. Run Inventory System

The run inventory stores what the player collected during the current run before
it is banked or extracted. It is not persistent by itself, but its items become
permanent when moved to the profile bank through extraction or safe banking.

Purpose:

- Track unbanked loot earned during waves.
- Provide a source for crafting and extraction decisions.
- Support death penalties.

Suggested shape:

- Add `src/systems/inventory.rs` with a run-scoped `RunInventory` resource.
- Insert `RunInventory` on `OnEnter(Screen::Gameplay)`.
- Remove it on `OnExit(Screen::Gameplay)` after death/extraction has resolved.
- Store item stacks by item id.

Acceptance criteria:

- Pickups add stacks to `RunInventory`.
- HUD or a debug menu can show current collected loot.
- Death can clear the inventory without touching persistent storage.

## 3. Between-Wave Reward Menu

The player needs a safe decision point after each wave, not only a monster buff
choice.

Purpose:

- Show collected loot.
- Let the player continue, craft, or extract once extraction exists.
- Make the round loop feel deliberate.

Suggested shape:

- Add a new `Menu` state such as `Menu::WaveReward` or `Menu::Camp`.
- After wave completion, open the reward menu before or after
  `Menu::MonsterBuff`.
- Keep gameplay paused while the menu is open.
- Start with read-only inventory display, then add actions.

Recommended flow:

1. Wave ends.
2. Open reward/inventory menu.
3. Player reviews loot and optionally crafts.
4. Open monster buff menu.
5. Player chooses the next risk.
6. Resume gameplay.

Acceptance criteria:

- The wave transition has a clear between-wave phase.
- The player can inspect current run loot before choosing to continue.
- Menus close cleanly and resume the next wave.

## 4. Extraction And Death-Loss System

The base game needs a way to leave with rewards and a reason death matters.

Purpose:

- Let the player bank some or all run loot.
- Destroy unextracted or unbanked run loot on death.
- Create a real decision between greed and safety.

Suggested shape:

- Add an `ExtractionState` or run outcome resource in `src/systems/`.
- Add an extract action in the between-wave menu.
- Add a persistent `BankInventory` later, but first prove the transfer from
  run inventory to a placeholder persistent resource.
- On player death, clear `RunInventory` and show what was lost.

Acceptance criteria:

- Choosing extract ends the run successfully.
- Extracted loot is retained outside the run.
- Dying ends the run and loses unbanked run loot.

## 5. Persistent Profile And Bank System

Persistence should come immediately after extraction exists. Otherwise item
progression cannot feel like an RPG progression system.

Purpose:

- Store extracted loot, crafted items, materials, and currency.
- Store unlocks or permanent progression.
- Make item and crafting decisions matter across runs.
- Support future class and passive tree systems.

Suggested shape:

- Add `src/systems/profile.rs` or a top-level `profile` module if it grows.
- Keep the first save format small and explicit.
- Persist bank inventory, material stacks, crafted item state, and simple unlock
  flags first.
- Save after extraction and after any craft that changes persistent inventory.

Acceptance criteria:

- Extracted loot and crafted persistent items survive leaving and restarting a run.
- Death does not delete banked loot or crafted persistent progress.
- Save/load failure has a safe fallback.

## 6. Minimal Crafting System

Crafting should be tiny at first, but it should still point toward permanent
RPG-style item progression. Temporary run effects can exist, but they should be
clearly marked as consumable run help, not the main progression path.

Purpose:

- Give players a reason to care about material drops.
- Add a strategic choice before accepting more corruption.
- Create or improve items, materials, or gear that can persist across runs.

Suggested shape:

- Add `src/systems/crafting.rs` with recipe definitions and validation.
- Add a simple `Recipe` type: input item stacks, output item, item upgrade,
  material stack, or explicit run-only effect.
- Start with one or two recipes that prove both consumption and persistence.
- Use profile-bank items or extracted materials for permanent recipes once the
  profile system exists.

Good first recipes:

- Salvage a weapon into bankable material shards based on rarity.
- Spend shards to craft a bankable upgrade material.
- Spend shards to apply a small permanent upgrade to a selected weapon or piece
  of gear.
- Heal the player before the next wave as an optional run-only recipe.

Acceptance criteria:

- Crafting consumes run inventory or profile-bank items according to the recipe.
- At least one recipe creates banked material, crafted gear, or a persistent item
  upgrade.
- Run-only recipes are labeled as run-only and do not replace permanent item
  progression.
- Invalid recipes cannot be crafted without ingredients.

## 7. Map Theme And Rotation System

The design calls for themed maps and different drops. The base version only
needs one additional map or theme to prove the structure.

Purpose:

- Track the active map theme.
- Filter enemy types and loot by theme.
- Rotate or reroll maps every few waves.

Suggested shape:

- Add `src/systems/map_progression.rs` for active map/theme state.
- Add `MapTheme` data that can be referenced by loot and enemy selection.
- Start with hardcoded themes before data-loading them.
- Rotate theme every 5 waves, matching the game idea.

Acceptance criteria:

- The active theme is available as a resource during gameplay.
- Loot tables can depend on the active theme.
- Enemy selection can eventually depend on the active theme.

## 8. Elite Enemy System

Elite enemies are the simplest way to make corruption change gameplay beyond
numeric scaling.

Purpose:

- Create spikes of danger.
- Add higher-value targets.
- Give corruption milestones a visible effect.

Suggested shape:

- Add an `Elite` component.
- During enemy spawning, roll an elite chance based on corruption or wave number.
- Apply elite stat multipliers and a visual marker.
- Give elites improved drop odds through the loot system.

Acceptance criteria:

- Elite enemies appear after a clear threshold or chance roll.
- Elites are visually distinguishable.
- Elites are harder and reward better loot.

## 9. Boss Or Run-Ending Encounter System

The base game needs a goal. A boss wave is the clearest first version.

Purpose:

- Provide a run climax.
- Give corruption a long-term consequence.
- Create a win condition for the base loop.

Suggested shape:

- Add `src/systems/boss.rs` or `src/enemies/boss.rs` once boss behavior differs
  enough from normal enemies.
- Trigger a boss at a wave number or corruption threshold.
- Pause normal wave spawning during the boss encounter.
- End the run successfully when the boss dies.

Acceptance criteria:

- A boss encounter can start deterministically.
- Normal wave flow does not fight the boss state.
- Killing the boss produces a successful run outcome.

## 10. HUD And Feedback Pass

Once the base systems exist, make the loop readable.

Purpose:

- Show what matters without opening debug tools.
- Make rewards and risk legible.

Needed feedback:

- Loot pickup text or small notifications.
- Current run inventory summary.
- Wave and extraction status.
- Current map theme.
- Elite/boss warning text.
- Death and extraction summaries.

Acceptance criteria:

- The player can understand what they gained, what they risk, and what happens
  next.

## Deferred Until After Base Game

These are important, but should wait until the base loop is playable:

- Large itemization depth beyond the first persistent item/crafting path.
- Full safe inventory UI.
- Class selection.
- Passive trees.
- Map passive tree.
- Many map themes.
- Data-driven everything.
- Boss maps as separate map types.
- Complex enemy state machines.

## First Concrete Milestone

Build this milestone before expanding content:

1. Enemies drop one material item.
2. The player can pick it up.
3. The item appears in `RunInventory`.
4. Between waves, a menu shows the collected material and persistent bank.
5. The player can craft one persistent material/item upgrade or extract the
   material.
6. Death loses unextracted material but never deletes banked or crafted
   persistent progress.

Once this works, the project has the smallest complete version of the intended
base game loop.
