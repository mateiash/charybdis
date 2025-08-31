use bevy::{math::{ops::{atan2, cos, sin, sqrt}, VectorSpace}, pbr::ViewKeyCache, prelude::*};

use crate::body::*;

const PIXELS_PER_AU : f32 = 10000.0;
pub const LIGHT_SPEED : f32 = 0.002;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app : &mut App) {
        println!("Physics connected.");

        app.add_systems(FixedUpdate, (PhysicsPlugin::compute_path, PhysicsPlugin::move_entities).chain());
    }
}

impl PhysicsPlugin {
    fn compute_path(single: Single<(&Transform, &Volume), With<Mass>>, mut query: Query<(&mut Transform, &mut Velocity, &mut Acceleration), Without<Mass>>, time: Res<Time>){
        let (mtrans, mvol) = single.into_inner();

        let mass_pos = mtrans.translation.clone();
        let r_s = mvol.radius;

        for (mut transform, mut velocity, mut acceleration) in &mut query {
            // conversion to polar shii

            let pol_coords = rect_to_pol(&transform.translation, &mass_pos);

            let r = pol_coords.0;
            let theta = pol_coords.1;

            println!("{}", sqrt(velocity.0.x*velocity.0.x + velocity.0.y*velocity.0.y));

            let pol_vel = rect_to_pol(&velocity.0, &Vec3::ZERO);

            let mut vr = pol_vel.0;
            let mut vtheta = pol_vel.1;

            let pol_acc = rect_to_pol(&acceleration.0, &Vec3::ZERO);

            let mut ar = pol_acc.0;
            let mut atheta = pol_acc.1;

            // actual calculations fr tho

            let f = 1.0 - r_s / r;

            ar += r*vtheta*vtheta - (LIGHT_SPEED*LIGHT_SPEED * r_s) / (2.0 * r * r)
                //-(r_s / (2.0 * r * r)) * ( (vr * vr) / f + f * r * r * vtheta * vtheta )
            ;
            atheta += -2.0 * vr * vtheta / r
                //-2.0 * vr * vtheta / r
                ;
            println!("ar {}", ar);
            println!("atheta {}", atheta);

            let new_acc = pol_to_rect((ar, atheta), &Vec3::ZERO);

            acceleration.0 = new_acc;
        }
    }

    fn move_entities(mut query: Query<(&mut Transform, &mut Velocity, &mut Acceleration), Without<Mass>>, time: Res<Time>) {
        for (mut transform, mut velocity, mut acceleration) in &mut query {

            //println!("{}", acceleration.0);
            //println!("{}", velocity.0);
            
            velocity.0.x += acceleration.0.x * time.delta_secs();
            velocity.0.y += acceleration.0.y * time.delta_secs();
            velocity.0.z += acceleration.0.z * time.delta_secs();
            
            transform.translation.x += velocity.0.x * time.delta_secs();
            transform.translation.y += velocity.0.y * time.delta_secs();
            transform.translation.z += velocity.0.z * time.delta_secs();
        }
    }
}

pub fn pixels_to_au(val : f32) -> f32 {
    return val / PIXELS_PER_AU;
} 

pub fn au_to_pixels(val : f32) -> f32 {
    return val * PIXELS_PER_AU;
}

pub fn rect_to_pol(coord : &Vec3, reference : &Vec3) -> (f32, f32){
    let mut x = coord.x - reference.x;
    let mut y = coord.y - reference.y;

    let mut r = sqrt(x*x + y*y);
    let mut theta = atan2(y, x);

    (r, theta)
}

pub fn pol_to_rect(coord : (f32, f32), reference : &Vec3) -> Vec3 {
    let mut x = coord.0 * cos(coord.1);
    let mut y = coord.0 * sin(coord.1);

    let res = Vec3::new(x, y, 0.0);

    return res + reference;
}