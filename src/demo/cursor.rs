use bevy::{prelude::*, window::PrimaryWindow};

use crate::{AppSystems, PausableSystems};

#[derive(Resource)]
pub struct CursorPosition(pub Option<Vec2>);

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(CursorPosition(None));
    app.add_systems(
        Update,
        update_cursor_position
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

fn update_cursor_position(
    mut cursor_pos: ResMut<CursorPosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    if window_query.is_empty() || camera_query.is_empty() {
        cursor_pos.0 = None;
    }

    let Ok((camera, camera_transform)) = camera_query.single() else {
        cursor_pos.0 = None;
        return;
    };

    let Ok(window) = window_query.single() else {
        cursor_pos.0 = None;
        return;
    };

    cursor_pos.0 = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate());
}
