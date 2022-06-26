use crate::color::*;
use crate::matrix::*;
use crate::patterns::*;
use crate::point::*;

#[derive(Clone, Debug)]
pub struct Stripe {
    pub pattern_a: Box<Pattern>,
    pub pattern_b: Box<Pattern>,
    pub transform: Matrix44,
}

impl Stripe {
    pub fn new(pattern_a: Pattern, pattern_b: Pattern) -> Self {
        Stripe {
            pattern_a: Box::new(pattern_a),
            pattern_b: Box::new(pattern_b),
            transform: Matrix44::identity(),
        }
    }

    pub fn pattern_at(&self, point: &Point) -> Color {
        let pt = self.transform.invert() * *point;
        if pt.x.floor() % 2.0 == 0.0 {
            return self.pattern_a.pattern_at(&pt);
        }
        self.pattern_b.pattern_at(&pt)
    }
}

#[cfg(test)]
#[path = "./stripe_tests.rs"]
mod stripe_tests;
