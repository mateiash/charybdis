use bevy::{math::VectorSpace, prelude::*};

use crate::physics::{pixels_to_au, au_to_pixels};

#[derive(Bundle)]
pub struct MassBody {
    mass : Mass,
    volume : Volume,
    velocity : Velocity,
    acceleration : Acceleration,
    transform : Transform,

    mesh : Mesh2d,
    mesh_material : MeshMaterial2d<ColorMaterial>,
}

impl MassBody {
    pub fn new(mass : f32, radius : f32, pos : (f32, f32, f32), vel : (f32, f32, f32), color : Color,
                meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>
    ) -> Self{
        let circle = meshes.add(Circle::new(radius));

        Self {
            mass : Mass(mass),
            volume : Volume { radius : radius },
            velocity : Velocity(vec3(vel.0, vel.1, vel.2)),
            acceleration : Acceleration(Vec3::ZERO),
            transform : Transform::from_xyz( pos.0, pos.1, pos.2 ),

            mesh : Mesh2d(circle),
            mesh_material : MeshMaterial2d(materials.add(color)),
        }
    }
}

#[derive(Bundle)]
pub struct Body {
    volume : Volume,
    velocity : Velocity,
    acceleration : Acceleration,
    transform : Transform,

    mesh : Mesh2d,
    mesh_material : MeshMaterial2d<ColorMaterial>,
}

impl Body {
    pub fn new(radius : f32, pos : (f32, f32, f32), vel : (f32, f32, f32), color : Color,
                meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>
    ) -> Self{
        let circle = meshes.add(Circle::new(radius));

        Self {
            volume : Volume { radius : radius },
            velocity : Velocity(vec3(vel.0, vel.1, vel.2)),
            acceleration : Acceleration(Vec3::ZERO),
            transform : Transform::from_xyz( pos.0, pos.1, pos.2 ),

            mesh : Mesh2d(circle),
            mesh_material : MeshMaterial2d(materials.add(color)),
        }
    }
}

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Acceleration(pub Vec3);

/// Measured in solar masses
#[derive(Component)]
pub struct Mass(pub f32);

#[derive(Component)]
pub struct Volume {
    /// In astronomical units
    pub radius : f32,
}