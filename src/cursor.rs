use bevy::prelude::*;

use crate::camera::MainCamera;

pub struct CursorWorldPosPlugin;

#[derive(Resource)]
pub struct CursorWorldPos {
    pub pos: Result<Vec3, Vec3>,
}

fn update_cursor_pos(
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    windows: Res<Windows>,
    mut cursor_ndc: ResMut<CursorWorldPos>,
) {
    if let Ok((camera, glob_transform)) = camera_query.get_single() {
        let window = windows.get_primary().unwrap();
        if let Some(cursor) = window.cursor_position() {
            let window_size = Vec2 {
                x: window.width(),
                y: window.height(),
            };
            let ndc = (cursor / window_size) * 2.0 - Vec2::ONE;
            let mouse_pos = camera.ndc_to_world(glob_transform, ndc.extend(0.0));
            if let Some(mouse_pos) = mouse_pos {
                cursor_ndc.pos = Ok(mouse_pos);
            }
        }
    }
}

impl Plugin for CursorWorldPosPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorWorldPos {
            pos: Ok(Vec3::ZERO),
        })
        .add_system(update_cursor_pos);
    }
}
