use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use crate::in_air::*;

#[derive(Component, Inspectable)]
struct Giraffe {
    jump_speed: f32,
    speed: f32,
    right_direction: Vec2,
}


#[derive(Bundle)]
struct GiraffeBundle {
    giraffe: Giraffe,
    sprite: SpriteBundle,
}

impl Default for GiraffeBundle {
    fn default() -> Self {
        Self { 
            giraffe: Giraffe {
                jump_speed: 800.0,
                speed: 600.0,
                right_direction: Vec2 { x: 1.0, y: 0.0 },
            }, 
            sprite: SpriteBundle { 
                sprite: Sprite { 
                    color: Color::YELLOW, 
                    custom_size: Some(Vec2{x: 100., y: 100.}), 
                    ..default()
                }, 
                ..default()
            } 
        }
    }
}

fn giraffe_movement(mut query: Query<(Entity, &Giraffe, &mut Transform), Without<InAir>>, 
                    time: Res<Time>, 
                    keys: Res<Input<KeyCode>>, 
                    mut commands: Commands) {
    for (e, g, mut t) in query.iter_mut() {
        for k in keys.get_pressed() {
            match k {
                KeyCode::W => {
                    commands.entity(e).insert(InAir{velocity: g.right_direction.perp() * g.jump_speed});
                }
                KeyCode::A => {
                    t.translation -= g.right_direction.extend(0.0) * g.speed * time.delta_seconds();
                }
                KeyCode::D => {
                    t.translation += g.right_direction.extend(0.0) * g.speed * time.delta_seconds();
                } 
                _ => {},
            } 
        }
    }
}

fn spawn_giraffe(mut commands: Commands) {
    commands.spawn(GiraffeBundle::default());
}

fn giraffe_hit_floor(   mut query: Query<(Entity, &InAir, &mut Giraffe)>, 
                        keys: Res<Input<KeyCode>>, 
                        mut commands: Commands) {
    for (e, ai, mut g) in query.iter_mut() {
        for k in keys.get_pressed() {
            if *k == KeyCode::Space {
                g.right_direction = ai.velocity.clamp_length(1.0, 1.0).perp();
                commands.entity(e).remove::<InAir>();
            }
        }
    }
}
pub struct GiraffePlugin;

impl Plugin for GiraffePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_giraffe)
            .add_system(giraffe_movement)
            .add_system(giraffe_hit_floor)

            //DEBUG

            .register_inspectable::<Giraffe>();
    }
}
