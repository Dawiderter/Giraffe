use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::prelude::*;

#[derive(Component, Inspectable)]
pub struct  InAir;

#[derive(Bundle)]
pub struct InAirBundle {
    pub in_air: InAir,
    pub righitbody: RigidBody,
    pub impulse: ExternalImpulse,
    pub colider: Collider,
}

impl Default for InAirBundle {
    fn default() -> Self {
        Self {
            in_air: InAir,
            righitbody: RigidBody::Dynamic,
            impulse: ExternalImpulse {
                impulse: Vec2 { x: 0.0, y: 0.0 }, 
                ..default()
                },
            colider: Collider::ball(100.0),
        }
    }
}

#[derive(Component)]
pub struct AddInAirBundle {
    pub impulse: Vec2,
}

fn add_in_air_bundle (query: Query<(Entity, &AddInAirBundle)>, mut commands: Commands) {
    for (e, a) in query.iter() {
        commands.entity(e).insert(InAirBundle {
            impulse: ExternalImpulse{impulse: a.impulse, ..default()},
            ..default()
        })
        .remove::<AddInAirBundle>();
    }
}

pub struct InAirPlugin;

impl Plugin for InAirPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(add_in_air_bundle)
            //DEBUG

            .register_inspectable::<InAir>();
    }
}
