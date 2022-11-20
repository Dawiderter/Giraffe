use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Head;

#[derive(Bundle)]
pub struct HeadBundle {
    head: Head,
    sprite: SpriteBundle,
}

impl HeadBundle {
    pub fn new() -> Self {
        HeadBundle {
            head: Head,
            sprite: SpriteBundle {
                transform: Transform::from_translation(Vec3 {
                    x: 100.0,
                    y: 50.0,
                    z: 0.0,
                }),
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2 { x: 25., y: 25. }),
                    ..default()
                },
                ..default()
            },
        }
    }
}

impl Default for HeadBundle {
    fn default() -> Self {
        Self::new()
    }
}
