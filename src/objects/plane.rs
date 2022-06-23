use crate::intersection::*;
use crate::material::*;
use crate::matrix::*;
use crate::objects::*;
use crate::point::Point;
use crate::ray::*;
use crate::vector3::Vector3;

#[derive(Copy, Clone, Debug)]
pub struct Plane {
    pub origin: Point,
    pub material: Material,
    pub transform: Matrix44,
}

impl Plane {
    pub fn new(material: Material) -> Self {
        Plane {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            material,
            transform: Matrix44::identity(),
        }
    }

    pub fn set_transform(&mut self, transform: Matrix44) {
        self.transform = transform;
    }

    pub fn normal_at(&self, point: &Point) -> Vector3 {
        let object_normal = Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let world_normal = self.transform.invert().transpose() * object_normal;
        world_normal.normalize()
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        let ray2 = ray.transform(self.transform.invert());
        if ray2.direction.y.abs() < f64::EPSILON {
            return None;
        }
        let t = -ray2.origin.y / ray2.direction.y;
        Some(vec![Intersection {
            distance: t,
            object: Object::Plane(*self),
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
