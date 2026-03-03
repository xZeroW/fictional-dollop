# Copilot instructions — my_rebanho

Purpose: help an AI coding agent be immediately productive in this Bevy-based game.

- **Big picture:** this repository is a Bevy 2D game structured as modular Bevy plugins. The app entrypoint is `src/main.rs`, which wires together plugins from `src/` such as `asset_tracking`, `audio`, `demo`, `menus`, `screens`, and `theme`.

- **Major components & why:**
  - `src/main.rs`: registers Bevy plugins and global `AppSystems` order. When adding system groups, update `configure_sets` here.
  - `src/screens/`: state-driven UI and game screens (splash, title, loading, gameplay). Follow existing patterns for states and `in_state` guards.
  - `src/demo/`: gameplay/demo code (movement, player, levels). Use these modules for entity/component examples.
  - `src/theme/`: palette, widget styles and reusable UI pieces.
  - `assets/`: organized into `images/`, `sprites/`, `audio/` (music, sfx). Native dev builds support hot-reload via features.

- **Build & run (concrete commands used in this workspace):**
  - Build dev: `bevy run` (VS Code task: "Run dev build" — sets `RUST_BACKTRACE=full`).
  - Build release: `bevy run --release`.

- **Important repo-specific notes for edits and fixes:**
  - Asset meta check is disabled for web builds in `src/main.rs` (see `AssetMetaCheck::Never`). Avoid reverting that change when touching the default plugin set — it prevents web build failures.
  - System ordering: `AppSystems` variants are explicitly ordered in `main.rs`. If you add a new variant, add it to the `configure_sets` call in the same order.
  - Pause handling: code uses a `Pause` state and `PausableSystems.run_if(in_state(Pause(false)))`. Use this same `run_if` pattern for systems that should stop when paused.

- **Conventions & patterns to follow:**
  - Prefer small, localized patches: add a new system or plugin rather than changing unrelated modules.
  - Plugins live in their module and expose a `plugin` to register (see `src/audio.rs`, `src/asset_tracking.rs`, `src/demo/mod.rs`).
  - Follow existing state and SystemSet patterns rather than introducing adhoc scheduling.
  - Respect clippy/lint allowances set in `Cargo.toml` (e.g., `too_many_arguments`, `type_complexity`).

- **Integration & external dependencies:**
  - Uses `bevy = 0.18`, `rand`, and `tracing` with specific features for dev profiling. Be cautious when changing `Cargo.toml` features because `package.metadata.bevy_cli` controls build behavior for `bevy run` commands.
  - Wasm target `wasm32-unknown-unknown` has a `getrandom` feature configured; web builds require the metadata entries in `Cargo.toml`.

- **When modifying game state or systems, examples:**
  - Add a system set variant in `src/main.rs` and append it to `configure_sets`.
  - Add a plugin in `src/<module>.rs` and register it in `main.rs`'s plugin tuple.
  - If touching asset loading, prefer the repo's asset watcher features for native dev (`dev_native`).

- **What the agent should do when proposing code:**
  - Provide a minimal `apply_patch` that only changes necessary files.
  - When changing `AppSystems` or states, update `src/main.rs` accordingly and run a quick static read of affected files.
  - Use existing folder/file examples: `src/demo/player.rs`, `src/screens/mod.rs`, `src/theme/palette.rs` for component, state, and style patterns.

- **Things to keep in mind:**
  - `get_single` or `get_single_mut` does not exist in Bevy 0.18; use `single()` or `single_mut()` instead.
  - `Camera2dBundle` is now `Camera2d`.
  - `SpriteBundle` is now `Sprite`.
  - If examples are needed, ask the user to add it to the example folder.

If anything here is unclear or you want more detail (examples, typical PR size, or test/run checks to run locally), tell me which area to expand.
