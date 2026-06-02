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
    components::{Bullet, Enemy, Player, Weapon},
    config,
    game::weapon_data::{Weapons, WeaponsHandle},
    screens::Screen,
};

pub(super) struct DevToolsPlugin;

impl Plugin for DevToolsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WeaponSelectorVisible>();
        app.init_resource::<WorldInspectorVisible>();
        app.init_resource::<CollisionGizmosVisible>();

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
        app.add_systems(
            Update,
            toggle_collision_gizmos.run_if(input_just_pressed(TOGGLE_COLLISION_GIZMOS_KEY)),
        );
        app.add_systems(
            Update,
            draw_collision_gizmos
                .run_if(in_state(Screen::Gameplay))
                .run_if(collision_gizmos_visible),
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
const TOGGLE_COLLISION_GIZMOS_KEY: KeyCode = KeyCode::F3;
const COLLISION_GIZMO_VIEW_RADIUS: f32 = 500.0;

#[derive(Resource, Default)]
struct WeaponSelectorVisible(bool);

#[derive(Resource, Default)]
struct WorldInspectorVisible(bool);

#[derive(Resource, Default)]
struct CollisionGizmosVisible(bool);

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

fn toggle_collision_gizmos(mut visible: ResMut<CollisionGizmosVisible>) {
    visible.0 = !visible.0;
}

fn weapon_selector_visible(visible: Res<WeaponSelectorVisible>) -> bool {
    visible.0
}

fn collision_gizmos_visible(visible: Res<CollisionGizmosVisible>) -> bool {
    visible.0
}

fn draw_collision_gizmos(
    mut gizmos: Gizmos,
    player_query: Query<&GlobalTransform, With<Player>>,
    enemy_query: Query<&GlobalTransform, With<Enemy>>,
    bullet_query: Query<&GlobalTransform, With<Bullet>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let player_pos = player_transform.translation().truncate();
    let view_radius_squared = COLLISION_GIZMO_VIEW_RADIUS * COLLISION_GIZMO_VIEW_RADIUS;

    gizmos.circle_2d(
        player_pos,
        config::PLAYER_BODY_RADIUS,
        Color::srgb(0.2, 0.8, 1.0),
    );
    gizmos.circle_2d(
        player_pos,
        config::PLAYER_ENEMY_CONTACT_RADIUS,
        Color::srgb(1.0, 0.9, 0.1),
    );

    for enemy_transform in &enemy_query {
        let enemy_pos = enemy_transform.translation().truncate();
        if player_pos.distance_squared(enemy_pos) > view_radius_squared {
            continue;
        }

        gizmos.circle_2d(
            enemy_pos,
            config::ENEMY_BODY_RADIUS,
            Color::srgb(1.0, 0.2, 0.2),
        );
    }

    for bullet_transform in &bullet_query {
        let bullet_pos = bullet_transform.translation().truncate();
        gizmos.circle_2d(
            bullet_pos,
            config::BULLET_ENEMY_COLLISION_RADIUS,
            Color::srgb(0.4, 1.0, 0.4),
        );
    }
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
