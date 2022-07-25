pub mod cube;
pub mod plane;
pub mod sphere;

use crate::intersection::*;
use crate::material::*;
use crate::matrix::*;
use crate::objects::cube::*;
use crate::objects::plane::*;
use crate::objects::sphere::*;
use crate::point::*;
use crate::ray::*;
use crate::vector3::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
}

impl Object {
    pub fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        let ray2 = ray.transform(self.transform().invert());
        match *self {
            Object::Sphere(ref s) => s.intersect(&ray2),
            Object::Plane(ref s) => s.intersect(&ray2),
            Object::Cube(ref s) => s.intersect(&ray2),
        }
    }

    pub fn normal_at(&self, hit_point: &Point) -> Vector3 {
        let local_point = self.transform().invert() * *hit_point;
        let local_normal = match *self {
            Object::Sphere(ref s) => s.normal_at(&local_point),
            Object::Plane(ref s) => s.normal_at(&local_point),
            Object::Cube(ref s) => s.normal_at(&local_point),
        };
        let world_normal = self.transform().invert().transpose() * local_normal;
        world_normal.normalize()
    }

    pub fn material(&self) -> &Material {
        match *self {
            Object::Sphere(ref s) => &s.material,
            Object::Plane(ref s) => &s.material,
            Object::Cube(ref s) => &s.material,
        }
    }

    pub fn set_material(&mut self, material: Material) {
        match *self {
            Object::Sphere(ref mut s) => s.material = material,
            Object::Plane(ref mut s) => s.material = material,
            Object::Cube(ref mut s) => s.material = material,
        }
    }

    pub fn transform(&self) -> &Matrix44 {
        match *self {
            Object::Sphere(ref s) => &s.transform,
            Object::Plane(ref s) => &s.transform,
            Object::Cube(ref s) => &s.transform,
        }
    }

    pub fn set_transform(mut self, transform: Matrix44) -> Self {
        match self {
            Object::Sphere(ref mut s) => s.set_transform(transform),
            Object::Plane(ref mut s) => s.set_transform(transform),
            Object::Cube(ref mut s) => s.set_transform(transform),
        }
        self
    }
}
