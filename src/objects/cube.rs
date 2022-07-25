use std::cmp::max;

use crate::intersection::*;
use crate::material::*;
use crate::matrix::*;
use crate::objects::*;
use crate::point::Point;
use crate::ray::*;
use crate::vector3::Vector3;

#[derive(Clone, Debug)]
pub struct Cube {
    pub material: Material,
    pub transform: Matrix44,
}

impl Cube {
    pub fn new(material: &Material) -> Self {
        Cube {
            material: material.clone(),
            transform: Matrix44::identity(),
        }
    }

    pub fn set_transform(&mut self, transform: Matrix44) {
        self.transform = transform;
    }

    pub fn normal_at(&self, point: &Point) -> Vector3 {
        let maxc = vec![point.x.abs(), point.y.abs(), point.z.abs()];
        let maxc = maxc.iter().copied().fold(f64::NAN, f64::max);

        if maxc == point.x.abs() {
            return Vector3 {
                x: point.x,
                y: 0.0,
                z: 0.0,
            };
        } else if maxc == point.y.abs() {
            return Vector3 {
                x: 0.0,
                y: point.y,
                z: 0.0,
            };
        }
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: point.z,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        let xminmax = self.check_axis(ray.origin.x, ray.direction.x);
        let yminmax = self.check_axis(ray.origin.y, ray.direction.y);
        let zminmax = self.check_axis(ray.origin.z, ray.direction.z);
        let tmin = vec![xminmax.0, yminmax.0, zminmax.0];
        let tmin = tmin.iter().copied().fold(f64::NAN, f64::max);
        let tmax = vec![xminmax.1, yminmax.1, zminmax.1];
        let tmax = tmax.iter().copied().fold(f64::NAN, f64::min);
        if tmin > tmax {
            return None;
        }
        Some(vec![
            Intersection {
                distance: tmin,
                object: Object::Cube(self.clone()),
            },
            Intersection {
                distance: tmax,
                object: Object::Cube(self.clone()),
            },
        ])
    }

    fn check_axis(&self, origin: f64, direction: f64) -> (f64, f64) {
        let tmin_numerator = -1.0 - origin;
        let tmax_numerator = 1.0 - origin;
        let tmin;
        let tmax;
        if direction.abs() >= std::f64::EPSILON {
            tmin = tmin_numerator / direction;
            tmax = tmax_numerator / direction;
        } else {
            tmin = tmin_numerator * std::f64::INFINITY;
            tmax = tmax_numerator * std::f64::INFINITY;
        }
        if tmin > tmax {
            return (tmax, tmin);
        }
        (tmin, tmax)
    }
}

impl PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        self.material == other.material && self.transform == other.transform
    }
}

#[cfg(test)]
#[path = "./cube_tests.rs"]
mod cube_tests;
