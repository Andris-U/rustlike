use bevy::{prelude::*};

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