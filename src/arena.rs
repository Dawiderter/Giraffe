use bevy::{prelude::*, sprite::MaterialMesh2dBundle, utils::FloatOrd};
use bevy_rapier2d::prelude::*;

use crate::PIXELS_PER_METER;

const FLOOR_RISE: f32 = 50.;
const WALL_WIDTH: f32 = 50.;

const ARENA_COLOR: Color = Color::rgb(0.29, 0.0, 0.51);

pub struct ArenaPlugin;

fn setup_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();

    let width = window.width();
    let height = FLOOR_RISE * 2.;

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box::new(width, height, 1.0).into())
                .into(),
            material: materials.add(ColorMaterial::from(ARENA_COLOR)),
            transform: Transform::from_translation(Vec3::new(0., -window.height() / 2., 0.)),
            ..default()
        },
        Collider::cuboid(width / 2., height / 2.),
    ));
}

fn setup_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();

    let width = WALL_WIDTH * 2.;
    let height = window.height();

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box::new(width, height, 1.0).into())
                .into(),
            material: materials.add(ColorMaterial::from(ARENA_COLOR)),
            transform: Transform::from_translation(Vec3::new(-window.width() / 2., 0., 0.)),
            ..default()
        },
        Collider::cuboid(width / 2., height / 2.),
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box::new(width, height, 1.0).into())
                .into(),
            material: materials.add(ColorMaterial::from(ARENA_COLOR)),
            transform: Transform::from_translation(Vec3::new(window.width() / 2., 0., 0.)),
            ..default()
        },
        Collider::cuboid(width / 2., height / 2.),
    ));
}

#[derive(Component)]
struct Ball;

fn test_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let radius = 50.;

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(radius).into()).into(),
            material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        Collider::ball(radius),
        RigidBody::Dynamic,
        Restitution {
            coefficient: 1.,
            combine_rule: CoefficientCombineRule::Max,
        },
        ExternalForce::default(),
        Ball,
    ));
}

fn test_ball_movement(
    mut query: Query<&mut ExternalForce, With<Ball>>,
    input : Res<Input<KeyCode>>,
) {
    for mut ball_force in query.iter_mut() {
        ball_force.force = Vec2::new(0., 0.);
        if input.pressed(KeyCode::Left) {
            ball_force.force = Vec2::new(-100., 0.)
        }
        if input.pressed(KeyCode::Right) {
            ball_force.force = Vec2::new(100., 0.)
        }
    }
}

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_floor)
            .add_startup_system(setup_walls)
            .add_startup_system(test_ball)
            .add_system(test_ball_movement);
    }
}
