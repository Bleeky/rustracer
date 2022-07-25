#[cfg(test)]
mod tests {
    use crate::material::Material;
    use crate::objects::cube::Cube;
    use crate::objects::Object;
    use crate::point::Point;
    use crate::ray::Ray;
    use crate::vector3::Vector3;
    #[test]
    fn test_ray_intersect_cube() {
        pub struct Test {
            pub ray: Ray,
            pub t1: f64,
            pub t2: f64,
        }
        let c = Cube::new(&Material::default());
        let tests = vec![
            Test {
                ray: Ray {
                    origin: Point {
                        x: 5.0,
                        y: 0.5,
                        z: 0.0,
                    },
                    direction: Vector3 {
                        x: -1.0,
                        y: 0.0,
                        z: 0.0,
                    },
                },
                t1: 4.0,
                t2: 6.0,
            },
            Test {
                ray: Ray {
                    origin: Point {
                        x: -5.0,
                        y: 0.5,
                        z: 0.0,
                    },
                    direction: Vector3 {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                    },
                },
                t1: 4.0,
                t2: 6.0,
            },
            Test {
                ray: Ray {
                    origin: Point {
                        x: 0.5,
                        y: 5.0,
                        z: 0.0,
                    },
                    direction: Vector3 {
                        x: 0.0,
                        y: -1.0,
                        z: 0.0,
                    },
                },
                t1: 4.0,
                t2: 6.0,
            },
            Test {
                ray: Ray {
                    origin: Point {
                        x: 0.5,
                        y: -5.0,
                        z: 0.0,
                    },
                    direction: Vector3 {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                },
                t1: 4.0,
                t2: 6.0,
            },
            Test {
                ray: Ray {
                    origin: Point {
                        x: 0.5,
                        y: 0.0,
                        z: 5.0,
                    },
                    direction: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: -1.0,
                    },
                },
                t1: 4.0,
                t2: 6.0,
            },
            Test {
                ray: Ray {
                    origin: Point {
                        x: 0.5,
                        y: 0.0,
                        z: -5.0,
                    },
                    direction: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 1.0,
                    },
                },
                t1: 4.0,
                t2: 6.0,
            },
        ];
        for test in tests {
            let i = c.intersect(&test.ray);
            let u = i.unwrap();
            assert_eq!(u.len(), 2);
            assert_eq!(u[0].distance, test.t1);
            assert_eq!(u[1].distance, test.t2);
        }
    }

    #[test]
    fn ray_miss_cube() {
        let c = Cube::new(&Material::default());
        let tests = vec![
            Ray {
                origin: Point {
                    x: -2.0,
                    y: 0.0,
                    z: 0.0,
                },
                direction: Vector3 {
                    x: 0.2673,
                    y: 0.5345,
                    z: 0.8018,
                },
            },
            Ray {
                origin: Point {
                    x: 0.0,
                    y: -2.0,
                    z: 0.0,
                },
                direction: Vector3 {
                    x: 0.8018,
                    y: 0.2673,
                    z: 0.5345,
                },
            },
            Ray {
                origin: Point {
                    x: 0.0,
                    y: 0.0,
                    z: -2.0,
                },
                direction: Vector3 {
                    x: 0.5345,
                    y: 0.8018,
                    z: 0.2673,
                },
            },
            Ray {
                origin: Point {
                    x: 2.0,
                    y: 0.0,
                    z: 2.0,
                },
                direction: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                },
            },
            Ray {
                origin: Point {
                    x: 0.0,
                    y: 2.0,
                    z: 2.0,
                },
                direction: Vector3 {
                    x: 0.0,
                    y: -1.0,
                    z: 0.0,
                },
            },
            Ray {
                origin: Point {
                    x: 2.0,
                    y: 2.0,
                    z: 0.0,
                },
                direction: Vector3 {
                    x: -1.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
        ];
        for test in tests {
            let i = c.intersect(&test);
            assert_eq!(i, None);
        }
    }

    #[test]
    fn normal_cube() {
        pub struct Test {
            pub point: Point,
            pub normal: Vector3,
        }
        let c = Object::Cube(Cube::new(&Material::default()));
        let tests = vec![
            Test {
                point: Point {
                    x: 1.0,
                    y: 0.5,
                    z: -0.8,
                },
                normal: Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
            Test {
                point: Point {
                    x: -1.0,
                    y: -0.2,
                    z: 0.9,
                },
                normal: Vector3 {
                    x: -1.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
            Test {
                point: Point {
                    x: -0.4,
                    y: 1.0,
                    z: -0.1,
                },
                normal: Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
            },
            Test {
                point: Point {
                    x: 0.3,
                    y: -1.0,
                    z: -0.7,
                },
                normal: Vector3 {
                    x: 0.0,
                    y: -1.0,
                    z: 0.0,
                },
            },
            Test {
                point: Point {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
                normal: Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
            Test {
                point: Point {
                    x: -1.0,
                    y: -1.0,
                    z: -1.0,
                },
                normal: Vector3 {
                    x: -1.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
        ];
        for test in tests {
            let i = c.normal_at(&test.point);
            assert_eq!(i, test.normal);
        }
    }
}
