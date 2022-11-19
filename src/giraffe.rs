use crate::camera::CameraTarget;
use crate::in_air::*;
use crate::on_floor::*;
use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::prelude::*;

use crate::neck::{NeckTarget, NeckPoints};
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
                    let mut bundle = NeckBundle::new(
                        transform.translation,
                        Vec3 {
                            x: 5.0,
                            y: 5.0,
                            z: 5.0,
                        },
                        meshes.add(Mesh::new(PrimitiveTopology::TriangleStrip)).into(),
                        materials.add(ColorMaterial::from(Color::RED)),
                    );

                    bundle.neckpoints = NeckPoints {
                        points: vec![
                            Vec3::new(0.0, 0.0, 0.0),
                            Vec3::new(100.0, 100.0, 0.0),
                            Vec3::new(50.0, 400.0, 0.0),
                        ],
                        last_point: Vec3::new(200.0, 300.0, 0.0),
                    };

                    commands.spawn(bundle);
                }
                _ => {}
            }
        }
    }
}

fn spawn_giraffe(mut commands: Commands) {
    commands.spawn((GiraffeBundle::default(), CameraTarget, NeckTarget));
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
            //DEBUG
            .register_inspectable::<Giraffe>();
    }
}
