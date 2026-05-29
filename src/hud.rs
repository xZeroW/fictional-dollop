use bevy::{camera::ClearColorConfig, camera::visibility::RenderLayers, prelude::*};
use bevy_lunex::{NoLunexPicking, RecomputeUiLayout, prelude::*};

use crate::{
    AppSystems,
    components::{Health, Player},
    screens::Screen,
};

const HEALTH_FILL_COLOR: Color = Color::srgb(0.82, 0.13, 0.16);
const HEALTH_BACK_COLOR: Color = Color::srgba(0.07, 0.05, 0.06, 0.92);
const HEALTH_FRAME_COLOR: Color = Color::srgba(0.02, 0.018, 0.02, 0.88);
const HEALTH_TEXT_COLOR: Color = Color::srgb(0.96, 0.88, 0.78);
const HUD_CAMERA_ORDER: isize = 10;
const HUD_RENDER_LAYER: usize = 1;

#[derive(Component)]
struct HudCamera;

#[derive(Component)]
struct PlayerHealthFill {
    fraction: f32,
}

#[derive(Component)]
struct PlayerHealthText;

pub(super) struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiLunexPlugins);
        app.add_systems(Startup, spawn_hud_camera);
        app.add_systems(OnEnter(Screen::Gameplay), spawn_hud);
        app.add_systems(
            Update,
            update_player_health_hud
                .in_set(AppSystems::Update)
                .run_if(in_state(Screen::Gameplay)),
        );
    }
}

fn spawn_hud_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("HUD Camera"),
        HudCamera,
        Camera2d,
        Camera {
            order: HUD_CAMERA_ORDER,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        Msaa::Off,
        UiSourceCamera::<0>,
        RenderLayers::layer(HUD_RENDER_LAYER),
    ));
}

fn spawn_hud(mut commands: Commands, camera: Query<Entity, With<HudCamera>>) {
    let Ok(camera) = camera.single() else {
        return;
    };

    let root = commands
        .spawn((
            Name::new("HUD"),
            UiLayoutRoot::new_2d(),
            UiFetchFromCamera::<0>,
            NoLunexPicking,
            RenderLayers::layer(HUD_RENDER_LAYER),
            DespawnOnExit(Screen::Gameplay),
        ))
        .with_children(|ui| {
            ui.spawn((
                Name::new("HUD Safe Area"),
                UiLayout::boundary()
                    .pos1(Ab(20.0))
                    .pos2(Rl(100.0) - Ab(20.0))
                    .pack(),
                UiDepth::Set(19.0),
                NoLunexPicking,
                RenderLayers::layer(HUD_RENDER_LAYER),
                Pickable::IGNORE,
            ))
            .with_children(|ui| {
                ui.spawn((
                    Name::new("Player Health Panel"),
                    UiLayout::window()
                        .pos((Rl(50.0), Rl(88.0)))
                        .anchor(Anchor::CENTER)
                        .size((Rw(42.0), Rh(7.0)))
                        .pack(),
                    UiDepth::Set(20.0),
                    NoLunexPicking,
                    Sprite::from_color(HEALTH_FRAME_COLOR, Vec2::ONE),
                    RenderLayers::layer(HUD_RENDER_LAYER),
                    Pickable::IGNORE,
                ))
                .with_children(|ui| {
                    ui.spawn((
                        Name::new("Player Health Backing"),
                        UiLayout::window()
                            .pos((Rl(50.0), Rl(58.0)))
                            .anchor(Anchor::CENTER)
                            .size((Rl(88.0), Rl(28.0)))
                            .pack(),
                        UiDepth::Set(21.0),
                        NoLunexPicking,
                        Sprite::from_color(HEALTH_BACK_COLOR, Vec2::ONE),
                        RenderLayers::layer(HUD_RENDER_LAYER),
                        Pickable::IGNORE,
                    ));

                    ui.spawn((
                        Name::new("Player Health Fill"),
                        PlayerHealthFill { fraction: -1.0 },
                        UiLayout::window()
                            .pos((Rl(6.0), Rl(58.0)))
                            .anchor(Anchor::CENTER_LEFT)
                            .size((Rl(88.0), Rl(28.0)))
                            .pack(),
                        UiDepth::Set(22.0),
                        NoLunexPicking,
                        Sprite::from_color(HEALTH_FILL_COLOR, Vec2::ONE),
                        RenderLayers::layer(HUD_RENDER_LAYER),
                        Pickable::IGNORE,
                    ));

                    ui.spawn((
                        Name::new("Player Health Text"),
                        PlayerHealthText,
                        UiLayout::window()
                            .pos((Rl(50.0), Rl(30.0)))
                            .anchor(Anchor::CENTER)
                            .pack(),
                        UiTextSize::from(Rh(30.0)),
                        UiDepth::Set(23.0),
                        NoLunexPicking,
                        Text2d::new("Health"),
                        TextFont::from_font_size(64.0),
                        TextColor(HEALTH_TEXT_COLOR),
                        RenderLayers::layer(HUD_RENDER_LAYER),
                        Pickable::IGNORE,
                    ));
                });
            });
        })
        .id();

    commands.entity(camera).add_child(root);
}

fn update_player_health_hud(
    mut commands: Commands,
    player_health: Query<&Health, With<Player>>,
    mut fill: Query<(&mut UiLayout, &mut PlayerHealthFill)>,
    mut text: Query<&mut Text2d, With<PlayerHealthText>>,
) {
    let health = player_health.single().ok();
    let (current, max, health_fraction) = health
        .map(|health| {
            let max = health.max.max(0.0);
            let current = health.current.clamp(0.0, max);
            let fraction = if max > 0.0 { current / max } else { 0.0 };

            (current, max, fraction)
        })
        .unwrap_or((0.0, 0.0, 0.0));

    if let Ok((mut fill_layout, mut fill)) = fill.single_mut()
        && (fill.fraction - health_fraction).abs() > f32::EPSILON
        && let Some(window) = fill_layout.get_mut_window(UiBase::id())
    {
        fill.fraction = health_fraction;
        window.set_width(Rl(88.0 * health_fraction));
        commands.trigger(RecomputeUiLayout);
    }

    if let Ok(mut text) = text.single_mut() {
        let next_text = format!("{current:.0} / {max:.0}");
        if text.as_str() != next_text {
            text.0 = next_text;
        }
    }
}
