use crate::color::*;
use crate::matrix::*;
use crate::patterns::*;
use crate::point::*;
use noise::{NoiseFn, Perlin};

#[derive(Clone, Debug)]
pub struct Perturbed {
    pub pattern: Box<Pattern>,
    pub transform: Matrix44,
    pub perlin: Perlin,
    pub factor: f64,
}

impl Perturbed {
    pub fn new(pattern: Pattern, factor: f64) -> Self {
        Perturbed {
            pattern: Box::new(pattern),
            perlin: Perlin::new(),
            transform: Matrix44::identity(),
            factor,
        }
    }

    pub fn pattern_at(&self, point: &Point) -> Color {
        let pt = self.transform.invert() * *point;
        let val = self.perlin.get([pt.x, pt.y, pt.z]);
        self.pattern.pattern_at(&(pt + val * self.factor))
    }
}
