use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct  InAir {
    pub timer: Timer,
}

#[derive(Bundle)]
pub struct InAirBundle {
    pub in_air: InAir,
    pub righitbody: RigidBody,
    pub impulse: ExternalImpulse,
    pub colider: Collider,
    pub gravity_scale: GravityScale,
}

impl Default for InAirBundle {
    fn default() -> Self {
        Self {
            in_air: InAir { 
                timer: Timer::new(
                    Duration::from_secs_f32(0.1), 
                TimerMode::Once)},
            righitbody: RigidBody::Dynamic,
            impulse: ExternalImpulse {
                impulse: Vec2 { x: 0.0, y: -100.0 }, 
                ..default()
                },
            colider: Collider::ball(100.0),
            gravity_scale: GravityScale(0.0),
        }
    }
}

#[derive(Component)]
pub struct AddInAirBundle {
    pub impulse: Vec2,
}

fn add_in_air_bundle (query: Query<(Entity, &AddInAirBundle)>, mut commands: Commands) {
    for (e, a) in query.iter() {
        commands.entity(e).insert((InAirBundle {
            impulse: ExternalImpulse{impulse: a.impulse, ..default()},
            ..default()
        }, Restitution {
                        coefficient: 1.,
                        combine_rule: CoefficientCombineRule::Max,
                    }))
        .remove::<AddInAirBundle>();
    }
}

fn update_in_air_timer(mut query: Query<&mut InAir>, time: Res<Time>) {
    for mut ia in query.iter_mut() {
        ia.timer.tick(time.delta());
    }
}

fn update_translation(removals: RemovedComponents<AddInAirBundle>, mut query: Query<&mut Transform>) {
    for e in removals.iter() {
        if query.contains(e) {
            query.get_mut(e).unwrap().translation += Vec3{x: 1., y: 0.0, z: 0.0};
        }
    }
}

pub struct InAirPlugin;

impl Plugin for InAirPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_to_stage(CoreStage::PreUpdate, add_in_air_bundle)
            .add_system(update_translation)
            .add_system(update_in_air_timer);
            //DEBUG
    }
}
