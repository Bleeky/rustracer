use crate::color::*;
use crate::point::*;

#[derive(Copy, Clone, Debug)]
pub struct SolidColor {
    pub color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        SolidColor { color }
    }

    pub fn pattern_at(&self, point: &Point) -> Color {
        self.color
    }
}
