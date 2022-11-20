use bevy::{prelude::*, render::render_resource::PrimitiveTopology, sprite::Mesh2dHandle};
use bevy_rapier2d::{prelude::*, rapier::prelude::Group};

use crate::platform::PLATFORM_GROUP;

const NECK_GROUP : Group = Group::GROUP_30;

const NECK_WIDTH: f32 = 25.0;

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
) {
    for entity in query.iter() {
        commands
            .get_entity(entity)
            .unwrap()
            .insert(ColorMesh2dBundle {
                mesh: meshes
                    .add(Mesh::new(PrimitiveTopology::TriangleStrip))
                    .into(),
                material: materials.add(ColorMaterial::from(Color::YELLOW)),
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

fn neck_bend_system(mut neck_query: Query<&mut NeckPoints>, rapier_ctx: Res<RapierContext>) {
    for mut neck in neck_query.iter_mut() {
        let ray_start = neck.points[neck.points.len() - 1];
        let ray_end = neck.last_point - ray_start;
        let ray_dir = ray_end.normalize();
        let max_toi = ray_end.x / ray_dir.x;

        let ray_pos = ray_start + ray_dir;

        if let Some((entity, toi)) = rapier_ctx.cast_ray(
            ray_pos,
            ray_dir,
            max_toi,
            false,
            QueryFilter::new().groups(InteractionGroups::none().with_memberships(NECK_GROUP).with_filter(PLATFORM_GROUP)),
        ) {
            let hit_point = ray_start + ray_dir * toi;
            println!("Entity {:?} hit at point {}", entity, hit_point);
            neck.add_point(hit_point);
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

        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            splits
                .iter()
                .flat_map(|(v1, v2)| [*v1, *v2])
                .map(|v| [v.x, v.y, 0.0])
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
            .add_system(neck_mouse)
            .add_system(update_collision)
            .add_system(neck_bend_system);
    }
}
