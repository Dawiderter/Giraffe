use std::any::Any;
use std::f32::consts::PI;

use crate::camera::CameraTarget;
use crate::circular::AngularVelocity;
use crate::cursor::CursorWorldPos;
use crate::in_air::*;
use crate::neck::Neck;
use crate::neck::NeckPoints;
use crate::on_floor::*;
use crate::platform::*;
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
pub struct Giraffe {
    jump_speed: f32,
    speed: f32,
    pub right_direction: Vec2,
}

#[derive(Component)]
struct GiraffeNeckStart(Vec2);

#[derive(Component)]
struct PreviousPlatform(Entity);

#[derive(Bundle)]
struct GiraffeBundle {
    name: Name,
    in_air: InAirBundle,
    sprite_bundle: SpriteBundle,
    giraffe: Giraffe,
    event: ActiveEvents,
    sleep: Sleeping,
    neckstart: GiraffeNeckStart,
    locked: LockedAxes,
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
            neckstart: GiraffeNeckStart(Vec2 { x: 80.0, y: 80.0 }),
            locked: LockedAxes::ROTATION_LOCKED_Z,
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
    neck_query: Query<&Neck>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    cursor_pos: Res<CursorWorldPos>,
    mut commands: Commands,
    rapier_ctx: Res<RapierContext>,
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
                KeyCode::Space => {}
                _ => {}
            }
        }
    }
}

fn neck_control_system(
    mut query: Query<(Entity, &Giraffe, &Transform)>,
    head_query: Query<(&Transform, &GlobalTransform), With<Head>>,
    neck_query: Query<&Neck>,
    keys: Res<Input<KeyCode>>,
    cursor_pos: Res<CursorWorldPos>,
    mut commands: Commands,
    rapier_ctx: Res<RapierContext>,
) {
    for (e, g, transform) in query.iter_mut() {
        if keys.just_pressed(KeyCode::F) {
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
                    };

                    let ray_start = head_glob_transform.translation().truncate();
                    let ray_dir = (head_glob_transform.translation() - transform.translation)
                        .normalize()
                        .truncate();
                    let max_toi = 1000.0;

                    let ray_pos = ray_start + ray_dir;

                    if let Some((entity, toi)) = rapier_ctx.cast_ray(
                        ray_pos,
                        ray_dir,
                        max_toi,
                        false,
                        QueryFilter::new().groups(InteractionGroups::all()),
                    ) {
                        let hit_point = ray_start + ray_dir * toi;
                        if neck_query.iter().count() == 0 {
                            commands.spawn(NeckBundle::new(
                                hit_point,
                                transform.translation.truncate(),
                            ));
                            commands.get_entity(e).unwrap().insert(AngularVelocity {
                                radius: hit_point.distance(ray_start),
                                speed: 10.0
                                    * if hit_point.angle_between(Vec2 { x: 1.0, y: 1.0 }).abs()
                                        > PI / 2.0
                                    {
                                        1.0
                                    } else {
                                        -1.0
                                    },
                                point: hit_point,
                            });
                            commands
                                .entity(e)
                                .remove::<OnFloorBundle>()
                                .insert(AddInAirBundle {
                                    impulse: g.right_direction.perp() * g.jump_speed,
                                });
                        }
                    }

                    commands.spawn(ShootingHeadBundle::new(transform_copy, velocity));
                }
            }
        }
    }
}

fn keep_neck_at_player_system(
    mut neck_query: Query<&mut NeckPoints>,
    mut query: Query<(&Transform, &GiraffeNeckStart), With<Giraffe>>,
) {
    for mut neck in neck_query.iter_mut() {
        if let Ok((transform, neckstart)) = query.get_single_mut() {
            neck.last_point = transform
                .transform_point(neckstart.0.extend(0.0))
                .truncate();
        }
    }
}

fn giraffe_turn_system(
    giraffe: Query<(&Giraffe, &Transform)>,
    mut query: Query<(&mut Transform, &mut Sprite), (With<GiraffeSprite>, Without<Giraffe>)>,
    mouse_pos: Res<CursorWorldPos>,
) {
    if let Ok((g, t)) = giraffe.get_single() {
        if let Ok((mut transform, mut sprite)) = query.get_single_mut() {
            println!("{}", RIGHT_DIRECTION.perp().perp().perp());
            println!("{}", g.right_direction);

            if g.right_direction
                .angle_between(RIGHT_DIRECTION.perp().perp().perp())
                > 0.0
            {
                transform.rotation =
                    Quat::from_rotation_z(g.right_direction.angle_between(RIGHT_DIRECTION));
            } else {
                transform.rotation = Quat::from_rotation_z(
                    2.0 * PI - g.right_direction.angle_between(RIGHT_DIRECTION),
                );
            }

            if let Ok(mouse_pos) = mouse_pos.pos {
                if g.right_direction
                    .angle_between(mouse_pos.truncate() - t.translation.truncate())
                    .abs()
                    < PI / 2.0
                {
                    sprite.flip_x = false;
                } else {
                    sprite.flip_x = true;
                }
            }
        }
    }
}

fn head_turn_system(
    mut query: Query<&mut Transform, With<Giraffe>>,
    mut child_query: Query<&mut Transform, (With<Head>, Without<Giraffe>, Without<GiraffeSprite>)>,
    mouse_pos: Res<CursorWorldPos>,
) {
    if let Ok(transform) = query.get_single_mut() {
        if let Ok(mouse_pos) = mouse_pos.pos {
            if let Ok(mut head) = child_query.get_single_mut() {
                head.translation = mouse_pos - transform.translation;

                head.translation = head.translation.normalize() * 100.0;
                head.translation.z = 0.0;
            }
        }
    }
}

#[derive(Component)]
struct GiraffeSprite;

fn spawn_giraffe(mut commands: Commands, handles: Res<AssetServer>) {
    commands
        .spawn((
            GiraffeBundle::default(),
            CameraTarget,
            NeckTarget,
            CollisionGroups::new(Group::from_bits(GIRAFFE_GROUP.bits()).unwrap(), Group::ALL),
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: handles.load_untyped("ż☻yrafa2.png").typed::<Image>(),
                    sprite: Sprite {
                        custom_size: Some(Vec2 { x: 150.0, y: 150.0 }),
                        ..default()
                    },
                    ..default()
                },
                GiraffeSprite,
            ));
        })
        .with_children(|parent| {
            parent.spawn(HeadBundle::new());
        });
}

fn remove_neck_system(
    mut query: Query<(Entity), With<Neck>>,
    mut giraffe_query: Query<Entity, (With<InAir>, With<Giraffe>)>,
    rapier_ctx: Res<RapierContext>,
    mut commands: Commands,
) {
    for e in giraffe_query.iter() {
        if rapier_ctx.contacts_with(e).count() > 0 {
            if let Ok(neck) = query.get_single() {
                commands.entity(neck).despawn();
            }
        }
    }
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
                    if contact_pair.manifolds().last().unwrap().points().len() > 0 {
                        let point = if contact_pair.collider1() == e {
                            contact_pair
                                .manifolds()
                                .last()
                                .unwrap()
                                .points()
                                .last()
                                .unwrap()
                                .local_p1()
                        } else {
                            contact_pair
                                .manifolds()
                                .last()
                                .unwrap()
                                .points()
                                .last()
                                .unwrap()
                                .local_p2()
                        };

                        commands
                            .entity(e)
                            .remove::<InAirBundle>()
                            .insert(AddOnFloorBundle {
                                on_which_floor: other_collider,
                            });
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
            .add_system(head_turn_system)
            .add_system(giraffe_turn_system)
            .add_system(keep_neck_at_player_system)
            .add_system(remove_neck_system)
            .add_system(neck_control_system)
            //DEBUG
            .register_inspectable::<Giraffe>();
    }
}
