use crate::matrix::*;
use crate::point::*;
use crate::vector3::*;
use std::fmt::Display;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    pub fn position(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    pub fn transform(&self, transform: Matrix44) -> Ray {
        let origin = self.origin * transform;
        let direction = self.direction * transform;
        Ray { origin, direction }
    }
}

impl Display for Ray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Ray {{ origin: {}, direction: {} }}",
            self.origin, self.direction
        )
    }
}

#[cfg(test)]
#[path = "./ray_tests.rs"]
mod ray_tests;
