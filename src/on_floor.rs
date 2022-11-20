use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::prelude::*;

#[derive(Component, Inspectable)]
pub struct  OnFloor{
    pub on_which_floor: Entity,
}

#[derive(Bundle)]
pub struct OnFloorBundle {
    on_floor: OnFloor,
    character_controller: KinematicCharacterController,
    colider: Collider,
}

#[derive(Component)]
pub struct AddOnFloorBundle {
    pub on_which_floor: Entity,
}

fn add_on_floor_bundle (query: Query<(Entity, &AddOnFloorBundle)>, mut commands: Commands) {
    for (e, a) in query.iter() {
        commands.entity(e).insert(OnFloorBundle {
            on_floor: OnFloor { 
                on_which_floor:  a.on_which_floor
            },
            character_controller: KinematicCharacterController{
                snap_to_ground: Some(CharacterLength::Absolute(0.0)),
                ..default()
            },
            colider: Collider::ball(100.0),
        })
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
