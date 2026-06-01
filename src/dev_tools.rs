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
use bevy_inspector_egui::{
    bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui},
    quick::WorldInspectorPlugin,
};

use crate::{
    components::{Player, Weapon},
    game::weapon_data::{Weapons, WeaponsHandle},
    screens::Screen,
};

pub(super) struct DevToolsPlugin;

impl Plugin for DevToolsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WeaponSelectorVisible>();
        app.init_resource::<WorldInspectorVisible>();

        // Log `Screen` state transitions.
        app.add_systems(Update, log_transitions::<Screen>);

        // Toggle the debug overlay for UI.
        app.add_systems(
            Update,
            toggle_debug_ui.run_if(input_just_pressed(TOGGLE_UI_DEBUG_KEY)),
        );
        app.add_systems(
            Update,
            toggle_debug_tools.run_if(input_just_pressed(TOGGLE_DEBUG_TOOLS_KEY)),
        );
        app.add_systems(
            Update,
            toggle_weapon_selector.run_if(input_just_pressed(TOGGLE_WEAPON_SELECTOR_KEY)),
        );

        // Add the world inspector, which allows inspecting and editing the world at runtime.
        app.add_plugins((
            EguiPlugin::default(),
            WorldInspectorPlugin::new().run_if(world_inspector_visible),
        ));
        app.add_systems(
            EguiPrimaryContextPass,
            weapon_selector_ui.run_if(in_state(Screen::Gameplay).and(weapon_selector_visible)),
        );

        app.add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: 18.0,
                    font_smoothing: FontSmoothing::default(),
                    ..default()
                },
                text_color: Color::srgb(0.75, 0.95, 0.72),
                refresh_interval: core::time::Duration::from_millis(100),
                enabled: false,
                frame_time_graph_config: FrameTimeGraphConfig {
                    enabled: false,
                    min_fps: 30.0,
                    target_fps: 60.0,
                },
            },
        });
    }
}

const TOGGLE_UI_DEBUG_KEY: KeyCode = KeyCode::Backquote;
const TOGGLE_DEBUG_TOOLS_KEY: KeyCode = KeyCode::F1;
const TOGGLE_WEAPON_SELECTOR_KEY: KeyCode = KeyCode::F2;

#[derive(Resource, Default)]
struct WeaponSelectorVisible(bool);

#[derive(Resource, Default)]
struct WorldInspectorVisible(bool);

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

fn toggle_debug_tools(
    mut overlay: ResMut<FpsOverlayConfig>,
    mut world_inspector: ResMut<WorldInspectorVisible>,
) {
    overlay.enabled = !overlay.enabled;
    overlay.frame_time_graph_config.enabled = overlay.enabled;
    world_inspector.0 = !world_inspector.0;
}

fn world_inspector_visible(visible: Res<WorldInspectorVisible>) -> bool {
    visible.0
}

fn toggle_weapon_selector(mut visible: ResMut<WeaponSelectorVisible>) {
    visible.0 = !visible.0;
}

fn weapon_selector_visible(visible: Res<WeaponSelectorVisible>) -> bool {
    visible.0
}

fn weapon_selector_ui(
    mut contexts: EguiContexts,
    mut weapon_query: Query<&mut Weapon, With<Player>>,
    weapons_handle: Res<WeaponsHandle>,
    weapons_assets: Res<Assets<Weapons>>,
) -> Result {
    let Ok(mut weapon) = weapon_query.single_mut() else {
        return Ok(());
    };
    let Some(weapons) = weapons_assets.get(&weapons_handle.0) else {
        return Ok(());
    };

    let mut weapon_keys = weapons.0.keys().collect::<Vec<_>>();
    weapon_keys.sort();

    egui::Window::new("Weapon Selector").show(contexts.ctx_mut()?, |ui| {
        ui.label(format!("Equipped: {}", weapon.key));
        ui.separator();

        for key in weapon_keys {
            let selected = weapon.key == *key;
            let weapon_name = weapons
                .0
                .get(key)
                .map(|weapon_data| weapon_data.name.as_str())
                .unwrap_or(key.as_str());
            let label = format!("{weapon_name} ({key})");

            if ui.selectable_label(selected, label).clicked() && !selected {
                weapon.equip(key.clone());
            }
        }
    });

    Ok(())
}
