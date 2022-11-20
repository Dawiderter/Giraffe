use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct ShootingHead(Vec2);

#[derive(Bundle)]
pub struct ShootingHeadBundle {
    head: ShootingHead,
    sprite: SpriteBundle,
    // collider: Collider,
}

pub fn ShootingHeadSystem(mut query: Query<(&mut Transform, &ShootingHead)>) {
    for (mut transform, vel) in query.iter_mut() {
        transform.translation += vel.0.extend(0.0);
    }
}

impl ShootingHeadBundle {
    pub fn new(transform: Transform, velocity: Vec2) -> Self {
        ShootingHeadBundle {
            head: ShootingHead(velocity),
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2 { x: 25., y: 25. }),
                    ..default()
                },
                transform: transform,
                ..default()
            },
        }
    }
}
