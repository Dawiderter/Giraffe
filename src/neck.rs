use bevy::{prelude::*, render::render_resource::PrimitiveTopology, sprite::Mesh2dHandle};
use bevy_rapier2d::{prelude::*, rapier::prelude::Group};

use crate::platform::PLATFORM_GROUP;

pub const NECK_GROUP: Group = Group::GROUP_30;

const NECK_WIDTH: f32 = 15.0;

pub struct NeckPlugin;

#[derive(Component)]
pub struct Neck;

#[derive(Bundle)]
pub struct NeckBundle {
    neck: Neck,
    pub neckpoints: NeckPoints,
    // pub collider: Collider,
    active_events: ActiveEvents,
}

impl NeckBundle {
    pub fn new(head_point: Vec2, body_point: Vec2) -> Self {
        Self {
            neck: Neck,
            neckpoints: NeckPoints {
                points: vec![head_point],
                last_point: body_point,
            },
            // collider: default(),
            active_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}

fn add_mesh(
    mut commands: Commands,
    query: Query<Entity, Added<Neck>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    server: Res<AssetServer>
) {
    let new_material = ColorMaterial {
        texture : Some(server.load("zebra rozebrana/zyr_szyja_crop.png")),
        ..default()
    };

    let new_material = materials.add(new_material);

    for entity in query.iter() {
        commands
            .get_entity(entity)
            .unwrap()
            .insert(ColorMesh2dBundle {
                mesh: meshes
                    .add(Mesh::new(PrimitiveTopology::TriangleStrip))
                    .into(),
                material: new_material.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                ..default()
            });
    }
}

fn update_collision(mut query: Query<(&mut Collider, &NeckPoints)>) {
    for (mut coll, neck) in query.iter_mut() {
        let last_point = neck.last_point;
        let second_to_last = neck.points[neck.points.len() - 1];

        *coll = Collider::polyline(vec![last_point, second_to_last], None);
    }
}

#[derive(Component)]
pub struct NeckTarget;

#[derive(Component)]
pub struct NeckPoints {
    pub points: Vec<Vec2>,
    pub last_point: Vec2,
}

impl NeckPoints {
    pub fn add_point(&mut self, point: Vec2) {
        self.points.push(point);
    }

    fn perp(v_a: Vec2, v_b: Vec2) -> Vec2 {
        let diff_vector = v_b - v_a;
        let perp: Vec2 = (-diff_vector.y, diff_vector.x).into();
        perp.normalize_or_zero()
    }

    fn split(&self, thickness: f32) -> Vec<(Vec2, Vec2)> {
        let mut res = Vec::new();
        if self.points.is_empty() {
            return res;
        }

        let mut points = self.points.clone();
        points.push(self.last_point);

        let perp_vector = Self::perp(points[0], points[1]);

        let first_point = points[0];

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
        let perp_vector: Vec2 = (-diff_vector.y, diff_vector.x).into();

        let last_split = (
            last + perp_vector * thickness,
            last - perp_vector * thickness,
        );

        res.push(last_split);

        res
    }

    fn gen_uv(&self) -> Vec<(Vec2, Vec2)> {
        let mut res = Vec::new();

        let mut points = self.points.clone();
        points.push(self.last_point);

        let lens: Vec<f32> = points
            .windows(2)
            .map(|v| v[0].distance(v[1]))
            .collect();
        let combined_len: f32 = lens.iter().sum();

        res.push((Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0)));
        let mut curr_len = 0.;

        for len in lens.iter() {
            curr_len += len;
            let progress = curr_len / combined_len;
            res.push((Vec2::new(0.0, progress), Vec2::new(1.0, progress)));
        }

        res
    }
}

#[derive(Component)]
pub struct NeckBendingPoints {
    pub points: Vec<Vec2>,
    pub transformed_points: Vec<Vec2>,
}

impl NeckBendingPoints {
    pub fn closest_point(&self, target_point: Vec2) -> Option<Vec2> {
        let mut closest_point = None;
        for point in &self.transformed_points {
            if let Some(close_point) = closest_point {
                if target_point.distance(*point) < target_point.distance(close_point) {
                    closest_point = Some(*point);
                }
            } else {
                closest_point = Some(*point);
            }
        }
        closest_point
    }

    pub fn from_rectangle(hxhy: Vec2) -> Self {
        NeckBendingPoints {
            points: vec![
                Vec2 {
                    x: -hxhy.x / 2.0,
                    y: hxhy.y / 2.0,
                },
                Vec2 {
                    x: hxhy.x / 2.0,
                    y: hxhy.y / 2.0,
                },
                Vec2 {
                    x: hxhy.x / 2.0,
                    y: -hxhy.y / 2.0,
                },
                Vec2 {
                    x: -hxhy.x / 2.0,
                    y: -hxhy.y / 2.0,
                },
            ],
            transformed_points: Vec::new(),
        }
    }
}

fn transform_bending_points(mut query: Query<(&Transform, &mut NeckBendingPoints)>) {
    for (trans, mut points) in query.iter_mut() {
        points.transformed_points = points
            .points
            .iter()
            .map(|point| trans.transform_point(point.extend(0.0)).truncate())
            .collect();
    }
}

fn neck_bend_system(
    mut neck_query: Query<&mut NeckPoints>,
    points_query: Query<&NeckBendingPoints>,
    rapier_ctx: Res<RapierContext>,
) {
    for mut neck in neck_query.iter_mut() {
        let ray_start = neck.points[neck.points.len() - 1];
        let ray_end = neck.last_point - ray_start;
        let ray_dir = ray_end.normalize();
        let max_toi = ray_end.x / ray_dir.x;

        let ray_pos = ray_start + ray_dir * 10.;

        if let Some((entity, toi)) = rapier_ctx.cast_ray(
            ray_pos,
            ray_dir,
            max_toi,
            false,
            QueryFilter::new().groups(
                InteractionGroups::none()
                    .with_memberships(NECK_GROUP)
                    .with_filter(PLATFORM_GROUP),
            ),
        ) {
            let hit_point = ray_start + ray_dir * toi;
            if let Ok(points) = points_query.get(entity) {
                let point = points.closest_point(hit_point).unwrap();
                neck.add_point(point);
            }
        }
    }
}

fn neck_mouse(
    mut neck_query: Query<&mut NeckPoints>,
    windows: Res<Windows>,
    input: Res<Input<MouseButton>>,
) {
    let window = windows.get_primary().unwrap();

    let cursor = window.cursor_position().unwrap_or(Vec2::new(0., 0.));

    let cursor = cursor - Vec2::new(window.width(), window.height()) / 2.;

    for mut neck in neck_query.iter_mut() {
        neck.last_point = cursor;
        if input.just_pressed(MouseButton::Left) {
            neck.add_point(cursor);
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

        // dbg!(splits.len());
        
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            splits
            .iter()
            .flat_map(|(v1, v2)| [*v1, *v2])
            .map(|v| [v.x, v.y, 0.0])
            .collect::<Vec<[f32; 3]>>(),
        );
        
        let uvs = points.gen_uv();
        
        // dbg!(uvs.len());
        
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_UV_0,
            uvs
                .iter()
                .flat_map(|(v1, v2)| [*v1, *v2])
                .map(|v| [v.x, v.y])
                .collect::<Vec<[f32; 2]>>(),
        );
    }
}

impl Plugin for NeckPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_system(neck_system)
            .add_system(neck_triangulate)
            .add_system(add_mesh)
            // .add_system(neck_mouse)
            .add_system(update_collision)
            .add_system(neck_bend_system)
            .add_system(transform_bending_points);
    }
}
