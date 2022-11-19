use arena::ArenaPlugin;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod arena;

const WINDOW_HEIGHT : f32 = 1000.;
const WINDOW_WIDTH_PER_HEIGHT : f32 = 3./4.;

const PIXELS_PER_METER : f32 = 100.;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WINDOW_HEIGHT * WINDOW_WIDTH_PER_HEIGHT,
                height: WINDOW_HEIGHT,
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METER))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_camera)
        .add_plugin(ArenaPlugin)
        .run();
}
