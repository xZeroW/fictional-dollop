//! The game's main screen states and transitions between them.

mod gameplay;
mod loading;
mod splash;
mod title;

use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;
pub use engine::Screen;

#[derive(AutoPlugin)]
pub(super) struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    #[auto_plugin]
    fn build(&self, app: &mut App) {
        app.add_plugins((
            gameplay::GameplayScreenPlugin,
            loading::LoadingScreenPlugin,
            splash::SplashScreenPlugin,
            title::TitleScreenPlugin,
        ));
    }
}
