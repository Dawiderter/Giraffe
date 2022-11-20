use bevy::prelude::*;
use bevy_rapier2d::prelude::KinematicCharacterController;

use crate::neck::NeckPoints;

#[derive(Component)]
pub struct AngularVelocity {
    pub radius: f32,
    pub speed: f32,
    pub point: Vec2,
}

pub struct AngularPlugin;

pub fn angular_velocity_system(mut query: Query<(&mut Transform, &mut KinematicCharacterController,
 &AngularVelocity)>) {
    for (mut transform, mut kcc, angular) in query.iter_mut() {
        let perp = transform.translation - angular.point.extend(0.0);
        let perp = Vec2 {
            x: -perp.y,
            y: perp.x,
        }
        .normalize()
            * angular.speed;
        kcc.translation = Some(perp);
    }
}

pub fn set_angular_point_system(
    mut query: Query<(Entity, &Transform, &mut AngularVelocity)>,
    mut neck_query: Query<&mut NeckPoints>,
    mut commands: Commands,
) {
    for (entity, transform, mut angular) in query.iter_mut() {
        if let Ok(neck) = neck_query.get_single_mut() {
            angular.point = *neck.points.last().unwrap();
            angular.radius = transform.translation.distance(angular.point.extend(0.0));
        } else {
            commands
                .get_entity(entity)
                .unwrap()
                .remove::<AngularVelocity>();
        }
    }
}

impl Plugin for AngularPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(angular_velocity_system)
            .add_system(set_angular_point_system);
    }
}
