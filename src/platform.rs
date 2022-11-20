use crate::neck::NeckBendingPoints;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Platform;

#[derive(Bundle)]
pub struct PlatformBundle {
    platform: Platform,
    sprite: SpriteBundle,
    collider: Collider,
}

impl PlatformBundle {
    pub fn with_start_pos(mut self, pos: Vec2) -> Self {
        self.sprite.transform.translation = pos.extend(0.);
        self
    }

    pub fn type_one(pos: Vec2, size: Vec2) -> Self {
        PlatformBundle {
            platform: Platform,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.8, 0.8, 1.0),
                    custom_size: Some(size),
                    ..default()
                },
                ..default()
            },
            collider: Collider::cuboid(size.x / 2.0, size.y / 2.0),
        }
        .with_start_pos(Vec2 { x: 300.0, y: 0.0 })
    }
}

pub fn spawn_platform(mut commands: Commands) {
    let size = Vec2 { x: 100.0, y: 40.0 };
    commands
        .spawn(
            PlatformBundle::type_one(Vec2 { x: 300.0, y: 0.0 }, size)
                .with_start_pos(Vec2 { x: 0.0, y: -300.0 }),
        )
        .with_children(|parent| {
            parent.spawn(NeckBendingPoints::from_rectangle(size));
        });
        commands
        .spawn(
            PlatformBundle::type_one(Vec2 { x: 300.0, y: 0.0 }, size)
                .with_start_pos(Vec2 { x: 0.0, y: 300.0 }),
        )
        .with_children(|parent| {
            parent.spawn(NeckBendingPoints::from_rectangle(size));
        });
}
