use bevy::{color::palettes::css::*, prelude::*};
use bevy_prototype_lyon::prelude::*;

mod body;

use crate::body::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(ShapePlugin);
    app.add_systems(Startup, populate);
    //app.add_systems(Update, calc_bend);
    app.run();
}

fn populate(mut commands: Commands) {
    commands.spawn(Camera2d);

    let body = Body::new(1.0, 100.0);

    commands.spawn(body);

}