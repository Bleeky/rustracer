use crate::color::*;
use crate::matrix::*;
use crate::patterns::*;
use crate::point::*;

#[derive(Clone, Debug)]
pub struct Checker {
    pub pattern_a: Box<Pattern>,
    pub pattern_b: Box<Pattern>,
    pub transform: Matrix44,
}

impl Checker {
    pub fn new(pattern_a: Pattern, pattern_b: Pattern) -> Self {
        Checker {
            pattern_a: Box::new(pattern_a),
            pattern_b: Box::new(pattern_b),
            transform: Matrix44::identity(),
        }
    }

    pub fn pattern_at(&self, point: &Point) -> Color {
        let pt = self.transform.invert() * *point;
        if ((pt.x + std::f64::EPSILON).floor()
            + (pt.y + std::f64::EPSILON).floor()
            + (pt.z + std::f64::EPSILON).floor())
            % 2.0
            == 0.0
        {
            return self.pattern_a.pattern_at(&pt);
        }
        self.pattern_b.pattern_at(&pt)
    }
}

#[cfg(test)]
#[path = "./checker_tests.rs"]
mod checker_tests;
