use crate::color::*;
use crate::point::Point;
use crate::rendering::*;
use crate::vector3::*;

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        //Create a line segment between the ray origin and the center of the sphere
        let vl: Vector3 = self.center - ray.origin;
        //Use vl as a hypotenuse and find the length of the adjacent side (since direction is a unit vector)
        // ajd = Vl . Vray = |Vl||Vray|cos(o)
        let adj = vl.dot(&ray.direction);
        //Find the length-squared of the opposite side
        //This is equivalent to (but faster than) (l.length() * l.length()) - (adj2 * adj2)
        let d2 = vl.dot(&vl) - (adj * adj);
        //If that length-squared is less than radius squared, the ray intersects the sphere
        let radius2 = self.radius * self.radius;
        if d2 > radius2 {
            return None;
        }
        let thc = (radius2 - d2).sqrt();
        let t0 = adj - thc;
        let t1 = adj + thc;

        if t0 < 0.0 && t1 < 0.0 {
            return None;
        }

        let distance = if t0 < t1 { t0 } else { t1 };
        Some(distance)
    }

    fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        (*hit_point - self.center).normalize()
    }
}
