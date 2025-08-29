use bevy::prelude::*;

use crate::physics::{pixels_to_au, au_to_pixels};

#[derive(Bundle)]
pub struct Body {
    mass : Mass,
    volume : Volume,
    velocity : Velocity,
    transform : Transform,

    mesh : Mesh2d,
    mesh_material : MeshMaterial2d<ColorMaterial>,
}

impl Body {
    pub fn new(mass : f32, radius : f32, pos : (f32, f32, f32), vel : (f32, f32, f32), color : Color,
                meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>
    ) -> Self{
        let circle = meshes.add(Circle::new(radius));

        Self {
            mass : Mass(mass),
            volume : Volume { radius : radius },
            velocity : Velocity { vx: vel.0, vy: vel.1, vz: vel.2 },
            transform : Transform::from_xyz( pos.0, pos.1, pos.2 ),

            mesh : Mesh2d(circle),
            mesh_material : MeshMaterial2d(materials.add(color)),
        }
    }
}

#[derive(Component)]
pub struct Velocity {
    pub vx : f32,
    pub vy : f32,
    pub vz : f32,
}

/// Measured in solar masses
#[derive(Component)]
pub struct Mass(pub f32);

#[derive(Component)]
pub struct Volume {
    /// In astronomical units
    radius : f32,
}