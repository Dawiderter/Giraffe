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

fn neck_system(mut query: Query<&mut Transform, With<Neck>>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let mut transform = query.single_mut();
    if let Some(position) = window.cursor_position() {
        // let position = position.normalize();
        let position = position
            - Vec2 {
                x: window.width() / 2.0,
                y: window.height() / 2.0,
            };
        let radian = f32::atan2(position.y, position.x);
        transform.rotation = Quat::from_rotation_z(radian);
        transform.translation = Vec3 {
            x: position.normalize().x * position.length() / 2.0,
            y: position.normalize().y * position.length() / 2.0,
            z: 0.0,
        };
        transform.scale = Vec3 {
            x: position.length(),
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
