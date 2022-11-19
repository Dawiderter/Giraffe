use crate::arena::Ball;
use bevy::prelude::*;

const NECK_WIDTH: f32 = 25.0;

pub struct NeckPlugin;

#[derive(Component)]
struct Neck;

#[derive(Bundle)]
pub struct NeckBundle {
    neck: Neck,
    pub sprite_bundle: SpriteBundle,
}

impl Default for NeckBundle {
    fn default() -> Self {
        Self {
            neck: Neck,
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.8, 0.8, 1.0),
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ..default()
            },
        }
    }
}

#[derive(Component)]
pub struct NeckTarget;

#[derive(Component)]
struct NeckPoints {
    points: Vec<Vec3>,
    last_point: Vec3,
}

impl NeckBendingPoints {
    pub fn from_rectangle(hxhy: Vec2) -> Self {
        NeckBendingPoints {
            points: vec![
                Vec2 {
                    x: -hxhy.x / 2.0,
                    y: hxhy.y / 2.0,
                }
                .extend(0.0),
                Vec2 {
                    x: hxhy.x / 2.0,
                    y: hxhy.y / 2.0,
                }
                .extend(0.0),
                Vec2 {
                    x: hxhy.x / 2.0,
                    y: -hxhy.y / 2.0,
                }
                .extend(0.0),
                Vec2 {
                    x: -hxhy.x / 2.0,
                    y: -hxhy.y / 2.0,
                }
                .extend(0.0),
            ],
        }
    }
}

#[derive(Component)]
pub struct NeckBendingPoints {
    pub points: Vec<Vec3>,
}

fn neck_bend_system(mut query: Query<&mut NeckPoints>) {}

fn neck_draw_system() {}

fn neck_system(
    mut query: Query<&mut Transform, With<Neck>>,
    windows: Res<Windows>,
    target_query: Query<&Transform, (Without<Neck>, With<NeckTarget>)>,
) {
    let window = windows.get_primary().unwrap();
    let mut transform = query.single_mut();
    if let Some(cursor) = window.cursor_position() {
        let ball = ball_query.single();
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

impl Plugin for NeckPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(neck_system);
    }
}
