use bevy::prelude::*;

use crate::body::*;

const PIXELS_PER_AU : f32 = 10000.0;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app : &mut App) {
        println!("Physics connected.");

        app.add_systems(Update, PhysicsPlugin::move_entities);
    }
}

impl PhysicsPlugin {
    fn move_entities(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
        for (mut transform, velocity) in &mut query {
            transform.translation.x += velocity.vx * time.delta_secs();
            transform.translation.y += velocity.vy * time.delta_secs();
            transform.translation.z += velocity.vz * time.delta_secs();
        }
    }
}

pub fn pixels_to_au(val : f32) -> f32 {
    return val / PIXELS_PER_AU;
} 

pub fn au_to_pixels(val : f32) -> f32 {
    return val * PIXELS_PER_AU;
}