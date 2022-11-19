use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::prelude::*;

#[derive(Component, Inspectable)]
pub struct  OnFloor;

#[derive(Bundle)]
pub struct OnFloorBundle {
    on_floor: OnFloor,
    character_controller: KinematicCharacterController,
    colider: Collider,
}

impl Default for OnFloorBundle {
    fn default() -> Self {
        Self {
            on_floor: OnFloor,
            character_controller: KinematicCharacterController::default(),
            colider: Collider::ball(100.0),
        }
    }
}

#[derive(Component)]
pub struct AddOnFloorBundle;

fn add_on_floor_bundle (query: Query<Entity, With<AddOnFloorBundle>>, mut commands: Commands) {
    for e in query.iter() {
        commands.entity(e).insert(OnFloorBundle::default())
        .remove::<AddOnFloorBundle>();
    }
}

pub struct OnFloorPlugin;

impl Plugin for OnFloorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(add_on_floor_bundle)
            //DEBUG

            .register_inspectable::<OnFloor>();
    }
}
