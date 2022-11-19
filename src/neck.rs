use crate::arena::Ball;
use bevy::{prelude::*, sprite::Mesh2dHandle};

const NECK_WIDTH: f32 = 25.0;

pub struct NeckPlugin;

#[derive(Component)]
struct Neck;

#[derive(Bundle)]
pub struct NeckBundle {
    neck: Neck,
    pub mesh_bundle: ColorMesh2dBundle,
    pub neckpoints: NeckPoints,
}

impl NeckBundle {
    pub fn new(
        head_point: Vec3,
        body_point: Vec3,
        mesh: Mesh2dHandle,
        material: Handle<ColorMaterial>,
    ) -> Self {
        Self {
            neck: Neck,
            mesh_bundle: ColorMesh2dBundle {
                mesh,
                material,
                ..default()
            },
            neckpoints: NeckPoints {
                points: vec![head_point],
                last_point: body_point,
            },
        }
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
    fn perp(v_a : Vec3, v_b : Vec3) -> Vec3 {
        let diff_vector = v_b - v_a;
        let perp : Vec3 = (-diff_vector.y, diff_vector.x, 0.).into();
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
        
        let second_to_last = points[points.len()-2];
        let last = points[points.len()-1];
        
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

fn neck_system(
    mut query: Query<&mut Transform, With<Neck>>,
    windows: Res<Windows>,
    target_query: Query<&Transform, (Without<Neck>, With<NeckTarget>)>,
) {
    let window = windows.get_primary().unwrap();
    let transform = query.get_single_mut();
    if let Ok(mut transform) = transform {
        if let Some(cursor) = window.cursor_position() {
            let ball = target_query.single();
            // let position = position.normalize();
            let cursor = cursor
                - Vec2 {
                    x: window.width() / 2.0,
                    y: window.height() / 2.0,
                };
            let radian = f32::atan2(ball.translation.y - cursor.y, ball.translation.x - cursor.x);
            let len = f32::sqrt(
                f32::powi(ball.translation.x - cursor.x, 2)
                    + f32::powi(ball.translation.y - cursor.y, 2),
            );
            let halfway = Vec3 {
                x: (cursor.x + ball.translation.x) / 2.0,
                y: (cursor.y + ball.translation.y) / 2.0,
                z: 0.0,
            };
            transform.rotation = Quat::from_rotation_z(radian);
            transform.translation = Vec3 {
                x: halfway.x,
                y: halfway.y,
                z: 0.0,
            };
            transform.scale = Vec3 {
                x: len,
                y: NECK_WIDTH,
                z: 0.0,
            };
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

        let splits = points.split(10.);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, splits.iter().flat_map(|(v1, v2)| [*v1,*v2]).map(|v| [v.x,v.y,v.z]).collect::<Vec<[f32;3]>>());
    }
}

impl Plugin for NeckPlugin {
    fn build(&self, app: &mut App) {
        app
           // .add_system(neck_system)
            .add_system(neck_triangulate);
    }
}
