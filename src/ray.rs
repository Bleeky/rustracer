use crate::intersection::*;
use crate::matrix::*;
use crate::objects::*;
use crate::point::*;
use crate::vector3::*;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    pub fn position(&self, distance: f64) -> Point {
        self.origin + (self.direction * distance)
    }

    pub fn transform(&self, transform: Matrix44) -> Ray {
        let origin = transform * self.origin;
        let direction = transform * self.direction;
        Ray {
            origin: origin,
            direction: direction,
        }
    }
}

pub fn intersect<'a>(
    ray: &Ray,
    object: &'a Object,
) -> Option<(Intersection<'a>, Intersection<'a>)> {
    let ray2 = ray.transform(object.transform().invert());
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
        return Some((
            Intersection {
                distance: t1,
                object: object,
            },
            Intersection {
                distance: t2,
                object: object,
            },
        ));
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::material::*;
    use crate::objects::sphere::*;
    use crate::ray::*;

    #[test]
    fn test_origin_direction_equality() {
        let origin = Point {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let direction = Vector3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let ray = Ray { origin, direction };
        assert_eq!(ray.direction, direction);
        assert_eq!(ray.origin, origin);
    }
    #[test]
    fn test_position() {
        let ray = Ray {
            origin: Point {
                x: 2.0,
                y: 3.0,
                z: 4.0,
            },
            direction: Vector3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        };
        assert_eq!(
            ray.position(0.0),
            Point {
                x: 2.0,
                y: 3.0,
                z: 4.0,
            }
        );
        assert_eq!(
            ray.position(1.0),
            Point {
                x: 3.0,
                y: 3.0,
                z: 4.0,
            }
        );
        assert_eq!(
            ray.position(-1.0),
            Point {
                x: 1.0,
                y: 3.0,
                z: 4.0,
            }
        );
        assert_eq!(
            ray.position(2.5),
            Point {
                x: 4.5,
                y: 3.0,
                z: 4.0,
            }
        );
    }

    #[test]
    fn test_intersection_1() {
        let ray = Ray {
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
        let sphere = Sphere::new(Material::default());
        let i = intersect(&ray, &sphere);
        assert_eq!(i.unwrap().0.distance, 4.0);
        assert_eq!(i.unwrap().1.distance, 6.0);
    }
    #[test]
    fn test_intersection_2() {
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 2.0,
                z: -5.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let sphere = Sphere::new(Material::default());
        let i = intersect(&ray, &sphere);
        assert_eq!(i, None);
    }
    #[test]
    fn test_intersection_3() {
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 1.0,
                z: -5.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let sphere = Sphere::new(Material::default());
        let i = intersect(&ray, &sphere);
        assert_eq!(i.unwrap().0.distance, 5.0);
        assert_eq!(i.unwrap().1.distance, 5.0);
    }
    #[test]
    fn test_intersection_4() {
        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let sphere = Sphere::new(Material::default());
        let i = intersect(&ray, &sphere);
        assert_eq!(i.unwrap().0.distance, -1.0);
        assert_eq!(i.unwrap().1.distance, 1.0);
    }

    #[test]
    fn test_transform_ray_1() {
        let ray = Ray {
            origin: Point {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        };
        let translation = Matrix44::translation(3.0, 4.0, 5.0);
        let transformed_ray = ray.transform(translation);
        assert_eq!(
            transformed_ray.origin,
            Point {
                x: 4.0,
                y: 6.0,
                z: 8.0,
            }
        );
        assert_eq!(
            transformed_ray.direction,
            Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            }
        );
    }
    #[test]
    fn test_transform_ray_2() {
        let ray = Ray {
            origin: Point {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        };
        let scaling = Matrix44::scaling(2.0, 3.0, 4.0);
        let transformed_ray = ray.transform(scaling);
        assert_eq!(
            transformed_ray.origin,
            Point {
                x: 2.0,
                y: 6.0,
                z: 12.0,
            }
        );
        assert_eq!(
            transformed_ray.direction,
            Vector3 {
                x: 0.0,
                y: 3.0,
                z: 0.0,
            }
        );
    }
}
