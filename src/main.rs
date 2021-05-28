use std::u16;

use bevy::{prelude::*};

struct Position {
    x:u32,
    y:u32,
    z:u16
}

fn main() {
    let window_descriptor = WindowDescriptor {
        title: "rustlike".to_string(),
        ..Default::default()
    };

    App::build()
    .insert_resource(window_descriptor)
    .add_plugins(DefaultPlugins)
    .run();
}
