pub mod sphere;

use crate::material::*;
use crate::matrix::*;
use crate::objects::sphere::*;
use crate::point::*;
use crate::vector3::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Object {
    Sphere(Sphere),
}

impl Object {
    pub fn normal_at(&self, hit_point: &Point) -> Vector3 {
        match *self {
            Object::Sphere(ref s) => s.normal_at(hit_point),
        }
    }
    pub fn material(&self) -> &Material {
        match *self {
            Object::Sphere(ref s) => &s.material,
        }
    }
    pub fn transform(&self) -> &Matrix44 {
        match *self {
            Object::Sphere(ref s) => &s.transform,
        }
    }
}
