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
    pub fn new(mass : f32, radius : f32) -> Self{
        let circle = shapes::Circle {
            radius: radius,
            center: Vec2::ZERO,
        };

        Self {
            mass : Mass(mass),
            volume : Volume { radius : radius },
            velocity : Velocity { vx: 0.0, vy: 0.0, vz: 0.0 },

            shape : ShapeBundle {
            path: GeometryBuilder::build_as(&circle),
                ..default()
            },
            fill : Fill::color(Color::BLACK),
            stroke : Stroke::new(Color::BLACK, 2.0),
        }
    }
}

#[derive(Component)]
pub struct Velocity {
    vx : f32,
    vy : f32,
    vz : f32,
}

/// Measured in solar masses
#[derive(Component)]
pub struct Mass(pub f32);

#[derive(Component)]
pub struct Volume {
    /// In astronomical units
    radius : f32,
}