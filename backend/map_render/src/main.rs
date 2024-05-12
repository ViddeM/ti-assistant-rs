use bevy::prelude::*;
use tile::setup;

pub mod tile;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, zooming)
        .run();
}
