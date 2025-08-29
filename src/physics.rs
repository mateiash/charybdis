use bevy::prelude::*;

use crate::body::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app : &mut App) {
        println!("Physics connected.");

        app.add_systems(FixedUpdate, PhysicsPlugin::move_entities);
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