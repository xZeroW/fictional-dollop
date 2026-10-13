use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

pub mod core;
pub mod domain;
pub mod infrastructure;

pub use core::{AppSystems, Menu, PausableSystems, Pause, Screen};

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<Screen>();
        app.init_state::<Menu>();
        app.init_state::<Pause>();

        app.add_plugins((infrastructure::InfrastructurePlugin, domain::DomainPlugin));

        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
                AppSystems::ResolveContacts,
                AppSystems::SpatialIndex,
                AppSystems::SpatialQueries,
                AppSystems::CollisionEvents,
                AppSystems::ApplyDamage,
                AppSystems::DamageEvents,
                AppSystems::DeathEvents,
                AppSystems::WaveTransitions,
            )
                .chain(),
        );
        app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));
    }
}
