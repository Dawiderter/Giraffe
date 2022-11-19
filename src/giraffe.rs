use crate::camera::CameraTarget;
use crate::camera::MainCamera;
use crate::in_air::*;
use crate::on_floor::*;
use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::prelude::*;

use crate::head::*;
use crate::neck::{NeckPoints, NeckTarget};
use crate::{in_air::InAir, neck::NeckBundle};

#[derive(Component, Inspectable)]
struct Giraffe {
    jump_speed: f32,
    speed: f32,
    right_direction: Vec2,
}

#[derive(Bundle)]
struct GiraffeBundle {
    name: Name,
    on_floor: OnFloorBundle,
    giraffe: Giraffe,
    sprite: SpriteBundle,
}

impl Default for GiraffeBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Giraffe"),
            on_floor: OnFloorBundle::default(),
            giraffe: Giraffe {
                jump_speed: 500.0,
                speed: 300.0,
                right_direction: Vec2 { x: 1.0, y: 0.0 },
            },
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::YELLOW,
                    custom_size: Some(Vec2 { x: 100., y: 100. }),
                    ..default()
                },
                ..default()
            },
        }
    }
}

fn giraffe_movement(
    mut query: Query<
        (
            Entity,
            &Giraffe,
            &mut KinematicCharacterController,
            &Transform,
        ),
        With<OnFloor>,
    >,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    for (e, g, mut kcc, transform) in query.iter_mut() {
        for k in keys.get_pressed() {
            match k {
                KeyCode::W => {
                    commands
                        .entity(e)
                        .remove::<OnFloorBundle>()
                        .insert(AddInAirBundle {
                            impulse: g.right_direction.perp() * g.jump_speed,
                        });
                }
                KeyCode::A => {
                    kcc.translation = Some(-g.right_direction * g.speed * time.delta_seconds());
                }
                KeyCode::D => {
                    kcc.translation = Some(g.right_direction * g.speed * time.delta_seconds());
                }
                KeyCode::Space => {
                    let bundle = NeckBundle::new(
                        Vec3 {
                            x: 5.0,
                            y: 5.0,
                            z: 5.0,
                        },
                        transform.translation,
                    );
                    commands.spawn(bundle);
                }
                _ => {}
            }
        }
    }
}

fn giraffe_turn_system(
    windows: Res<Windows>,
    mut query: Query<(&mut Transform, &mut Collider), With<Giraffe>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
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
            if let Ok((mut transform, mut collider)) = query.get_single_mut() {
                if let Some(mouse_pos) = mouse_pos {
                    let looking_left = f32::signum(transform.scale.x);
                    if looking_left < 0.0 && mouse_pos.x > transform.translation.x
                        || looking_left > 0.0 && mouse_pos.x < transform.translation.x
                    {
                        transform.scale.x = -transform.scale.x;
                        collider.set_scale(Vec2 { x: 1.0, y: 1.0 }, 100);
                    };
                }
            }
        }
    }
}

fn spawn_giraffe(mut commands: Commands) {
    commands
        .spawn((GiraffeBundle::default(), CameraTarget, NeckTarget))
        .with_children(|parent| {
            parent.spawn(HeadBundle::new());
        });
}

fn giraffe_hit_floor(
    mut query: Query<(Entity, &InAir, &mut Giraffe)>,
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    for (e, ai, mut g) in query.iter_mut() {
        for k in keys.get_pressed() {
            // TODO
        }
    }
}
pub struct GiraffePlugin;

impl Plugin for GiraffePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_giraffe)
            .add_system(giraffe_movement)
            .add_system(giraffe_hit_floor)
            .add_system(giraffe_turn_system)
            //DEBUG
            .register_inspectable::<Giraffe>();
    }
}
