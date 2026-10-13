use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

pub mod core;
pub mod domain;
pub mod infrastructure;

pub use core::{AppSystems, Menu, PausableSystems, Pause, Screen};

#[derive(AutoPlugin)]
pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    #[auto_plugin]
    fn build(&self, app: &mut App) {
        app.add_plugins((infrastructure::InfrastructurePlugin, domain::DomainPlugin));
    }
}
