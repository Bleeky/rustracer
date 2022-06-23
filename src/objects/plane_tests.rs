#[cfg(test)]
mod tests {
    use crate::objects::plane::*;
    use crate::point::Point;
    use crate::vector3::Vector3;

    #[test]
    fn test_plane_normal() {
        let p = Plane::new(Material::default());
        let n1 = p.normal_at(&Point::zero());
        let n2 = p.normal_at(&Point {
            x: 10.0,
            y: 0.0,
            z: -10.0,
        });
        let n3 = p.normal_at(&Point {
            x: 5.0,
            y: 0.0,
            z: 150.0,
        });
        assert_eq!(
            n1,
            Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0
            }
        );
        assert_eq!(
            n2,
            Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0
            }
        );
        assert_eq!(
            n3,
            Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0
            }
        );
    }

    #[test]
    fn test_ray_plane_parallel() {
        let p = Plane::new(Material::default());
        let r = Ray {
            origin: Point {
                x: 0.0,
                y: 10.0,
                z: 0.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        assert_eq!(p.intersect(&r), None);
    }

    #[test]
    fn test_coplanar_ray_intersect() {
        let p = Plane::new(Material::default());
        let r = Ray {
            origin: Point::zero(),
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        assert_eq!(p.intersect(&r), None);
    }

    #[test]
    fn test_ray_intersect_plane_from_above() {
        let p = Plane::new(Material::default());
        let r = Ray {
            origin: Point {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
        };
        let i = p.intersect(&r);
        assert_eq!(i.unwrap()[0].distance, 1.0);
    }

    #[test]
    fn test_ray_intersect_plane_from_below() {
        let p = Plane::new(Material::default());
        let r = Ray {
            origin: Point {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        };
        let i = p.intersect(&r);
        assert_eq!(i.unwrap()[0].distance, 1.0);
    }
}
