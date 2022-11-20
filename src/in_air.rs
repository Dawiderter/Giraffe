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
                    Duration::from_secs_f32(1.0), 
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
        commands.entity(e).insert(InAirBundle {
            impulse: ExternalImpulse{impulse: a.impulse, ..default()},
            ..default()
        })
        .remove::<AddInAirBundle>();
    }
}

fn update_in_air_timer(mut query: Query<&mut InAir>, time: Res<Time>) {
    for mut ia in query.iter_mut() {
        ia.timer.tick(time.delta());
    }
}

pub struct InAirPlugin;

impl Plugin for InAirPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(add_in_air_bundle)
            .add_system(update_in_air_timer);
            //DEBUG
    }
}
