use crate::matrix::*;
use crate::point::*;
use crate::vector3::*;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    pub fn position(&self, distance: f64) -> Point {
        self.origin + (self.direction * distance)
    }

    pub fn transform(&self, transform: Matrix44) -> Ray {
        let origin = transform * self.origin;
        let direction = transform * self.direction;
        Ray {
            origin: origin,
            direction: direction,
        }
    }
}

#[cfg(test)]
#[path = "./ray_tests.rs"]
mod ray_tests;
