use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

#[auto_configure_system_set(plugin = crate::EnginePlugin, schedule = Update, chain)]
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum AppSystems {
    TickTimers,
    RecordInput,
    Update,
    ResolveContacts,
    SpatialIndex,
    SpatialQueries,
    CollisionEvents,
    ApplyDamage,
    DamageEvents,
    DeathEvents,
    WaveTransitions,
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[auto_init_state(plugin = crate::EnginePlugin)]
pub struct Pause(pub bool);

#[auto_configure_system_set(
    plugin = crate::EnginePlugin,
    schedule = Update,
    config(run_if = in_state(Pause(false)))
)]
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PausableSystems;

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[auto_init_state(plugin = crate::EnginePlugin)]
pub enum Screen {
    #[default]
    Splash,
    Title,
    Loading,
    Gameplay,
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[auto_init_state(plugin = crate::EnginePlugin)]
pub enum Menu {
    #[default]
    None,
    Main,
    Credits,
    Settings,
    Inventory,
    Equipment,
    Pause,
    GameOver,
}
