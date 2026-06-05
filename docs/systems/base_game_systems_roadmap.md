# Base Game Systems Roadmap

This roadmap focuses on new systems needed to turn the current combat, wave,
enemy, and monster corruption prototype into a base game loop.

The goal is not full content breadth. The goal is a complete playable skeleton:
enter a run, survive waves, choose monster risk, gain loot, make between-wave
decisions, die or extract, and keep some progress.

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
7. Manage loot and craft a small upgrade between waves.
8. Continue, extract, or die.
9. Persist extracted loot or unlocked progress.

## Recommended Build Order

1. Loot drop system.
2. Run inventory system.
3. Between-wave reward/inventory menu.
4. Extraction and death-loss rules.
5. Minimal crafting system.
6. Map theme and map rotation system.
7. Elite enemy system.
8. Boss or run-ending encounter system.
9. Persistent profile system.
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

The run inventory stores what the player collected during the current run. It is
not persistent by default.

Purpose:

- Track temporary loot earned during waves.
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
- Destroy unextracted temporary loot on death.
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
- Dying ends the run and loses temporary loot.

## 5. Minimal Crafting System

Crafting should be tiny at first. It exists to make loot useful between waves.

Purpose:

- Give players a reason to care about material drops.
- Add a strategic choice before accepting more corruption.

Suggested shape:

- Add `src/systems/crafting.rs` with recipe definitions and validation.
- Add a simple `Recipe` type: input item stacks, output effect or item.
- Start with one or two recipes.
- Prefer temporary run upgrades before permanent build complexity.

Good first recipes:

- Heal the player before the next wave.
- Increase weapon damage for the current run.
- Increase player speed for the current run.

Acceptance criteria:

- Crafting consumes run inventory items.
- Crafting applies a visible gameplay effect.
- Invalid recipes cannot be crafted without ingredients.

## 6. Map Theme And Rotation System

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

## 7. Elite Enemy System

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

## 8. Boss Or Run-Ending Encounter System

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

## 9. Persistent Profile System

Persistence should come after extraction exists. Otherwise there is nothing
meaningful to save.

Purpose:

- Store extracted loot.
- Store unlocks or permanent progression.
- Support future class and passive tree systems.

Suggested shape:

- Add `src/systems/profile.rs` or a top-level `profile` module if it grows.
- Keep the first save format small and explicit.
- Persist bank inventory and simple unlock flags first.

Acceptance criteria:

- Extracted loot survives leaving and restarting a run.
- Death does not delete banked loot.
- Save/load failure has a safe fallback.

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

- Large itemization and equipment depth.
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
4. Between waves, a menu shows the collected material.
5. The player can craft one temporary upgrade or extract the material.
6. Death loses unextracted material.

Once this works, the project has the smallest complete version of the intended
base game loop.
