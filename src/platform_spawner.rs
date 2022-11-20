use bevy::prelude::*;
use rand::Rng;

use crate::{camera::MainCamera, platform::PlatformBundle};

const PLATFORM_SPAWN_DY: f32 = 200.0;

pub struct PlatformSpawnerPlugin;

#[derive(Bundle)]
struct PlatformSpawnerBundle {
    max_height: MaxHeightComponent,
}

impl PlatformSpawnerBundle {
    pub fn new() -> Self {
        PlatformSpawnerBundle {
            max_height: MaxHeightComponent {
                threshold: 2000.0,
                prev_height: -2001.0,
                height: 0.0,
            },
        }
    }
}

#[derive(Component)]
struct MaxHeightComponent {
    threshold: f32,
    prev_height: f32,
    height: f32,
}

fn setup_platform_spawner(mut commands: Commands) {
    commands.spawn(PlatformSpawnerBundle::new());
}

fn generate_platforms(
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut query: Query<&mut MaxHeightComponent>,
    windows: Res<Windows>,
    mut commands: Commands,
) {
    let mut maxheightcomponent;
    if let Ok((_, glob_transform)) = camera_query.get_single() {
        maxheightcomponent = query.single_mut();
        maxheightcomponent.height =
            f32::max(glob_transform.translation().y, maxheightcomponent.height);
        if maxheightcomponent.height - maxheightcomponent.threshold > maxheightcomponent.prev_height
        {
            let window = windows.get_primary();

            let width = window.unwrap().width();

            let mut rng = rand::thread_rng();

            let mut i = PLATFORM_SPAWN_DY;
            while i < maxheightcomponent.threshold {
                let rx: f32 = (rng.gen::<f32>() * width) - width / 2.0;
                // println!("{}, {}", rx, maxheightcomponent.prev_height + i);

                commands.spawn(PlatformBundle::type_one(
                    Vec2 {
                        x: rx,
                        y: maxheightcomponent.height + i,
                    },
                    Vec2 { x: 300.0, y: 100.0 },
                ));

                i += PLATFORM_SPAWN_DY;
            }
            maxheightcomponent.height += maxheightcomponent.threshold;
        }
    }
}

impl Plugin for PlatformSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(generate_platforms)
            .add_startup_system(setup_platform_spawner);
    }
}
