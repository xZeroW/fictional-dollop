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

- Single binary crate `my_game`; app wiring is in `src/main.rs` via `AppPlugin`.
- Top-level plugins registered in `main.rs`: `AssetsPlugin`, `AudioPlugin`, `GamePlugin`, `HudPlugin`, `SystemsPlugin`, `ListenersPlugin`, `MenusPlugin`, `ScreensPlugin`, `ThemePlugin`, `EnemiesPlugin`, and `DevToolsPlugin` only behind `feature = "dev"`.
- `src/game/` owns gameplay setup and templates (`level`, `map`, `player`, `weapon_data`); `src/systems/` owns frame systems, gameplay lifecycle systems, and their run-scoped resources; `src/listeners/` owns message listeners; `src/enemies/` owns enemy data/assets/systems.
- `Screen` lives in `src/screens/mod.rs`; `Menu` lives in `src/menus/mod.rs`; `Pause(pub bool)` and `AppSystems` live in `src/main.rs`.
- If adding an `AppSystems` variant, add it to the chained `configure_sets(Update, ...)` order in `main.rs` in the same change.
- Put gameplay resources with `OnEnter`/`OnExit` lifecycle systems in `src/systems/` (for example wave state or monster progression), not in `src/game/`, unless the module is only setup/template code.
- Gameplay systems that should pause must be in `PausableSystems`; most should also gate with `run_if(in_state(Screen::Gameplay))`.
- Level-owned gameplay entities should be parented under `LevelEntity` from `src/game/level.rs`; `Level` has `DespawnOnExit(Screen::Gameplay)` and resources are cleaned up on exit.

## Patterns

- New plugins should be named `*Plugin` structs with `impl Plugin`, matching `src/systems/*`; do not add bare `fn plugin(app: &mut App)` registrations.
- Parent modules register child plugins; keep `src/main.rs` focused on top-level app modules.
- Entity templates are usually functions returning `impl Bundle` (`game/player.rs`, `systems/auto_attack.rs`, `enemies/data.rs`); prefer that before adding custom bundle structs.
- Cross-system reactions use Bevy messages/listeners under `src/messages.rs` and `src/listeners/`; avoid coupling collision, damage, and death behavior into producer systems.
- Runtime cleanup is state-driven with `DespawnOnExit(Screen::Gameplay)` plus resources inserted on enter and removed on exit.

## Assets And Data

- Dynamic asset manifests live in `assets/data/*.assets.ron` and are loaded by `bevy_asset_loader` in `src/assets.rs`.
- Weapon stats are `assets/data/weapon_data.ron` via `bevy_common_assets` in `src/game/weapon_data.rs`.
- Enemy stats are `assets/data/enemies_data.ron`; enemy visuals are `assets/data/enemies.assets.ron`. Keep RON keys aligned with `asset_key` and sprite paths.
- The map is currently hardcoded in `src/game/map.rs` as `assets/maps/map1.tmx` through `bevy_ecs_tiled`.

## Bevy 0.18 Gotchas

- Use `Camera2d` and `Sprite`, not the removed `Camera2dBundle`/`SpriteBundle` APIs.
- Query single-entity access returns `Result`; use `single()` / `single_mut()` and handle `Err` instead of older `get_single*` APIs.
- `children!` must use square brackets; `clippy.toml` enforces this via `nonstandard_macro_braces`.
- Input uses `leafwing_input_manager`; player input is wired through `PlayerAction` in `src/game/player.rs`.

## Existing Guidance

- `docs/systems/wave.md` is current for wave lifecycle and explains why `AppSystems::WaveTransitions` runs after normal updates.
