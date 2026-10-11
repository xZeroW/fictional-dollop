use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    components::{Enemy, Health},
    config::GameSettings,
    screens::Screen,
};

#[cfg(all(feature = "dev", debug_assertions))]
const ATTACK_NAMEPLATE_OFFSET_Y: f32 = 42.0;
const HEALTH_NAMEPLATE_OFFSET_Y: f32 = 58.0;
const NAMEPLATE_FONT_SIZE: f32 = 16.0;
#[cfg(all(feature = "dev", debug_assertions))]
const ATTACK_NAMEPLATE_TEXT_COLOR: Color = Color::srgb(1.0, 0.92, 0.68);
const HEALTH_NAMEPLATE_TEXT_COLOR: Color = Color::srgb(0.55, 1.0, 0.55);

pub(super) struct NameplatePlugin;

impl Plugin for NameplatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (attach_nameplates_to_new_enemies, update_health_nameplates)
                .in_set(PausableSystems)
                .in_set(AppSystems::Update)
                .run_if(in_state(Screen::Gameplay)),
        );
        app.add_systems(
            Update,
            update_nameplate_visibility
                .in_set(AppSystems::Update)
                .run_if(in_state(Screen::Gameplay))
                .run_if(resource_changed::<GameSettings>),
        );

        #[cfg(all(feature = "dev", debug_assertions))]
        app.add_systems(
            Update,
            update_attack_time_nameplates
                .in_set(PausableSystems)
                .in_set(AppSystems::Update)
                .run_if(in_state(Screen::Gameplay)),
        );
    }
}

#[derive(Component)]
struct EnemyNameplate;

#[cfg(all(feature = "dev", debug_assertions))]
#[derive(Component)]
struct AttackTimeNameplate;

#[derive(Component)]
struct HealthNameplate;

fn attach_nameplates_to_new_enemies(
    mut commands: Commands,
    settings: Res<GameSettings>,
    enemies: Query<(Entity, &Transform), Added<Enemy>>,
) {
    let nameplate_visibility = if settings.enemy_nameplates {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };

    for (enemy, transform) in &enemies {
        let scale = transform.scale.x.abs().max(f32::EPSILON);

        commands.entity(enemy).with_children(|enemy| {
            enemy.spawn((
                Name::new("Enemy Health Nameplate"),
                EnemyNameplate,
                HealthNameplate,
                Text2d::new(""),
                TextFont::from_font_size(NAMEPLATE_FONT_SIZE),
                TextColor(HEALTH_NAMEPLATE_TEXT_COLOR),
                nameplate_visibility,
                Transform::from_xyz(0.0, HEALTH_NAMEPLATE_OFFSET_Y / scale, 1.0)
                    .with_scale(Vec3::splat(1.0 / scale)),
            ));

            #[cfg(all(feature = "dev", debug_assertions))]
            enemy.spawn((
                Name::new("Enemy Attack Time Nameplate"),
                EnemyNameplate,
                AttackTimeNameplate,
                Text2d::new("READY"),
                TextFont::from_font_size(NAMEPLATE_FONT_SIZE),
                TextColor(ATTACK_NAMEPLATE_TEXT_COLOR),
                nameplate_visibility,
                Transform::from_xyz(0.0, ATTACK_NAMEPLATE_OFFSET_Y / scale, 1.0)
                    .with_scale(Vec3::splat(1.0 / scale)),
            ));
        });
    }
}

#[cfg(all(feature = "dev", debug_assertions))]
fn update_attack_time_nameplates(
    settings: Res<GameSettings>,
    enemies: Query<(&crate::components::AttackCooldown, &Children), With<Enemy>>,
    mut nameplates: Query<&mut Text2d, With<AttackTimeNameplate>>,
) {
    if !settings.enemy_nameplates {
        return;
    }

    for (cooldown, children) in &enemies {
        let remaining =
            (cooldown.timer.duration().as_secs_f32() - cooldown.timer.elapsed_secs()).max(0.0);
        let next_text = if cooldown.timer.is_finished() {
            "READY".to_string()
        } else {
            format!("{remaining:.1}s")
        };

        for child in children.iter() {
            let Ok(mut text) = nameplates.get_mut(child) else {
                continue;
            };

            if text.as_str() != next_text {
                text.0 = next_text.clone();
            }
        }
    }
}

fn update_health_nameplates(
    settings: Res<GameSettings>,
    enemies: Query<(&Health, &Children), With<Enemy>>,
    mut nameplates: Query<&mut Text2d, With<HealthNameplate>>,
) {
    if !settings.enemy_nameplates {
        return;
    }

    for (health, children) in &enemies {
        let current = health.current.max(0.0).ceil();
        let max = health.max.max(0.0).ceil();
        let next_text = format!("{current:.0} / {max:.0}");

        for child in children.iter() {
            let Ok(mut text) = nameplates.get_mut(child) else {
                continue;
            };

            if text.as_str() != next_text {
                text.0 = next_text.clone();
            }
        }
    }
}

fn update_nameplate_visibility(
    settings: Res<GameSettings>,
    mut nameplates: Query<&mut Visibility, With<EnemyNameplate>>,
) {
    let visibility = if settings.enemy_nameplates {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };

    for mut nameplate_visibility in &mut nameplates {
        *nameplate_visibility = visibility;
    }
}
