use bevy::prelude::*;
use bevy_editor_pls::prelude::*;

mod giraffe;
mod in_air;

use crate::giraffe::*;
use crate::in_air::*;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1600.0,
                height: 900.0,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup_camera)
        .add_plugin(EditorPlugin)
        .add_plugin(GiraffePlugin)
        .add_plugin(InAirPlugin)
        .run();

    println!("Giraffe ; D");
}
