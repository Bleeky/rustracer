pub mod blend;
pub mod checker;
pub mod gradient;
pub mod perturbed;
pub mod radial_gradient;
pub mod ring;
pub mod solid_color;
pub mod stripe;

use crate::color::*;
use crate::matrix::*;
use crate::objects::*;
use crate::patterns::blend::*;
use crate::patterns::checker::*;
use crate::patterns::gradient::*;
use crate::patterns::perturbed::*;
use crate::patterns::radial_gradient::*;
use crate::patterns::ring::*;
use crate::patterns::solid_color::*;
use crate::patterns::stripe::*;
use crate::point::*;

#[derive(Clone, Debug)]
pub enum Pattern {
    Stripe(Stripe),
    Gradient(Gradient),
    RadialGradient(RadialGradient),
    Ring(Ring),
    Checker(Checker),
    SolidColor(SolidColor),
    Perturbed(Perturbed),
    Blend(Blend),
    TestPattern,
}

impl Pattern {
    pub fn pattern_at_object(&self, point: &Point, object: &Object) -> Color {
        let object_point = object.transform().invert() * *point;
        match *self {
            Pattern::Stripe(ref s) => s.pattern_at(&object_point),
            Pattern::Gradient(ref s) => s.pattern_at(&object_point),
            Pattern::Ring(ref s) => s.pattern_at(&object_point),
            Pattern::Checker(ref s) => s.pattern_at(&object_point),
            Pattern::RadialGradient(ref s) => s.pattern_at(&object_point),
            Pattern::SolidColor(ref s) => s.pattern_at(&object_point),
            Pattern::Blend(ref s) => s.pattern_at(&object_point),
            Pattern::Perturbed(ref s) => s.pattern_at(&object_point),
            Pattern::TestPattern => Color {
                red: point.x as f32,
                green: point.y as f32,
                blue: point.z as f32,
            },
        }
    }

    pub fn pattern_at(&self, point: &Point) -> Color {
        match *self {
            Pattern::Stripe(ref s) => s.pattern_at(&point),
            Pattern::Gradient(ref s) => s.pattern_at(&point),
            Pattern::Ring(ref s) => s.pattern_at(&point),
            Pattern::Checker(ref s) => s.pattern_at(&point),
            Pattern::RadialGradient(ref s) => s.pattern_at(&point),
            Pattern::SolidColor(ref s) => s.pattern_at(&point),
            Pattern::Blend(ref s) => s.pattern_at(&point),
            Pattern::Perturbed(ref s) => s.pattern_at(&point),
            Pattern::TestPattern => Color {
                red: point.x as f32,
                green: point.y as f32,
                blue: point.z as f32,
            },
        }
    }

    pub fn set_transform(mut self, transform: Matrix44) -> Self {
        match self {
            Pattern::Stripe(ref mut s) => s.transform = transform,
            Pattern::Gradient(ref mut s) => s.transform = transform,
            Pattern::Ring(ref mut s) => s.transform = transform,
            Pattern::Checker(ref mut s) => s.transform = transform,
            Pattern::RadialGradient(ref mut s) => s.transform = transform,
            Pattern::Blend(ref mut s) => s.transform = transform,
            Pattern::Perturbed(ref mut s) => s.transform = transform,
            Pattern::SolidColor(ref mut _s) => {}
            Pattern::TestPattern => {}
        }
        self
    }
}
