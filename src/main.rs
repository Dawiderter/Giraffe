use arena::ArenaPlugin;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_editor_pls::prelude::*;

mod arena;

const WINDOW_HEIGHT: f32 = 900.;
const WINDOW_WIDTH_PER_HEIGHT: f32 = 1.;

const PIXELS_PER_METER: f32 = 100.;
use bevy_kira_audio::prelude::*;
use neck::{NeckBundle, NeckPlugin};

mod neck;

mod bendable_platform;

fn spawn_neck(mut commands: Commands) {
    commands.spawn(NeckBundle::default());
}

mod giraffe;
mod in_air;

use crate::giraffe::*;
use crate::in_air::*;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WINDOW_HEIGHT * WINDOW_WIDTH_PER_HEIGHT,
                height: WINDOW_HEIGHT,
                position: WindowPosition::Centered,
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(ArenaPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(NeckPlugin)
        .add_startup_system(spawn_neck)
        .add_startup_system(setup_camera)
        .add_plugin(EditorPlugin)
        .add_plugin(GiraffePlugin)
        .add_plugin(InAirPlugin)
        .run();

    println!("Giraffe ; D");
}
