use std::ops::Range;

use graphics_shapes::coord::Coord;

pub enum ShapeEquation<F, Y>
where
    F: Fn(isize) -> Y,
    Coord: From<Y>,
{
    /// Input of X to return Y
    Linear { domain: Range<isize>, y: F },
    /// Input of T into separate X and Y equations
    Parametric { domain: Range<isize>, xy: F },
}

impl<F: Fn(isize) -> Y, Y> ShapeEquation<F, Y>
where
    Coord: From<Y>,
{
    pub fn into_points<'a>(self, step: usize) -> Vec<Coord> {
        match self {
            ShapeEquation::Linear { domain, y } => {
                let steps = domain.step_by(step);
                steps.map(|x| Coord::from(y(x))).collect()
            }
            ShapeEquation::Parametric { domain, xy } => {
                let steps = domain.step_by(step);
                steps.map(|t| Coord::from(xy(t))).collect()
            }
        }
    }
}
