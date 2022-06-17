#[cfg(test)]
mod tests {
    use crate::point::*;

    #[test]
    fn test_point_mul_translation() {
        let m = Matrix44::translation(5.0, -3.0, 2.0);
        let p = Point {
            x: -3.0,
            y: 4.0,
            z: 5.0,
        };
        assert_eq!(
            m * p,
            Point {
                x: 2.0,
                y: 1.0,
                z: 7.0,
            }
        );
    }
    #[test]
    fn test_point_mul_translation_inverse() {
        let mi = Matrix44::translation(5.0, -3.0, 2.0).invert();
        let p = Point {
            x: -3.0,
            y: 4.0,
            z: 5.0,
        };
        assert_eq!(
            mi * p,
            Point {
                x: -8.0,
                y: 7.0,
                z: 3.0,
            }
        );
    }

    #[test]
    fn test_reflection_neg_scaling() {
        let ms = Matrix44::scaling(-1.0, 1.0, 1.0);
        let p = Point {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };
        assert_eq!(
            ms * p,
            Point {
                x: -2.0,
                y: 3.0,
                z: 4.0,
            }
        );
    }

    #[test]
    fn test_point_rotate_x() {
        let p = Point {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let half_quarter = Matrix44::rotation_x(std::f64::consts::FRAC_PI_4);
        let full_quarter = Matrix44::rotation_x(std::f64::consts::FRAC_PI_2);
        assert_eq!(
            half_quarter * p,
            Point {
                x: 0.0,
                y: 2.0_f64.sqrt() / 2.0,
                z: 2.0_f64.sqrt() / 2.0,
            }
        );
        assert_eq!(
            full_quarter * p,
            Point {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            }
        );
    }

    #[test]
    fn test_inverse_rotate_x() {
        let p = Point {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let half_quarter = Matrix44::rotation_x(std::f64::consts::FRAC_PI_4).invert();
        assert_eq!(
            half_quarter * p,
            Point {
                x: 0.0,
                y: 2.0_f64.sqrt() / 2.0,
                z: -(2.0_f64.sqrt() / 2.0),
            }
        );
    }

    #[test]
    fn test_point_rotate_y() {
        let p = Point {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        let half_quarter = Matrix44::rotation_y(std::f64::consts::FRAC_PI_4);
        let full_quarter = Matrix44::rotation_y(std::f64::consts::FRAC_PI_2);
        assert_eq!(
            half_quarter * p,
            Point {
                x: 2.0_f64.sqrt() / 2.0,
                y: 0.0,
                z: 2.0_f64.sqrt() / 2.0,
            }
        );
        assert_eq!(
            full_quarter * p,
            Point {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            }
        );
    }
    #[test]
    fn test_point_rotate_z() {
        let p = Point {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let half_quarter = Matrix44::rotation_z(std::f64::consts::FRAC_PI_4);
        let full_quarter = Matrix44::rotation_z(std::f64::consts::FRAC_PI_2);
        assert_eq!(
            half_quarter * p,
            Point {
                x: -(2.0_f64.sqrt() / 2.0),
                y: 2.0_f64.sqrt() / 2.0,
                z: 0.0,
            }
        );
        assert_eq!(
            full_quarter * p,
            Point {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            }
        );
    }

    #[test]
    fn test_point_shearing_x_to_y() {
        let s = Matrix44::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };
        assert_eq!(
            s * p,
            Point {
                x: 5.0,
                y: 3.0,
                z: 4.0,
            }
        );
    }
    #[test]
    fn test_point_shearing_x_to_z() {
        let s = Matrix44::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };
        assert_eq!(
            s * p,
            Point {
                x: 6.0,
                y: 3.0,
                z: 4.0,
            }
        );
    }
    #[test]
    fn test_point_shearing_z_to_y() {
        let s = Matrix44::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Point {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };
        assert_eq!(
            s * p,
            Point {
                x: 2.0,
                y: 3.0,
                z: 7.0,
            }
        );
    }
}
