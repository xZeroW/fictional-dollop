use bevy::prelude::*;

use crate::{
    components::{AttackCooldown, Enemy, Health},
    config::GameSettings,
};

#[derive(Component)]
pub struct AttackTimeNameplate;

#[derive(Component)]
pub struct HealthNameplate;

pub fn update_attack_time_nameplates(
    settings: Res<GameSettings>,
    enemies: Query<(&AttackCooldown, &Children), With<Enemy>>,
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

pub fn update_health_nameplates(
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

pub fn update_nameplate_visibility(
    settings: Res<GameSettings>,
    mut nameplates: Query<&mut Visibility, Or<(With<AttackTimeNameplate>, With<HealthNameplate>)>>,
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
