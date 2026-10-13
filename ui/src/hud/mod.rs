use bevy::prelude::*;

use crate::{
    AppSystems,
    domain::{
        entities::{Health, Player},
        inventory::RunInventory,
        progression::{MonsterProgression, WaveState},
    },
    screen::Screen,
};

const HEALTH_FILL_COLOR: Color = Color::srgb(0.82, 0.13, 0.16);
const HEALTH_BACK_COLOR: Color = Color::srgba(0.07, 0.05, 0.06, 0.92);
const HEALTH_FRAME_COLOR: Color = Color::srgba(0.02, 0.018, 0.02, 0.88);
const HEALTH_TEXT_COLOR: Color = Color::srgb(0.96, 0.88, 0.78);
const RUN_TEXT_COLOR: Color = Color::srgb(0.84, 0.92, 0.96);
#[derive(Component, Clone, Default)]
struct PlayerHealthFill {
    fraction: f32,
}

#[derive(Component, Clone, Default)]
struct PlayerHealthText;

#[derive(Component, Clone, Default)]
struct RunStatsText;

pub(super) struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Gameplay), hud.spawn());
        app.add_systems(
            Update,
            (update_player_health_hud, update_run_stats_hud)
                .in_set(AppSystems::Update)
                .run_if(in_state(Screen::Gameplay)),
        );
    }
}

fn hud() -> impl Scene {
    bsn! {
        Name("HUD")
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
        }
        Pickable { should_block_lower: false, is_hoverable: false }
        template_value(DespawnOnExit(Screen::Gameplay))
        Children [
            (
                Name("HUD Safe Area")
                Node {
                    position_type: PositionType::Absolute,
                    left: px(20),
                    right: px(20),
                    top: px(20),
                    bottom: px(20),
                }
                Pickable { should_block_lower: false, is_hoverable: false }
                Children [
                    (
                        Name("Run Stats Text")
                        RunStatsText
                        Node {
                            position_type: PositionType::Absolute,
                            left: px(0),
                            top: px(0),
                        }
                        Text("")
                        TextFont { font_size: FontSize::Px(32.0) }
                        TextColor(RUN_TEXT_COLOR)
                        Pickable { should_block_lower: false, is_hoverable: false }
                    ),
                    (
                        Name("Player Health Panel")
                        Node {
                            position_type: PositionType::Absolute,
                            left: percent(29),
                            bottom: px(0),
                            width: percent(42),
                            height: percent(7),
                        }
                        BackgroundColor(HEALTH_FRAME_COLOR)
                        Pickable { should_block_lower: false, is_hoverable: false }
                        Children [
                            (
                                Name("Player Health Backing")
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: percent(6),
                                    top: percent(36),
                                    width: percent(88),
                                    height: percent(28),
                                }
                                BackgroundColor(HEALTH_BACK_COLOR)
                                Pickable { should_block_lower: false, is_hoverable: false }
                            ),
                            (
                                Name("Player Health Fill")
                                PlayerHealthFill { fraction: -1.0 }
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: percent(6),
                                    top: percent(36),
                                    width: percent(88),
                                    height: percent(28),
                                }
                                BackgroundColor(HEALTH_FILL_COLOR)
                                Pickable { should_block_lower: false, is_hoverable: false }
                            ),
                            (
                                Name("Player Health Text")
                                PlayerHealthText
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: percent(0),
                                    top: percent(0),
                                    width: percent(100),
                                    height: percent(100),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                }
                                Text("Health")
                                TextFont { font_size: FontSize::Px(28.0) }
                                TextColor(HEALTH_TEXT_COLOR)
                                Pickable { should_block_lower: false, is_hoverable: false }
                            ),
                        ]
                    ),
                ]
            ),
        ]
    }
}

fn update_player_health_hud(
    player_health: Query<&Health, With<Player>>,
    mut fill: Query<(&mut Node, &mut PlayerHealthFill)>,
    mut text: Query<&mut Text, (With<PlayerHealthText>, Without<RunStatsText>)>,
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

    if let Ok((mut fill_node, mut fill)) = fill.single_mut()
        && (fill.fraction - health_fraction).abs() > f32::EPSILON
    {
        fill.fraction = health_fraction;
        fill_node.width = percent(88.0 * health_fraction);
    }

    if let Ok(mut text) = text.single_mut() {
        let next_text = format!("{current:.0} / {max:.0}");
        if text.as_str() != next_text {
            text.0 = next_text;
        }
    }
}

fn update_run_stats_hud(
    mut text: Query<&mut Text, (With<RunStatsText>, Without<PlayerHealthText>)>,
    wave_state: Res<WaveState>,
    progression: Res<MonsterProgression>,
    inventory: Res<RunInventory>,
) {
    let Ok(mut text) = text.single_mut() else {
        return;
    };

    let time_remaining = (wave_state.timer.duration().as_secs_f32()
        - wave_state.timer.elapsed_secs())
    .max(0.0)
    .ceil();
    let next_text = format!(
        "Wave {}  Time {:.0}s\nCorruption {}\nEnemy: HP x{:.2}  DMG x{:.2}  SPD x{:.2}\nRewards: Quantity x{:.2}  Rarity x{:.2}\n{}",
        wave_state.current_wave,
        time_remaining,
        progression.corruption,
        progression.enemy_health_mult,
        progression.enemy_damage_mult,
        progression.enemy_speed_mult,
        progression.reward_quantity_mult,
        progression.reward_rarity_mult,
        inventory.summary(),
    );

    if text.as_str() != next_text {
        text.0 = next_text;
    }
}
