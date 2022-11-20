use crate::camera::CameraTarget;
use crate::camera::MainCamera;
use crate::cursor::CursorWorldPos;
use crate::in_air::*;
use crate::on_floor::*;
use crate::shooting_head::ShootingHead;
use crate::shooting_head::ShootingHeadBundle;
use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::prelude::*;

use crate::head::*;
use crate::neck::NeckTarget;
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
    head_query: Query<(&Transform, &GlobalTransform), With<Head>>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    cursor_pos: Res<CursorWorldPos>,
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
                        Vec2 {
                            x: 5.0,
                            y: 5.0,
                        },
                        transform.translation.truncate(),
                    );
                    commands.spawn(bundle);
                }
                KeyCode::F => {
                    if let Ok((head_transform, head_glob_transform)) = head_query.get_single() {
                        if let Ok(cursor_pos) = cursor_pos.pos {
                            let mut transform_copy = *head_transform;
                            let (scale, rotation, translation) =
                                head_glob_transform.to_scale_rotation_translation();
                            transform_copy.translation = translation;
                            transform_copy.rotation = transform_copy.rotation + rotation;
                            transform_copy.scale = scale;
                            let velocity = Vec2 {
                                x: cursor_pos.normalize().x,
                                y: cursor_pos.normalize().y,
                            } * 10.0;

                            commands.spawn(ShootingHeadBundle::new(transform_copy, velocity));
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn giraffe_turn_system(
    mut query: Query<(&mut Transform, &mut Collider), With<Giraffe>>,
    mut child_query: Query<&mut Transform, (With<Head>, Without<Giraffe>)>,
    mouse_pos: Res<CursorWorldPos>,
) {
    if let Ok((mut transform, mut collider)) = query.get_single_mut() {
        if let Ok(mouse_pos) = mouse_pos.pos {
            if let Ok(mut head) = child_query.get_single_mut() {
                head.translation.x = mouse_pos.x.abs();
                head.translation.y = mouse_pos.y;
                head.translation.z = 0.0;

                head.translation = head.translation.normalize() * 100.0;
            }
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
