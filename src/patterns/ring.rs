use crate::color::*;
use crate::matrix::*;
use crate::point::*;

#[derive(Copy, Clone, Debug)]
pub struct Ring {
    pub color_a: Color,
    pub color_b: Color,
    pub transform: Matrix44,
}

impl Ring {
    pub fn new(color_a: Color, color_b: Color) -> Self {
        Ring {
            color_a,
            color_b,
            transform: Matrix44::identity(),
        }
    }

    pub fn pattern_at(&self, point: &Point) -> Color {
        if ((point.x * point.x + point.z * point.z).sqrt()).floor() % 2.0 == 0.0 {
            return self.color_a;
        }
        self.color_b
    }
}

#[cfg(test)]
#[path = "./ring_tests.rs"]
mod ring_tests;
