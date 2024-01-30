use std::f32::consts::PI;

use graphics_shapes::coord::Coord;

use crate::math::ShapeEquation;

/// https://math.stackexchange.com/questions/1958939/parametric-equation-for-rectangular-tubing-with-corner-radius
pub fn rounded_rectangle(b: isize, a: isize, r: isize, fidelity: usize) -> Vec<Coord> {
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
