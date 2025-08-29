use bevy::prelude::*;

mod body;
mod physics;

use crate::body::*;
use crate::physics::{pixels_to_au, PhysicsPlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(PhysicsPlugin);
    app.add_systems(Startup, (populate, set_scale).chain());
    //app.add_systems(Update, calc_bend);
    app.run();
}

fn populate(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2d);

    commands.spawn(Body::new(1.0, 0.0002, (-0.03, 0.02, 0.0), (0.002, 0.0, 0.0), Color::WHITE, &mut meshes, &mut materials));
    commands.spawn(Body::new(1.0, 0.01, (0.0, 0.0, 0.0), (0.0, 0.0, 0.0), Color::BLACK, &mut meshes, &mut materials));
}

fn set_scale(
  mut projection: Single<&mut Projection, With<Camera>>,
) {
    let Projection::Orthographic(orth) = projection.as_mut() else {
        return;
    };

    orth.scale = pixels_to_au(1.0);
}