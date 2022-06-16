use crate::intersection::*;
use crate::material::*;
use crate::matrix::*;
use crate::objects::*;
use crate::point::Point;
use crate::ray::*;
use crate::vector3::Vector3;

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Material,
    pub transform: Matrix44,
}

impl Sphere {
    pub fn new(material: Material) -> Self {
        Sphere {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 1.0,
            material,
            transform: Matrix44::identity(),
        }
    }

    pub fn set_transform(&mut self, transform: Matrix44) {
        self.transform = transform;
    }

    pub fn normal_at(&self, point: &Point) -> Vector3 {
        let object_point = self.transform.invert() * *point;
        let object_normal = object_point - self.center;
        let world_normal = self.transform.invert().transpose() * object_normal;
        world_normal.normalize()
    }

    pub fn intersect<'a>(&self, ray: &Ray) -> Option<(f64, f64)> {
        let ray2 = ray.transform(self.transform.invert());
        let oc = ray2.origin
            - Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        let a = ray2.direction.dot(&ray2.direction);
        let b = 2.0 * oc.dot(&ray2.direction);
        let c = oc.dot(&oc) - 1.0;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant >= 0.0 {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            return Some((t1, t2));
        }
        None
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center && self.radius == other.radius
    }
}

#[cfg(test)]
#[path = "./sphere_tests.rs"]
mod sphere_tests;
