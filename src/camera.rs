use bevy::prelude::*;

use crate::arena::WallMoveTarget;

pub fn camera_viewport_to_world() {
    todo!()
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), WallMoveTarget, MainCamera));
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CameraTarget;

fn camera_movement_system(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<CameraTarget>)>,
    target_query: Query<&Transform, With<CameraTarget>>,
) {
    let target_avg_y = target_query.single().translation.y;

    for mut camera_trans in camera_query.iter_mut() {        
        camera_trans.translation.y = target_avg_y;
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .add_system(camera_movement_system);
    }
}
