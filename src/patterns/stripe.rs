use crate::color::*;
use crate::matrix::*;
use crate::point::*;

#[derive(Copy, Clone, Debug)]
pub struct Stripe {
    pub color_a: Color,
    pub color_b: Color,
    pub transform: Matrix44,
}

impl Stripe {
    pub fn new(color_a: Color, color_b: Color) -> Self {
        Stripe {
            color_a,
            color_b,
            transform: Matrix44::identity(),
        }
    }

    pub fn pattern_at(&self, point: &Point) -> Color {
        if point.x.floor() % 2.0 == 0.0 {
            return self.color_a;
        }
        self.color_b
    }
}

#[cfg(test)]
#[path = "./stripe_tests.rs"]
mod stripe_tests;
