use arena::ArenaPlugin;
use bevy::prelude::*;
use bevy::render::render_resource::Texture;
use bevy_editor_pls::prelude::*;
use bevy_rapier2d::prelude::*;
use circular::angular_velocity_system;
use circular::AngularPlugin;
use cursor::CursorWorldPosPlugin;
use platform_spawner::PlatformSpawnerPlugin;
use shooting_head::ShootingHeadSystem;

mod arena;
mod on_floor;

mod platform_spawner;

mod camera;

// mod parallax;
//
mod cursor;

mod circular;

use crate::giraffe::*;
use crate::in_air::*;
use crate::on_floor::*;
use bevy_kira_audio::prelude::*;
use neck::{NeckBundle, NeckPlugin};
use platform::spawn_platform;

const WINDOW_HEIGHT: f32 = 900.;
const WINDOW_WIDTH_PER_HEIGHT: f32 = 1.;
const PIXELS_PER_METER: f32 = 100.;
use bevy_kira_audio::prelude::*;

mod neck;

mod shooting_head;

mod giraffe;
mod head;
mod in_air;

mod platform;

use crate::camera::CameraPlugin;
use crate::giraffe::*;
use crate::in_air::*;

#[derive(Resource)]
struct ExtraAssets(Vec<HandleUntyped>);

fn load_extra_assets(mut commands: Commands, server: Res<AssetServer>) {
    if let Ok(handles) = server.load_folder("assets") {
        commands.insert_resource(ExtraAssets(handles));
    }
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
        .add_startup_system(load_extra_assets)
        .add_plugin(ArenaPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(NeckPlugin)
        // .add_startup_system(spawn_platform)
        .add_plugin(EditorPlugin)
        .add_plugin(GiraffePlugin)
        .add_plugin(InAirPlugin)
        .add_plugin(OnFloorPlugin)
        .add_plugin(CursorWorldPosPlugin)
        .add_system(ShootingHeadSystem)
        .add_plugin(AngularPlugin)
        .add_plugin(PlatformSpawnerPlugin)
        .run();
}
