# Wave System

The wave system tracks timed gameplay waves. Each wave lasts for `WAVE_DURATION`
seconds, currently `60.0`.

## State

Wave state is stored in the `WaveState` resource in `src/systems/wave.rs`.

- `current_wave`: the active wave number, starting at `1`.
- `timer`: a repeating Bevy `Timer` using `config::WAVE_DURATION`.

`WaveState` derives `Reflect` and is registered with Bevy so it can be inspected
in the world inspector under resources.

## Lifecycle

`WaveState` is created when entering `Screen::Gameplay`.

It is removed when exiting `Screen::Gameplay`, so it does not persist when
returning to the main menu.

## Update Order

The wave timer runs during `Update` with these constraints:

- only while in `Screen::Gameplay`
- only while the game is not paused through `PausableSystems`
- in `AppSystems::WaveTransitions`, after normal gameplay updates

Running after gameplay updates ensures wave cleanup happens at the end of the
frame.

## Wave Completion

When the timer finishes, the system:

1. Despawns all entities with the `Enemy` component.
2. Resets `EnemySpawner.spawned_count` to `0`.
3. Increments `current_wave`.

Resetting `spawned_count` is required because enemy spawning uses that counter
to enforce the enemy cap.

## Related Files

- `src/systems/wave.rs`: wave state and wave transition system
- `src/config.rs`: `WAVE_DURATION`
- `src/main.rs`: `AppSystems::WaveTransitions` ordering
- `src/game/level.rs`: gameplay resource cleanup for level-owned resources
