#[cfg(test)]
mod tests {
    use crate::objects::sphere::*;
    use crate::point::Point;
    use crate::vector3::Vector3;
    #[test]
    fn test_default_transformation() {
        let mut s = Sphere::new(&Material::default());
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
        let s = Object::Sphere(Sphere::new(&Material::default()))
            .set_transform(Matrix44::translation(5.0, 0.0, 0.0));
        let i = s.intersect(&r);
        assert_eq!(i, None);
    }

    #[test]
    fn test_scaled_intersect() {
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
        let s = Object::Sphere(Sphere::new(&Material::default()))
            .set_transform(Matrix44::scaling(2.0, 2.0, 2.0));
        let i = s.intersect(&r);
        assert!(Option::is_some(&i));
        let u = i.unwrap();
        assert_eq!(u[0].distance, 3.0);
        assert_eq!(u[1].distance, 7.0);
    }

    #[test]
    fn test_normal_at() {
        let s = Sphere::new(&Material::default());
        assert_eq!(
            s.normal_at(&Point {
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
        let s = Sphere::new(&Material::default());
        assert_eq!(
            s.normal_at(&Point {
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
        let s = Sphere::new(&Material::default());
        assert_eq!(
            s.normal_at(&Point {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            }),
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            }
        );
    }
    #[test]
    fn test_normal_at_4() {
        let s = Sphere::new(&Material::default());
        assert_eq!(
            s.normal_at(&Point {
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
    fn test_normal_is_normalized_vec() {
        let s = Sphere::new(&Material::default());
        let n = s.normal_at(&Point {
            x: 3.0_f64.sqrt() / 3.0,
            y: 3.0_f64.sqrt() / 3.0,
            z: 3.0_f64.sqrt() / 3.0,
        });
        assert_eq!(n, n.normalize());
    }

    #[test]
    fn test_normal_at_nonaxial_point() {
        let s = Sphere::new(&Material::default());
        assert_eq!(
            s.normal_at(&Point {
                x: 3.0_f64.sqrt() / 3.0,
                y: 3.0_f64.sqrt() / 3.0,
                z: 3.0_f64.sqrt() / 3.0,
            }),
            Vector3 {
                x: 3.0_f64.sqrt() / 3.0,
                y: 3.0_f64.sqrt() / 3.0,
                z: 3.0_f64.sqrt() / 3.0,
            }
        );
    }

    #[test]
    fn test_normal_at_translated() {
        let mut s = Sphere::new(&Material::default());
        s.set_transform(Matrix44::translation(0.0, 1.0, 0.0));
        assert_eq!(
            s.normal_at(&Point {
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

    #[test]
    fn test_normal_on_transformed() {
        let mut s = Sphere::new(&Material::default());
        s.set_transform(Matrix44::rotation_z(std::f64::consts::PI / 5.0).scale(1.0, 0.5, 1.0));
        assert_eq!(
            s.normal_at(&Point {
                x: 0.0,
                y: 2.0_f64.sqrt() / 2.0,
                z: -2.0_f64.sqrt() / 2.0,
            }),
            Vector3 {
                x: 0.0,
                y: 0.9701425001453319,
                z: -0.24253562503633297,
            }
        );
    }
}
