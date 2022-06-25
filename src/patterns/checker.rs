use crate::color::*;
use crate::matrix::*;
use crate::point::*;

#[derive(Copy, Clone, Debug)]
pub struct Checker {
    pub color_a: Color,
    pub color_b: Color,
    pub transform: Matrix44,
}

impl Checker {
    pub fn new(color_a: Color, color_b: Color) -> Self {
        Checker {
            color_a,
            color_b,
            transform: Matrix44::identity(),
        }
    }

    pub fn pattern_at(&self, point: &Point) -> Color {
        if (point.x.floor() + point.y.floor() + point.z.floor()) % 2.0 == 0.0 {
            return self.color_a;
        }
        self.color_b
    }
}

#[cfg(test)]
#[path = "./checker_tests.rs"]
mod checker_tests;
