use crate::matrix::*;
use crate::vector3::*;
use std::fmt::Display;
use std::ops::{Add, Mul, Sub};
#[derive(Copy, Clone, Debug)]

pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn zero() -> Point {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Add<Vector3> for Point {
    type Output = Point;

    fn add(self, other: Vector3) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Vector3;

    fn sub(self, other: Point) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<Matrix44> for Point {
    type Output = Point;

    fn mul(self, other: Matrix44) -> Point {
        Point {
            x: self.x * other.elements[0][0]
                + self.y * other.elements[0][1]
                + self.z * other.elements[0][2]
                + other.elements[0][3],
            y: self.x * other.elements[1][0]
                + self.y * other.elements[1][1]
                + self.z * other.elements[1][2]
                + other.elements[1][3],
            z: self.x * other.elements[2][0]
                + self.y * other.elements[2][1]
                + self.z * other.elements[2][2]
                + other.elements[2][3],
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < 1e-15
            && (self.y - other.y).abs() < 1e-15
            && (self.z - other.z).abs() < 1e-15
    }
}

impl Display for Point {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "Point\n[{},{},{}]", self.x, self.y, self.z,)
    }
}

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
