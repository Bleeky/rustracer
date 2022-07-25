use crate::intersection::*;
use crate::material::*;
use crate::matrix::*;
use crate::objects::*;
use crate::point::Point;
use crate::ray::*;
use crate::vector3::Vector3;

#[derive(Clone, Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Material,
    pub transform: Matrix44,
}

impl Sphere {
    pub fn new(material: &Material) -> Self {
        Sphere {
            center: Point::zero(),
            radius: 1.0,
            material: material.clone(),
            transform: Matrix44::identity(),
        }
    }

    pub fn glass(refractive_index: f32) -> Self {
        Sphere {
            center: Point::zero(),
            radius: 1.0,
            material: Material {
                transparency: 1.0,
                refractive_index,
                ..Material::default()
            },
            transform: Matrix44::identity(),
        }
    }

    pub fn set_transform(&mut self, transform: Matrix44) {
        self.transform = transform;
    }

    pub fn normal_at(&self, point: &Point) -> Vector3 {
        Vector3 {
            x: point.x,
            y: point.y,
            z: point.z,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        let sphere_to_ray = ray.origin - Point::zero();
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant >= 0.0 {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            return Some(vec![
                Intersection {
                    distance: t1,
                    object: Object::Sphere(self.clone()),
                },
                Intersection {
                    distance: t2,
                    object: Object::Sphere(self.clone()),
                },
            ]);
        }
        None
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center
            && self.radius == other.radius
            && self.transform == other.transform
    }
}

#[cfg(test)]
#[path = "./sphere_tests.rs"]
mod sphere_tests;
