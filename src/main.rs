use std::u16;

use bevy::{prelude::*};

struct Position {
    x:u32,
    y:u32,
    z:u16
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // TODO: Decide on project structure and extract this into sprite loader file.
    let texture_handle = asset_server.load("../assets/sprites/16x16.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(32., 32.),
        16,
        16,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite {
                color: Color::rgb(33., 33., 33.),
                index: 2,
                ..Default::default()
            },
            ..Default::default()
        });
}

fn main() {
    let window_descriptor = WindowDescriptor {
        title: "rustlike".to_string(),
        ..Default::default()
    };

    App::build()
        .insert_resource(window_descriptor)
        .add_startup_system(setup.system())
        .add_plugins(DefaultPlugins)
        .run();
}