use crate::material::*;
use crate::matrix::*;
use crate::point::Point;
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
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center && self.radius == other.radius
    }
}

#[cfg(test)]
#[path = "./sphere_tests.rs"]
mod sphere_tests;
