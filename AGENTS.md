# AGENTS.md - Agentic Coding Guidelines for my_game

## Overview

This is a Bevy 2D game structured as modular Bevy plugins. The app entrypoint is `src/main.rs`, which wires together plugins from `src/` such as `audio`, `demo`, `menus`, `screens`, and `theme`.

Design patterns are inspired by [TheBevyFlock/bevy_new_2d](https://github.com/TheBevyFlock/bevy_new_2d/blob/main/docs/design.md).

---

## Build, Lint, and Test Commands

### Building the Project

```bash
# Development build (uses dynamic linking for faster compile times)
bevy run
# Or: cargo run

# Release build
bevy run --release
# Or: cargo run --release
```

### Running Tests

```bash
# Run all tests
cargo test

# Run a single test by name
cargo test test_name_here

# Run tests with output
cargo test -- --nocapture
```

### Adding new packages

```bash
# Add new package
cargo add <package_name>
```

---

## Code Style Guidelines

### Imports

- **Bevy imports**: Use `use bevy::prelude::*;` at the top of files
- **External crates**: Import specific items, e.g., `use leafwing_input_manager::prelude::*;`
- **Crate imports**: Use grouped imports with `crate::` prefix
- **Module ordering**: Std library → External crates → `crate::` modules

```rust
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    demo::{
        animation::PlayerAnimation,
        movement::{MovementController, ScreenWrap},
    },
    ron_asset::CharacterAssets,
};
```

### Formatting

- **Indentation**: 4 spaces (Rust default)
- **Line length**: Prefer lines under 100 characters when reasonable
- **Blank lines**: Use single blank line between function definitions and logical sections
- **Trailing commas**: Use trailing commas in struct literals and match arms

### Naming Conventions

| Element | Convention | Example |
|---------|------------|---------|
| Modules | snake_case | `mod audio;` |
| Structs/Enums | PascalCase | `struct Player` |
| Functions | snake_case | `fn spawn_player` |
| Variables | snake_case | `let max_speed` |
| Constants | SCREAMING_SNAKE_CASE | `const MAX_SPEED: f32 = 400.0;` |
| Component markers | PascalCase + descriptive | `struct Player;` |
| System sets | PascalCase | `enum AppSystems` |

### Types and Bevy Patterns

- **Bevy 0.18 API**: Use `Camera2d` (not `Camera2dBundle`), use `Sprite` (not `SpriteBundle`)
- **Query methods**: Use `single()` / `single_mut()` (not `get_single()`)
- **Component queries**: Use `Query<&Component>` for immutable, `Query<&mut Component>` for mutable
- **Resources**: Use `Res<T>` for immutable, `ResMut<T>` for mutable access
- **System parameters**: Order matters: `Commands`, `Query`, `Res`, `ResMut`, then other types

### Error Handling

- **Avoid panics in production**: Use `expect()` only for truly impossible states, prefer `ok()` / `unwrap_or()`
- **Bevy error patterns**: Use `Option` and `Result` types with appropriate fallbacks
- **Debug messages**: Use `tracing` crate for logging (already configured in `Cargo.toml`)

---

## Project Structure and Conventions

### Plugin Architecture

Each module should expose a `plugin` function:

```rust
pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, my_system.in_set(AppSystems::Update));
}
```

Register plugins in `src/main.rs`:

```rust
app.add_plugins((
    my_module::plugin,
    // ...
));
```

### State Management

- Use `#[derive(States)]` for game states
- Add states to `src/screens/mod.rs` following the `Screen` enum pattern
- Use `in_state()` guards for conditional system execution

### System Ordering

- Define `AppSystems` enum variants in `src/main.rs`
- Order variants in `configure_sets` call - this determines execution order
- Use `SystemSet` for systems that should be paused together

### Pause Handling

- Systems that should pause use: `.in_set(PausableSystems).run_if(in_state(Pause(false)))`
- The `Pause` state is a boolean state: `Pause(pub bool)`

---

## Clippy Allowances

The project has specific clippy allowances in `Cargo.toml`. Do not add `#[allow(...)]` attributes to bypass these:

```toml
[lints.clippy]
too_many_arguments = "allow"   # Bevy dependency injection needs many params
type_complexity = "allow"      # Queries naturally have complex types
nonstandard_macro_braces = "warn"  # Follow standard Rust brace style
```

---

## Bevy 0.18 Specific Notes

- `get_single` / `get_single_mut` do NOT exist - use `single()` / `single_mut()`
- `Camera2dBundle` is now `Camera2d`
- `SpriteBundle` is now `Sprite`
- Use `ActionState<T>` from `leafwing_input_manager` for input handling

---

## Asset Management

- Assets live in `assets/`
- Native dev builds support hot-reload via `dev_native` feature
- Use `bevy_asset_loader` for engine assets (Images, Audio, TextureAtlasLayout)
- Use `bevy_common_assets` for custom game data (stats, configurations)

| File | Purpose |
|------|---------|
| `assets/audio/` | Anything audio related, music, sfx |
| `assets/data/` | Screen state definitions, game data .ron files |
| `assets/images/` | Images to render as it is |
| `assets/sprites/` | Sprites (classes, enemies, weapons) |
| `assets/maps/` | Maps tilesets |

---

## Key Files Reference

| File | Purpose |
|------|---------|
| `src/main.rs` | App entry, plugin registration, system ordering |
| `src/screens/mod.rs` | Screen state definitions |
| `src/demo/player.rs` | Player component and input handling |
| `src/demo/movement.rs` | Movement controller system |
| `src/demo/level.rs` | Level spawning, LevelEntity resource |
| `src/demo/weapon.rs` | Weapon and bullet spawning |
| `src/demo/weapon_data.rs` | Weapon stats loaded from .ron |
| `src/enemies/mod.rs` | Enemy spawning system |
| `src/enemies/monster_data.rs` | Enemy assets and data from .ron |
| `src/theme/palette.rs` | Color palette definitions |
| `Cargo.toml` | Dependencies, features, lint config |

---

## Timer-Based Spawning

Use Bevy's built-in `on_timer` for systems that run at regular intervals:

```rust
use bevy::time::common_conditions::on_timer;
use std::time::Duration;

app.add_systems(
    Update,
    spawn_enemies
        .in_set(PausableSystems)
        .in_set(AppSystems::Update)
        .run_if(in_state(Screen::Gameplay))
        .run_if(on_timer(Duration::from_secs_f32(1.0))),
);
```

This approach is cleaner than manual timer management.

---

## Level Entity Pattern

When spawning entities that should be cleaned up together:

1. **Add a Level marker component and resource** in `src/demo/level.rs`:
```rust
#[derive(Component)]
pub struct Level;

#[derive(Resource)]
pub struct LevelEntity(pub Entity);
```

2. **Insert the resource when spawning Level**:
```rust
let level = commands.spawn((Level, ...)).id();
commands.insert_resource(LevelEntity(level));
```

3. **Spawn children using the resource**:
```rust
fn spawn_enemies(
    mut commands: Commands,
    level_entity: Option<Res<LevelEntity>>,
    // ...
) {
    let Some(level_entity) = level_entity else { return; };
    
    commands.entity(level_entity.0).with_children(|parent| {
        parent.spawn((Enemy, ...));
    });
}
```

This ensures all enemies are despawned when the Level despawns (via `DespawnOnExit`).

---

## bevy_common_assets for Game Data

Use `bevy_common_assets` to load custom game data from .ron files:

```rust
use bevy_common_assets::ron::RonAssetPlugin;

// Define your data struct
#[derive(serde::Deserialize, Asset, TypePath)]
pub struct Weapons(pub HashMap<String, WeaponData>);

#[derive(serde::Deserialize)]
pub struct WeaponData {
    pub name: String,
    pub damage: i32,
    pub velocity: f32,
    pub cooldown: f32,
    // ...
}

// Register the plugin
app.add_plugins(RonAssetPlugin::<Weapons>::new(&["weapon_data.ron"]));

// Load manually if needed
fn load_weapon_data(mut commands: Commands, server: Res<AssetServer>) {
    let handle = server.load("data/weapon_data.ron");
    commands.insert_resource(WeaponsHandle(handle));
}
```

This keeps game data (stats, configs) separate from engine assets.

---

## Bundle Functions Pattern

Write functions that return `impl Bundle` to define simple entity templates:

```rust
pub fn monster(health: u32, transform: Transform) -> impl Bundle {
    (
        Name::new("Monster"),
        Health::new(health),
        transform,
    )
}
```

Extend a bundle function with additional components:

```rust
pub fn boss_monster(transform: Transform) -> impl Bundle {
    (
        monster(1000, transform),
        Better,
        Faster,
        Stronger,
    )
}
```

Compose bundle functions for entity hierarchies:

```rust
pub fn dangerous_forest() -> impl Bundle {
    (
        Name::new("Dangerous Forest"),
        Transform::default(),
        children![
            monster(100, Transform::from_xyz(10.0, 0.0, 0.0)),
            monster(200, Transform::from_xyz(20.0, 0.0, 0.0)),
            boss_monster(Transform::from_xyz(30.0, 0.0, 0.0)),
        ],
    )
}
```

Spawn using bundle functions:

```rust
fn spawn_dangerous_forest(mut commands: Commands) {
    commands.spawn(dangerous_forest());
}
```

**Limitations:**
- **No dependency injection**: Pass required data as arguments
- **No replacing components**: Cannot modify components from extended bundles; use `commands.spawn(foo()).insert(Replacement)` instead

---

## Dev Tools Pattern

Group development-only systems in a dedicated plugin:

```rust
// dev_tools.rs
pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (draw_debug_lines, show_debug_console, show_fps_counter));
}
```

Only include this plugin in dev builds (via `#[cfg(debug_assertions)]` or feature gate) to guarantee it won't be included in release builds.

---

## When Proposing Code

- Provide minimal patches that only change necessary files
- Update `AppSystems` ordering in `main.rs` when adding new system sets
- Use existing files as examples: `src/demo/player.rs`, `src/screens/mod.rs`, `src/theme/palette.rs`
- Run `cargo check` before submitting changes
