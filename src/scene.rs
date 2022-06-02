use crate::color::*;
use crate::light::*;
use crate::plane::*;
use crate::point::Point;
use crate::rendering::*;
use crate::sphere::*;
use crate::vector3::*;

pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
}

impl Intersectable for Object {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match *self {
            Object::Sphere(ref s) => s.intersect(ray),
            Object::Plane(ref p) => p.intersect(ray),
        }
    }
    fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        match *self {
            Object::Sphere(ref s) => s.surface_normal(hit_point),
            Object::Plane(ref p) => p.surface_normal(hit_point),
        }
    }
}

impl Object {
    pub fn color(&self) -> &Color {
        match *self {
            Object::Sphere(ref s) => &s.color,
            Object::Plane(ref p) => &p.color,
        }
    }
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub shadow_bias: f64,
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}

pub struct Intersection<'a> {
    pub distance: f64,
    pub object: &'a Object,
}

impl<'a> Intersection<'a> {
    pub fn new<'b>(distance: f64, object: &'b Object) -> Intersection<'b> {
        if !distance.is_finite() {
            panic!("Intersection must have a finite distance.");
        }
        Intersection {
            distance: distance,
            object: object,
        }
    }
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.objects
            .iter()
            .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}
