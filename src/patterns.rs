pub mod checker;
pub mod gradient;
pub mod ring;
pub mod stripe;

use crate::color::*;
use crate::matrix::*;
use crate::objects::*;
use crate::patterns::checker::*;
use crate::patterns::gradient::*;
use crate::patterns::ring::*;
use crate::patterns::stripe::*;
use crate::point::*;

#[derive(Copy, Clone, Debug)]
pub enum Pattern {
    Stripe(Stripe),
    Gradient(Gradient),
    Ring(Ring),
    Checker(Checker),
}

impl Pattern {
    pub fn pattern_at_object(&self, point: &Point, object: &Object) -> Color {
        let object_point = object.transform().invert() * *point;
        let pattern_point = self.transform().invert() * object_point;
        match *self {
            Pattern::Stripe(ref s) => s.pattern_at(&pattern_point),
            Pattern::Gradient(ref s) => s.pattern_at(&pattern_point),
            Pattern::Ring(ref s) => s.pattern_at(&pattern_point),
            Pattern::Checker(ref s) => s.pattern_at(&pattern_point),
        }
    }

    pub fn transform(&self) -> &Matrix44 {
        match *self {
            Pattern::Stripe(ref s) => &s.transform,
            Pattern::Gradient(ref s) => &s.transform,
            Pattern::Ring(ref s) => &s.transform,
            Pattern::Checker(ref s) => &s.transform,
        }
    }

    pub fn set_transform(&mut self, transform: Matrix44) -> Self {
        match *self {
            Pattern::Stripe(ref mut s) => s.transform = transform,
            Pattern::Gradient(ref mut s) => s.transform = transform,
            Pattern::Ring(ref mut s) => s.transform = transform,
            Pattern::Checker(ref mut s) => s.transform = transform,
        }
        *self
    }
}
