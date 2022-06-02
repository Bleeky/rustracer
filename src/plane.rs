use crate::color::*;
use crate::point::Point;
use crate::rendering::*;
use crate::vector3::*;

pub struct Plane {
    pub origin: Point,
    pub normal: Vector3,
    pub color: Color,
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);
        // if the dot product is 0 or close to 0, the ray is parallel to the plane
        if denom > 1e-6 {
            let v = self.origin - ray.origin;
            let distance = v.dot(&normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }

    fn surface_normal(&self, _: &Point) -> Vector3 {
        -self.normal
    }
}
