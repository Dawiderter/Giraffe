use crate::camera::CameraTarget;
use crate::in_air::*;
use crate::neck::NeckTarget;
use crate::on_floor::*;
use crate::platform::*;
use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::prelude::*;

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
    giraffe: Giraffe,
    sprite: SpriteBundle,
    event: ActiveEvents,
    sleep: Sleeping,
}

impl Default for GiraffeBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Giraffe"),
            in_air: InAirBundle::default(),
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
        ),
        With<OnFloor>,
    >,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    for (e, g, mut kcc) in query.iter_mut() {
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

fn spawn_giraffe(mut commands: Commands) {
    commands.spawn((GiraffeBundle::default(), CameraTarget, NeckTarget));
}

fn giraffe_hit_floor(
    mut giraffe: Query<(Entity, &InAir, &Transform, &mut Giraffe)>,
    platforms: Query<(&Platform, &Transform)>,
    rapier_context: Res<RapierContext>,
    mut commands: Commands,
) {
    for (e, ia, t, mut g) in giraffe.iter_mut() {
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
                    let point = 
                    if contact_pair.collider1() == e {
                        contact_pair.manifolds().last().unwrap().points().last().unwrap().local_p1()
                    } else {
                        contact_pair.manifolds().last().unwrap().points().last().unwrap().local_p2()
                    };

                    println!("{}", point);
                    g.right_direction = point.perp();
                    return;
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
            //DEBUG
            .register_inspectable::<Giraffe>();
    }
}
