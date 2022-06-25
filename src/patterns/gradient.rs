use crate::color::*;
use crate::matrix::*;
use crate::point::*;

#[derive(Copy, Clone, Debug)]
pub struct Gradient {
    pub color_from: Color,
    pub color_to: Color,
    pub transform: Matrix44,
}

impl Gradient {
    pub fn new(color_from: Color, color_to: Color) -> Self {
        Gradient {
            color_from,
            color_to,
            transform: Matrix44::identity(),
        }
    }

    pub fn pattern_at(&self, point: &Point) -> Color {
        let distance = self.color_to - self.color_from;
        let fraction = point.x - point.x.floor();
        self.color_from + distance * fraction as f32
    }
}

#[cfg(test)]
#[path = "./gradient_tests.rs"]
mod gradient_tests;
