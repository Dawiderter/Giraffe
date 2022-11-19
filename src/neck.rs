use crate::arena::Ball;
use bevy::{prelude::*, sprite::Mesh2dHandle, render::render_resource::PrimitiveTopology};

const NECK_WIDTH: f32 = 25.0;

pub struct NeckPlugin;

#[derive(Component)]
struct Neck;

#[derive(Bundle)]
pub struct NeckBundle {
    neck: Neck,
    pub neckpoints: NeckPoints,
}

impl NeckBundle {
    pub fn new(head_point: Vec3, body_point: Vec3) -> Self {
        Self {
            neck: Neck,
            neckpoints: NeckPoints {
                points: vec![head_point],
                last_point: body_point,
            },
        }
    }
}

fn add_mesh(mut commands: Commands, mut query : Query<Entity, Added<Neck>>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    for entity in query.iter() {
        commands.get_entity(entity).unwrap().insert(ColorMesh2dBundle {
            mesh: meshes.add(Mesh::new(PrimitiveTopology::TriangleStrip)).into(),
            material: materials.add(ColorMaterial::from(Color::YELLOW)),
            ..default()
        });
    }
}

#[derive(Component)]
pub struct NeckTarget;

#[derive(Component)]
pub struct NeckPoints {
    pub points: Vec<Vec3>,
    pub last_point: Vec3,
}

impl NeckPoints {
    pub fn add_point(&mut self ,point: Vec3) {
        self.points.push(point);
    }
    
    fn perp(v_a: Vec3, v_b: Vec3) -> Vec3 {
        let diff_vector = v_b - v_a;
        let perp: Vec3 = (-diff_vector.y, diff_vector.x, 0.).into();
        perp.normalize_or_zero()
    }

    fn split(&self, thickness: f32) -> Vec<(Vec3, Vec3)> {
        let mut res = Vec::new();
        if self.points.is_empty() {
            return res;
        }

        let mut points = self.points.clone();
        points.push(self.last_point);

        let first_point = points[0];
        let second_point = points[1];

        let perp_vector = Self::perp(points[0], points[1]);

        let first_split = (
            first_point + perp_vector * thickness,
            first_point - perp_vector * thickness,
        );

        res.push(first_split);

        res.extend(points.windows(3).map(|v| {
            let perp1 = Self::perp(v[0], v[1]);

            let perp2 = Self::perp(v[1], v[2]);

            let bisection = ((perp1 + perp2) / 2.).normalize_or_zero();

            (v[1] + bisection * thickness, v[1] - bisection * thickness)
        }));

        let second_to_last = points[points.len() - 2];
        let last = points[points.len() - 1];

        let diff_vector = (last - second_to_last).normalize();
        let perp_vector: Vec3 = (-diff_vector.y, diff_vector.x, 0.).into();

        let last_split = (
            last + perp_vector * thickness,
            last - perp_vector * thickness,
        );

        res.push(last_split);

        res
    }
}

#[derive(Component)]
pub struct NeckBendingPoints {
    pub points: Vec<Vec3>,
}

impl NeckBendingPoints {
    pub fn from_rectangle(hxhy: Vec2) -> Self {
        NeckBendingPoints {
            points: vec![
                Vec2 {
                    x: -hxhy.x / 2.0,
                    y: hxhy.y / 2.0,
                }
                .extend(1.0),
                Vec2 {
                    x: hxhy.x / 2.0,
                    y: hxhy.y / 2.0,
                }
                .extend(1.0),
                Vec2 {
                    x: hxhy.x / 2.0,
                    y: -hxhy.y / 2.0,
                }
                .extend(1.0),
                Vec2 {
                    x: -hxhy.x / 2.0,
                    y: -hxhy.y / 2.0,
                }
                .extend(1.0),
            ],
        }
    }
}

fn neck_bend_system(mut query: Query<&mut NeckPoints>) {}

fn neck_draw_system() {}

fn neck_mouse(mut neck_query: Query<&mut NeckPoints>, windows: Res<Windows>, input : Res<Input<MouseButton>>) {

    let window = windows.get_primary().unwrap();

    let cursor = window.cursor_position().unwrap_or(Vec2::new(0., 0.));

    let cursor = cursor - Vec2::new(window.width(), window.height())/2.;

    for mut neck in neck_query.iter_mut() {
        neck.last_point = cursor.extend(0.);
        if input.just_pressed(MouseButton::Left) {
            neck.add_point(cursor.extend(0.));
        }
    }

}

fn neck_triangulate(
    mut query: Query<(&mut Mesh2dHandle, &NeckPoints), With<Neck>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (mesh, points) in query.iter_mut() {
        let handle = mesh.0.clone();
        let mesh = meshes.get_mut(&handle).unwrap();

        let splits = points.split(NECK_WIDTH);

        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            splits
                .iter()
                .flat_map(|(v1, v2)| [*v1, *v2])
                .map(|v| [v.x, v.y, v.z])
                .collect::<Vec<[f32; 3]>>(),
        );
    }
}

impl Plugin for NeckPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_system(neck_system)
            .add_system(neck_triangulate)
            .add_system(add_mesh)
            .add_system(neck_mouse);
    }
}
