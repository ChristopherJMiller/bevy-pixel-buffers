use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_pixel_buffers::{
    buffer::BufferedImage, math::ShapeEquation, plugin::BufferedImageUpdatePlugin,
};
use buffer_graphics_lib::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BufferedImageUpdatePlugin)
        .add_systems(Startup, setup)
        //.add_systems(Update, update_time_on_button)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let mut buffered_image = BufferedImage::new(256, 128);
    let mut graphics = buffered_image.graphics().unwrap();

    button(&mut graphics, "Play", 255, 64, 25);

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
            256,
            32,
            5,
        );
    });
}

fn button(graphics: &mut Graphics, text: &str, width: isize, height: isize, radius: isize) {
    let points_list = rounded_rectangle(width / 2, height / 2, radius, 2);

    info!("{:?}", points_list);

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

/// https://math.stackexchange.com/questions/1958939/parametric-equation-for-rectangular-tubing-with-corner-radius
fn rounded_rectangle(b: isize, a: isize, r: isize, fidelity: usize) -> Vec<Coord> {
    let a = a as f32;
    let b = b as f32;
    let r = r as f32;
    let fidelity = fidelity as isize;

    let inv_fidelity = 1.0 / fidelity as f32;

    let t_step = (1..=8)
        .map(|x| ((x - 1) * fidelity..=x * fidelity))
        .collect::<Vec<_>>();

    ShapeEquation::Parametric {
        domain: (0..8 * fidelity),
        xy: |t| {
            let step = t_step
                .iter()
                .enumerate()
                .find(|step| step.1.contains(&t))
                .unwrap_or_else(|| panic!("Failed to find range for t value {}", t))
                .0;
            let t = t as f32 * inv_fidelity;
            let x = match step {
                0 => -(b - r) * (2.0 * t - 1.0),
                1 => -b + r - r * (0.5 * PI * (t - 1.0)).sin(),
                2 => -b,
                3 => -b + r - r * (0.5 * PI * (t - 3.0)).cos(),
                4 => (b - r) * (2.0 * t - 9.0),
                5 => b - r + r * (0.5 * PI * (t - 5.0)).sin(),
                6 => b,
                7 => b - r + r * (0.5 * PI * (t - 7.0)).cos(),
                _ => 0.0,
            }
            .round() as isize;

            let y = match step {
                0 => a,
                1 => a - r + r * (0.5 * PI * (t - 1.0)).cos(),
                2 => -(a - r) * (2.0 * t - 5.0),
                3 => -a + r - r * (0.5 * PI * (t - 3.0)).sin(),
                4 => -a,
                5 => -a + r - r as f32 * (0.5 * PI * (t as f32 - 5.0)).cos(),
                6 => (a - r) * (2.0 * t - 13.0),
                7 => a - r + r * (0.5 * PI * (t as f32 - 7.0)).sin(),
                _ => 0.0,
            }
            .round() as isize;

            // Bottom left hand corner drawing
            (x + b as isize, y + a as isize)
        },
    }
    .into_points(1)
}
