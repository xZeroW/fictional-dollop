# AGENTS.md

## Commands

- Local dev run: `cargo run` uses the default `dev_native` feature set from `Cargo.toml`.
- If Bevy CLI is installed, `.zed/tasks.json` uses `bevy run`; it is not required for normal local checks.
- Cargo release equivalent: `cargo run --release --no-default-features`. Do not use plain `cargo run --release` when checking release behavior; default features include dev tooling/hot reload.
- Fast local compile check: `cargo check --locked`.
- Tests: `cargo test --locked`; focused test filter: `cargo test --locked test_name`.
- Formatting check from CI: `cargo fmt --all -- --check`.
- Docs check from CI: `cargo doc --locked --workspace --profile ci --all-features --document-private-items --no-deps`.
- Clippy shape from CI: `cargo clippy --locked --workspace --all-targets --profile ci --all-features`.
- Bevy lint shape from CI, if `bevy_lint` is installed: `bevy_lint --locked --workspace --all-targets --profile ci --all-features`.
- CI test shape: `cargo test --locked --workspace --all-targets --profile ci --no-fail-fast`.
- CI uses nightly `nightly-2025-06-26` plus unstable `RUSTFLAGS` (`-Zshare-generics`, `-Zthreads`, and cranelift for tests). Local stable commands are useful but not exact CI parity.
- Web compile check from CI needs `wasm32-unknown-unknown` and `RUSTFLAGS='--cfg getrandom_backend="wasm_js"'`: `cargo check --config 'profile.web.inherits="dev"' --profile ci --no-default-features --features dev --target wasm32-unknown-unknown`.

## Architecture

- Workspace crates are `my_game` (the executable and composition root), `engine` (gameplay/runtime), and `ui`.
- `my_game` selects Bevy's platform plugins and composes `engine::EnginePlugin` with `ui::UiPlugin`.
- `engine/src/core/` owns shared states and scheduling contracts; `engine/src/domain/` owns feature-oriented gameplay; `engine/src/infrastructure/` owns configuration, asset collections, and data loaders.
- `Screen`, `Menu`, `Pause(pub bool)`, and `AppSystems` live in `engine/src/core/state.rs` and are re-exported from `engine`.
- If adding an `AppSystems` variant, add it to the chained `configure_sets(Update, ...)` order in `engine/src/lib.rs` in the same change.
- Put gameplay resources and their `OnEnter`/`OnExit` lifecycle systems in the owning domain.
- Gameplay systems that should pause must be in `PausableSystems`; most should also gate with `run_if(in_state(Screen::Gameplay))`.
- Level-owned gameplay entities should be parented under `LevelEntity` from `domain/world/level.rs`; `Level` has `DespawnOnExit(Screen::Gameplay)` and resources are cleaned up on exit.

## Patterns

- New plugins should be named `*Plugin` structs with `impl Plugin`; do not add bare `fn plugin(app: &mut App)` registrations.
- Parent domain modules register child plugins; keep `my_game/src/main.rs` focused on platform setup and plugin composition.
- Entity templates are usually functions returning `impl Bundle` (for example `domain/entities/player.rs`, `domain/combat/auto_attack.rs`, and `domain/enemies/data.rs`); prefer that before adding custom bundle structs.
- Cross-system messages and reactions live in the domain that owns their meaning (for example combat damage messages); avoid global mechanism-based folders.
- Runtime cleanup is state-driven with `DespawnOnExit(Screen::Gameplay)` plus resources inserted on enter and removed on exit.

## Assets And Data

- Dynamic asset manifests live in `assets/data/*.assets.ron` and are loaded by `bevy_asset_loader` in `engine/src/infrastructure/assets.rs`.
- Weapon stats are `assets/data/weapon_data.ron` via `bevy_common_assets` in `engine/src/infrastructure/weapon.rs`.
- Enemy stats are `assets/data/enemies_data.ron`; enemy visuals are `assets/data/enemies.assets.ron`. Keep RON keys aligned with `asset_key` and sprite paths.
- The map is currently hardcoded in `engine/src/domain/world/map.rs` as `assets/maps/map1.tmx` through `bevy_ecs_tiled`.

## Bevy 0.19 Gotchas

- Use `Camera2d` and `Sprite`, not the removed `Camera2dBundle`/`SpriteBundle` APIs.
- Query single-entity access returns `Result`; use `single()` / `single_mut()` and handle `Err` instead of older `get_single*` APIs.
- `children!` must use square brackets; `clippy.toml` enforces this via `nonstandard_macro_braces`.
- Input uses `leafwing_input_manager`; player input is wired through `PlayerAction` in `engine/src/domain/entities/player.rs`.

## Existing Guidance

- `docs/systems/wave.md` is current for wave lifecycle and explains why `AppSystems::WaveTransitions` runs after normal updates.
