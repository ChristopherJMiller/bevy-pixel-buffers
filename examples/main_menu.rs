use bevy::prelude::*;
use bevy_pixel_buffers::BufferedImage;
use buffer_graphics_lib::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let mut buffered_image = BufferedImage::new(256, 32);
    let text = buffer_graphics_lib::prelude::Text::new(
        "Some text",
        TextPos::cr((1, 1)),
        (WHITE, TextSize::Large),
    );

    buffered_image.graphics().unwrap().draw(&text);

    commands.spawn(SpriteBundle {
        texture: asset_server.add(buffered_image.image()),
        ..default()
    });
}
