use crate::color::*;
use crate::matrix::*;
use crate::patterns::*;
use crate::point::*;

#[derive(Clone, Debug)]
pub struct RadialGradient {
    pub pattern_a: Box<Pattern>,
    pub pattern_b: Box<Pattern>,
    pub transform: Matrix44,
}

impl RadialGradient {
    pub fn new(pattern_a: Pattern, pattern_b: Pattern) -> Self {
        RadialGradient {
            pattern_a: Box::new(pattern_a),
            pattern_b: Box::new(pattern_b),
            transform: Matrix44::identity(),
        }
    }

    pub fn pattern_at(&self, point: &Point) -> Color {
        let pt = self.transform.invert() * *point;
        let distance = (pt.x * pt.x + pt.z * pt.z).sqrt();
        let fraction = distance - distance.floor();
        self.pattern_a.pattern_at(&pt)
            + (self.pattern_b.pattern_at(&pt) - self.pattern_a.pattern_at(&pt)) * fraction as f32
    }
}
