//! Reusable UI widgets & theming.

// Unused utilities may trigger this lints undesirably.
#![allow(dead_code)]

pub mod interaction;
pub mod palette;
pub mod widget;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::{interaction::InteractionPalette, palette as ui_palette, widget};
}

use bevy::prelude::*;

pub(super) struct ThemePlugin;

impl Plugin for ThemePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(interaction::InteractionPlugin);
    }
}
