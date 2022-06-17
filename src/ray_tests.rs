#[cfg(test)]
mod tests {
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
