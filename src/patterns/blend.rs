use crate::color::*;
use crate::matrix::*;
use crate::patterns::*;
use crate::point::*;

#[derive(Clone, Debug)]
pub struct Blend {
    pub pattern_a: Box<Pattern>,
    pub pattern_b: Box<Pattern>,
    pub transform: Matrix44,
    pub blend_ratio: f32,
}

impl Blend {
    pub fn new(pattern_a: Pattern, pattern_b: Pattern, blend_ratio: f32) -> Self {
        Blend {
            pattern_a: Box::new(pattern_a),
            pattern_b: Box::new(pattern_b),
            blend_ratio,
            transform: Matrix44::identity(),
        }
    }

    pub fn pattern_at(&self, point: &Point) -> Color {
        let pt = self.transform.invert() * *point;
        self.pattern_a.pattern_at(&pt) * (1.0 - self.blend_ratio)
            + self.pattern_b.pattern_at(&pt) * self.blend_ratio
    }
}
