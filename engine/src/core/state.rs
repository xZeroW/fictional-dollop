use bevy::prelude::*;

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
pub struct Pause(pub bool);

#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PausableSystems;

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Screen {
    #[default]
    Splash,
    Title,
    Loading,
    Gameplay,
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
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
