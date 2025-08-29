use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod body;
mod physics;

use crate::body::*;
use crate::physics::PhysicsPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(ShapePlugin);
    app.add_plugins(PhysicsPlugin);
    app.add_systems(Startup, populate);
    //app.add_systems(Update, calc_bend);
    app.run();
}

fn populate(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn(Body::new(1.0, 0.5, (-100.0, 50.0, 0.0), (20.0, 0.0, 0.0), Color::WHITE));
    commands.spawn(Body::new(1.0, 30.0, (0.0, 0.0, 0.0), (0.0, 0.0, 0.0), Color::BLACK));

}