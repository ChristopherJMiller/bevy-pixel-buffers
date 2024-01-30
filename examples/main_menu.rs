use bevy::prelude::*;
use bevy_pixel_buffers::{
    buffer::BufferedImage, plugin::BufferedImageUpdatePlugin, shapes::rounded_rectangle,
};
use buffer_graphics_lib::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BufferedImageUpdatePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update_time_on_button)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let mut buffered_image = BufferedImage::new(256, 128);
    let mut graphics = buffered_image.graphics().unwrap();

    button(&mut graphics, "Play", 255, 64, 5);

    commands
        .spawn(SpriteBundle {
            texture: asset_server.add(buffered_image.image()),
            ..default()
        })
        .insert(buffered_image);
}

fn update_time_on_button(time: Res<Time>, mut query: Query<&mut BufferedImage>) {
    query.for_each_mut(|mut image| {
        let mut graphics = image.graphics().unwrap();
        graphics.clear(TRANSPARENT);
        button(
            &mut graphics,
            &time.elapsed_seconds_f64().floor().to_string(),
            255,
            32,
            ((1.0 * time.elapsed_seconds_f64().floor()).round() as isize).min(16),
        );
    });
}

fn button(graphics: &mut Graphics, text: &str, width: isize, height: isize, radius: isize) {
    let points_list = rounded_rectangle(width / 2, height / 2, radius, 2);

    let polygon = graphics_shapes::polygon::Polygon::from_points(&points_list);

    graphics.draw_polygon(polygon.clone(), DrawType::Fill(BLACK));
    graphics.draw_polygon(polygon, DrawType::Stroke(WHITE));

    let (text_width, text_height) =
        TextSize::Large.measure(text, WrappingStrategy::Cutoff(width as usize));

    let start_x = (width as usize / 2) - (text_width / 2);
    let start_y = (height as usize / 2) - (text_height / 2);

    let text = buffer_graphics_lib::prelude::Text::new(
        text,
        TextPos::Px(start_x as isize, start_y as isize),
        (WHITE, TextSize::Large),
    );

    graphics.draw(&text);
}
