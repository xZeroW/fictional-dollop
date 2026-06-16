//! The settings menu.
//!
//! Additional settings and accessibility options should go here.

use bevy::{
    audio::Volume, ecs::spawn::SpawnWith, input::common_conditions::input_just_pressed, prelude::*,
};

use crate::{config::GameSettings, menus::Menu, screens::Screen, theme::prelude::*};

pub(super) struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Menu::Settings), spawn_settings_menu);
        app.add_systems(
            Update,
            go_back.run_if(in_state(Menu::Settings).and(input_just_pressed(KeyCode::Escape))),
        );

        app.add_systems(
            Update,
            (update_global_volume_label, update_enemy_nameplates_label)
                .run_if(in_state(Menu::Settings)),
        );
    }
}

fn spawn_settings_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Settings Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Settings),
        children![
            widget::header("Settings"),
            settings_grid(),
            widget::button("Back", go_back_on_click),
        ],
    ));
}

fn settings_grid() -> impl Bundle {
    (
        Name::new("Settings Grid"),
        Node {
            display: Display::Grid,
            row_gap: px(10),
            column_gap: px(30),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        children![
            (
                widget::label("Master Volume"),
                Node {
                    justify_self: JustifySelf::End,
                    align_self: AlignSelf::Center,
                    ..default()
                }
            ),
            global_volume_widget(),
            (
                widget::label("Enemy Nameplates"),
                Node {
                    justify_self: JustifySelf::End,
                    align_self: AlignSelf::Center,
                    ..default()
                }
            ),
            enemy_nameplates_widget(),
        ],
    )
}

fn global_volume_widget() -> impl Bundle {
    (
        Name::new("Global Volume Widget"),
        Node {
            justify_self: JustifySelf::Start,
            ..default()
        },
        children![
            widget::button_small("-", lower_global_volume),
            (
                Name::new("Current Volume"),
                Node {
                    padding: UiRect::horizontal(px(10)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(widget::label(""), GlobalVolumeLabel)],
            ),
            widget::button_small("+", raise_global_volume),
        ],
    )
}

fn enemy_nameplates_widget() -> impl Bundle {
    (
        Name::new("Enemy Nameplates Widget"),
        Node {
            justify_self: JustifySelf::Start,
            ..default()
        },
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent
                .spawn((
                    Name::new("Enemy Nameplates Toggle"),
                    Button,
                    Node {
                        width: px(160),
                        height: px(48),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        border_radius: BorderRadius::MAX,
                        ..default()
                    },
                    BackgroundColor(ui_palette::BUTTON_BACKGROUND),
                    InteractionPalette {
                        none: ui_palette::BUTTON_BACKGROUND,
                        hovered: ui_palette::BUTTON_HOVERED_BACKGROUND,
                        pressed: ui_palette::BUTTON_PRESSED_BACKGROUND,
                    },
                    children![(
                        Name::new("Enemy Nameplates Toggle Text"),
                        EnemyNameplatesLabel,
                        Text("".to_string()),
                        TextFont::from_font_size(24.0),
                        TextColor(ui_palette::BUTTON_TEXT),
                        Pickable::IGNORE,
                    )],
                ))
                .observe(toggle_enemy_nameplates);
        })),
    )
}

const MIN_VOLUME: f32 = 0.0;
const MAX_VOLUME: f32 = 3.0;

fn lower_global_volume(_: On<Pointer<Click>>, mut global_volume: ResMut<GlobalVolume>) {
    let linear = (global_volume.volume.to_linear() - 0.1).max(MIN_VOLUME);
    global_volume.volume = Volume::Linear(linear);
}

fn raise_global_volume(_: On<Pointer<Click>>, mut global_volume: ResMut<GlobalVolume>) {
    let linear = (global_volume.volume.to_linear() + 0.1).min(MAX_VOLUME);
    global_volume.volume = Volume::Linear(linear);
}

fn toggle_enemy_nameplates(_: On<Pointer<Click>>, mut settings: ResMut<GameSettings>) {
    settings.enemy_nameplates = !settings.enemy_nameplates;
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct GlobalVolumeLabel;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct EnemyNameplatesLabel;

fn update_global_volume_label(
    global_volume: Res<GlobalVolume>,
    mut label: Single<&mut Text, With<GlobalVolumeLabel>>,
) {
    let percent = 100.0 * global_volume.volume.to_linear();
    label.0 = format!("{percent:3.0}%");
}

fn update_enemy_nameplates_label(
    settings: Res<GameSettings>,
    mut label: Single<&mut Text, With<EnemyNameplatesLabel>>,
) {
    label.0 = if settings.enemy_nameplates {
        "On".to_string()
    } else {
        "Off".to_string()
    };
}

fn go_back_on_click(
    _: On<Pointer<Click>>,
    screen: Res<State<Screen>>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    next_menu.set(if screen.get() == &Screen::Title {
        Menu::Main
    } else {
        Menu::Pause
    });
}

fn go_back(screen: Res<State<Screen>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(if screen.get() == &Screen::Title {
        Menu::Main
    } else {
        Menu::Pause
    });
}
