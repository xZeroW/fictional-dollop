//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{
    dev_tools::{
        fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig},
        states::log_transitions,
    },
    input::common_conditions::input_just_pressed,
    prelude::*,
    text::FontSmoothing,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::screens::Screen;

pub(super) struct DevToolsPlugin;

impl Plugin for DevToolsPlugin {
    fn build(&self, app: &mut App) {
        // Log `Screen` state transitions.
        app.add_systems(Update, log_transitions::<Screen>);

        // Toggle the debug overlay for UI.
        app.add_systems(
            Update,
            toggle_debug_ui.run_if(input_just_pressed(TOGGLE_UI_DEBUG_KEY)),
        );
        app.add_systems(
            Update,
            toggle_fps_overlay.run_if(input_just_pressed(TOGGLE_FPS_KEY)),
        );

        // Add the world inspector, which allows inspecting and editing the world at runtime.
        app.add_plugins((EguiPlugin::default(), WorldInspectorPlugin::new()));

        app.add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: 18.0,
                    font_smoothing: FontSmoothing::default(),
                    ..default()
                },
                text_color: Color::srgb(0.75, 0.95, 0.72),
                refresh_interval: core::time::Duration::from_millis(100),
                enabled: true,
                frame_time_graph_config: FrameTimeGraphConfig {
                    enabled: true,
                    min_fps: 30.0,
                    target_fps: 60.0,
                },
            },
        });
    }
}

const TOGGLE_UI_DEBUG_KEY: KeyCode = KeyCode::Backquote;
const TOGGLE_FPS_KEY: KeyCode = KeyCode::F1;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

fn toggle_fps_overlay(mut overlay: ResMut<FpsOverlayConfig>) {
    overlay.enabled = !overlay.enabled;
    overlay.frame_time_graph_config.enabled = overlay.enabled;
}
