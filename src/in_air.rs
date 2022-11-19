use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::prelude::*;

#[derive(Component, Inspectable)]
pub struct  InAir {
    pub velocity: Vec2, 
}

impl Default for InAir {
    fn default() -> Self {
        Self {
            velocity: Vec2 { x: 800.0, y: 0.0 },
        }
    }
}

fn in_air_movement( mut query: Query<(&mut KinematicCharacterController, &InAir)>, 
                    time: Res<Time>) {
    for (mut kcc, ia) in query.iter_mut() {
        kcc.translation = Some(ia.velocity * time.delta_seconds());
    }
}

pub struct InAirPlugin;

impl Plugin for InAirPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(in_air_movement)

            //DEBUG

            .register_inspectable::<InAir>();
    }
}
