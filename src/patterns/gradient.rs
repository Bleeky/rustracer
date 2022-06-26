use crate::color::*;
use crate::matrix::*;
use crate::patterns::*;
use crate::point::*;

#[derive(Clone, Debug)]
pub struct Gradient {
    pub pattern_a: Box<Pattern>,
    pub pattern_b: Box<Pattern>,
    pub transform: Matrix44,
}

impl Gradient {
    pub fn new(pattern_a: Pattern, pattern_b: Pattern) -> Self {
        Gradient {
            pattern_a: Box::new(pattern_a),
            pattern_b: Box::new(pattern_b),
            transform: Matrix44::identity(),
        }
    }

    pub fn pattern_at(&self, point: &Point) -> Color {
        let pt = self.transform.invert() * *point;
        let distance = self.pattern_b.pattern_at(&pt) - self.pattern_a.pattern_at(&pt);
        let fraction = pt.x - pt.x.floor();
        self.pattern_a.pattern_at(&pt) + distance * fraction as f32
    }
}

#[cfg(test)]
#[path = "./gradient_tests.rs"]
mod gradient_tests;
