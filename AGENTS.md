# AGENTS.md - Agentic Coding Guidelines for my_rebanho

## Overview

This is a Bevy 2D game structured as modular Bevy plugins. The app entrypoint is `src/main.rs`, which wires together plugins from `src/` such as `audio`, `demo`, `menus`, `screens`, and `theme`.

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

### Linting and Formatting

```bash
# Check formatting (rustfmt)
cargo fmt --check

# Format code automatically
cargo fmt

# Run clippy lints
cargo clippy

# Run all checks (fmt + clippy + tests)
cargo check
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

- Assets live in `assets/images/` and `assets/audio/`
- Native dev builds support hot-reload via `dev_native` feature

---

## Key Files Reference

| File | Purpose |
|------|---------|
| `src/main.rs` | App entry, plugin registration, system ordering |
| `src/screens/mod.rs` | Screen state definitions |
| `src/demo/player.rs` | Player component and input handling |
| `src/demo/movement.rs` | Movement controller system |
| `src/theme/palette.rs` | Color palette definitions |
| `Cargo.toml` | Dependencies, features, lint config |

---

## When Proposing Code

- Provide minimal patches that only change necessary files
- Update `AppSystems` ordering in `main.rs` when adding new system sets
- Use existing files as examples: `src/demo/player.rs`, `src/screens/mod.rs`, `src/theme/palette.rs`
- Run `cargo check` before submitting changes
