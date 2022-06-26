pub mod plane;
pub mod sphere;

use crate::intersection::*;
use crate::material::*;
use crate::matrix::*;
use crate::objects::plane::*;
use crate::objects::sphere::*;
use crate::point::*;
use crate::ray::*;
use crate::vector3::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
}

impl Object {
    pub fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        match *self {
            Object::Sphere(ref s) => s.intersect(ray),
            Object::Plane(ref s) => s.intersect(ray),
        }
    }

    pub fn normal_at(&self, hit_point: &Point) -> Vector3 {
        match *self {
            Object::Sphere(ref s) => s.normal_at(hit_point),
            Object::Plane(ref s) => s.normal_at(hit_point),
        }
    }

    pub fn material(&self) -> &Material {
        match *self {
            Object::Sphere(ref s) => &s.material,
            Object::Plane(ref s) => &s.material,
        }
    }

    pub fn set_material(&mut self, material: Material) {
        match *self {
            Object::Sphere(ref mut s) => s.material = material,
            Object::Plane(ref mut s) => s.material = material,
        }
    }

    pub fn transform(&self) -> &Matrix44 {
        match *self {
            Object::Sphere(ref s) => &s.transform,
            Object::Plane(ref s) => &s.transform,
        }
    }

    pub fn set_transform(&mut self, transform: Matrix44) {
        match *self {
            Object::Sphere(ref mut s) => s.set_transform(transform),
            Object::Plane(ref mut s) => s.set_transform(transform),
        }
    }
}
