use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

use crate::game::config as cfg;
use crate::game::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Startup, set_camera_scale_after_spawn.after(setup))
            .add_systems(
                Update,
                (zoom, sync_camera_position).in_set(crate::PausableSystems),
            );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera2d, Msaa::Off));
}

fn set_camera_scale_after_spawn(mut query: Query<&mut Projection, With<Camera>>) {
    for mut projection in &mut query {
        if let Projection::Orthographic(ortho) = &mut *projection {
            ortho.scale = cfg::ORTHO_MIN_SCALE;
            break;
        }
    }
}

fn zoom(
    mut mouse_wheel_events: MessageReader<MouseWheel>,
    mut query: Query<&mut Projection, With<Camera>>,
) {
    for event in mouse_wheel_events.read() {
        for mut projection in &mut query {
            let Projection::Orthographic(ortho) = &mut *projection else {
                continue;
            };

            if event.y < 0.0 {
                ortho.scale = (ortho.scale + 0.1).clamp(cfg::ORTHO_MIN_SCALE, cfg::ORTHO_MAX_SCALE);
            } else if event.y > 0.0 {
                ortho.scale = (ortho.scale - 0.1).clamp(cfg::ORTHO_MIN_SCALE, cfg::ORTHO_MAX_SCALE);
            }
        }
    }
}

fn sync_camera_position(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let (Ok(player_transform), Ok(mut camera_transform)) = (player.single(), camera.single_mut())
    {
        camera_transform.translation = player_transform.translation;
    }
}
