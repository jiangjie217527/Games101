pub use crate::rasterizer::to_vec4;
pub use crate::triangle::Triangle;
use nalgebra::Vector2;
pub struct Aabb {
    pub minimum: Vector2<usize>,
    pub maximum: Vector2<usize>,
}

pub fn fmax(x: f64, y: f64) -> f64 {
    if x > y {
        x
    } else {
        y
    }
}

pub fn fmin(x: f64, y: f64) -> f64 {
    if x < y {
        x
    } else {
        y
    }
}

pub fn fabs(x: f64) -> f64 {
    if x < 0.0 {
        -x
    } else {
        x
    }
}

impl Aabb {
    pub fn new(t: &Triangle) -> Self {
        // let v1 = to_vec4(t.v[0],Some(1.0));
        // let v2 = to_vec4(t.v[1],Some(1.0));
        // let v3 = to_vec4(t.v[2],Some(1.0));
        let minimum = Vector2::new(
            fmin(t.v[0].x, fmin(t.v[1].x, t.v[2].x)) as usize,
            fmin(t.v[0].y, fmin(t.v[1].y, t.v[2].y)) as usize,
        );
        let maximum = Vector2::new(
            fmax(t.v[0].x, fmax(t.v[1].x, t.v[2].x)) as usize,
            fmax(t.v[0].y, fmax(t.v[1].y, t.v[2].y)) as usize,
        );
        Self { minimum, maximum }
    }
}
