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

    pub fn normal_at(&self, point: Point) -> Vector3 {
        let object_point = self.transform.invert() * point;
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
mod tests {
    use crate::point::*;
    use crate::ray::*;
    use crate::sphere::*;
    use crate::vector3::*;
    #[test]
    fn test_default_transformation() {
        let mut s = Sphere::new(Material::default());
        assert_eq!(s.transform, Matrix44::identity());
        let t = Matrix44::translation(2.0, 3.0, 4.0);
        s.set_transform(t);
        assert_eq!(s.transform, t);
    }
    #[test]
    fn test_translated_intersect() {
        let r = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let mut s = Sphere::new(Material::default());
        let t = Matrix44::translation(5.0, 0.0, 0.0);
        s.set_transform(t);
        let i = intersect(&r, &s);
        assert_eq!(i, None);
    }
    #[test]
    fn test_scaled_intersect_2() {
        let r = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let mut s = Sphere::new(Material::default());
        let t = Matrix44::scaling(2.0, 2.0, 2.0);
        s.set_transform(t);
        let i = intersect(&r, &s);
        assert_eq!(i.unwrap().0.distance, 3.0);
        assert_eq!(i.unwrap().1.distance, 7.0);
    }
    #[test]
    fn test_normal_at() {
        let s = Sphere::new(Material::default());
        assert_eq!(
            s.normal_at(Point {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            }),
            Vector3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            }
        );
    }
    #[test]
    fn test_normal_at_2() {
        let s = Sphere::new(Material::default());
        assert_eq!(
            s.normal_at(Point {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            }),
            Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            }
        );
    }
    #[test]
    fn test_normal_at_3() {
        let s = Sphere::new(Material::default());
        assert_eq!(
            s.normal_at(Point {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            }),
            Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            }
            .normalize()
        );
    }
    #[test]
    fn test_normal_at_translated() {
        let mut s = Sphere::new(Material::default());
        s.set_transform(Matrix44::translation(0.0, 1.0, 0.0));
        assert_eq!(
            s.normal_at(Point {
                x: 0.0,
                y: 1.70711,
                z: -0.70711,
            }),
            Vector3 {
                x: 0.0,
                y: 0.7071067811865475,
                z: -0.7071067811865476,
            }
        );
    }
}
