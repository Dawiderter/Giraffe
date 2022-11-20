use crate::camera::CameraTarget;
use crate::camera::MainCamera;
use crate::cursor::CursorWorldPos;
use crate::in_air::*;
use crate::on_floor::*;
use crate::platform::*;
use crate::shooting_head::ShootingHead;
use crate::shooting_head::ShootingHeadBundle;
use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::prelude::*;

use crate::head::*;
use crate::neck::NeckTarget;
use crate::{in_air::InAir, neck::NeckBundle};

const GIRAFFE_GROUP: bevy_rapier2d::rapier::geometry::Group =
    bevy_rapier2d::rapier::geometry::Group::GROUP_1;
const RIGHT_DIRECTION: Vec2 = Vec2 { x: 1.0, y: 0.0 };

#[derive(Component, Inspectable)]
struct Giraffe {
    jump_speed: f32,
    speed: f32,
    pub right_direction: Vec2,
}

#[derive(Bundle)]
struct GiraffeBundle {
    name: Name,
    in_air: InAirBundle,
    sprite_bundle: SpriteBundle,
    giraffe: Giraffe,
    event: ActiveEvents,
    sleep: Sleeping,
}

impl Default for GiraffeBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Giraffe"),
            sprite_bundle: SpriteBundle::default(),
            in_air: InAirBundle::default(),
            giraffe: Giraffe {
                jump_speed: 500.0,
                speed: 300.0,
                right_direction: RIGHT_DIRECTION,
            },
            event: ActiveEvents::COLLISION_EVENTS,
            sleep: Sleeping::disabled(),
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
                    let bundle =
                        NeckBundle::new(Vec2 { x: 5.0, y: 5.0 }, transform.translation.truncate());
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
    giraffe: Query<&Giraffe>,
    mut query: Query<&mut Transform, (With<GiraffeSprite>, Without<Giraffe>)>,
    mut child_query: Query<&mut Transform, (With<Head>, Without<Giraffe>, Without<GiraffeSprite>)>,
    mouse_pos: Res<CursorWorldPos>,
) {
    if let Ok(g) = giraffe.get_single() {
        if let Ok( mut transform) = query.get_single_mut() {
            transform.rotation = Quat::from_rotation_z(g.right_direction.angle_between(RIGHT_DIRECTION));
            // if let Ok(mouse_pos) = mouse_pos.pos {
            //     if let Ok(mut head) = child_query.get_single_mut() {
            //         head.translation.x = mouse_pos.x.abs();
            //         head.translation.y = mouse_pos.y;
            //         head.translation.z = 0.0;
    
            //         head.translation = head.translation.normalize() * 100.0;
            //     }
            //     let looking_left = f32::signum(transform.scale.x);
            //     if looking_left < 0.0 && mouse_pos.x > transform.translation.x
            //         || looking_left > 0.0 && mouse_pos.x < transform.translation.x
            //     {
            //         transform.scale.x = -transform.scale.x;
            //     };
            // }
        }
    }
}

#[derive(Component)]
struct GiraffeSprite;

fn spawn_giraffe(mut commands: Commands) {
    commands
        .spawn((
            GiraffeBundle::default(),
            CameraTarget,
            NeckTarget,
            CollisionGroups::new(Group::from_bits(GIRAFFE_GROUP.bits()).unwrap(), Group::ALL),
        ))
        .with_children(|parent| {
            parent.spawn((SpriteBundle {
                sprite: Sprite {
                    color: Color::YELLOW,
                    custom_size: Some(Vec2 { x: 100., y: 100. }),
                    ..default()
                },
                ..default()
            },GiraffeSprite)).with_children(|parent| {
                parent.spawn(HeadBundle::new());
            });
        });
}

fn giraffe_hit_floor(
    mut giraffe: Query<(Entity, &InAir, &mut Giraffe)>,
    platforms: Query<(&Platform, &Transform)>,
    rapier_context: Res<RapierContext>,
    mut commands: Commands,
) {
    for (e, ia, mut g) in giraffe.iter_mut() {
        if ia.timer.finished() {
            for contact_pair in rapier_context.contacts_with(e) {
                let other_collider = if contact_pair.collider1() == e {
                    contact_pair.collider2()
                } else {
                    contact_pair.collider1()
                };
        
                if platforms.contains(other_collider) {
                    commands.entity(e)
                        .remove::<InAirBundle>()
                        .insert(AddOnFloorBundle {
                            on_which_floor: other_collider,
                        });
                    if contact_pair.manifolds().last().unwrap().points().len() > 0 {
                        let point = 
                        if contact_pair.collider1() == e {
                            contact_pair.manifolds().last().unwrap().points().last().unwrap().local_p1()
                        } else {
                            contact_pair.manifolds().last().unwrap().points().last().unwrap().local_p2()
                        };
    
                        println!("{}", point);
                        g.right_direction = point.clamp_length(1.0, 1.0).perp();
                        return;
                    }
                }
            }
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
