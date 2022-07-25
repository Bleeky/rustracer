use crate::intersection::*;
use crate::material::*;
use crate::matrix::*;
use crate::objects::*;
use crate::point::Point;
use crate::ray::*;
use crate::vector3::Vector3;

#[derive(Clone, Debug)]
pub struct Plane {
    pub origin: Point,
    pub material: Material,
    pub transform: Matrix44,
}

impl Plane {
    pub fn new(material: Material) -> Self {
        Plane {
            origin: Point::zero(),
            material,
            transform: Matrix44::identity(),
        }
    }

    pub fn set_transform(&mut self, transform: Matrix44) {
        self.transform = transform;
    }

    pub fn normal_at(&self, _point: &Point) -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        if ray.direction.y.abs() < f64::EPSILON {
            return None;
        }
        let t = -ray.origin.y / ray.direction.y;
        Some(vec![Intersection {
            distance: t,
            object: Object::Plane(self.clone()),
        }])
    }
}

impl PartialEq for Plane {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.transform == other.transform
    }
}

#[cfg(test)]
#[path = "./plane_tests.rs"]
mod plane_tests;
