use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

#[derive(Bundle)]
pub struct Body {
    mass : Mass,
    volume : Volume,
    velocity : Velocity,


    shape : ShapeBundle,
    fill : Fill,
    stroke : Stroke,
}

impl Body {
    pub fn new(mass : f32, radius : f32, pos : (f32, f32, f32), vel : (f32, f32, f32), color : Color) -> Self{
        let circle = shapes::Circle {
            radius: radius,
            center: Vec2::new(pos.0, pos.1),
        };

        Self {
            mass : Mass(mass),
            volume : Volume { radius : radius },
            velocity : Velocity { vx: vel.0, vy: vel.1, vz: vel.2 },

            shape : ShapeBundle {
            path: GeometryBuilder::build_as(&circle),
                ..default()
            },
            fill : Fill::color(color),
            stroke : Stroke::new(color, 2.0),
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