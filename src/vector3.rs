use std::fmt::Display;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn length(&self) -> f64 {
        self.norm().sqrt()
    }

    pub fn norm(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&self) -> Vector3 {
        let length = self.length();
        if length > 0.0 {
            let inv_len = 1.0 / length;
            return Vector3 {
                x: self.x * inv_len,
                y: self.y * inv_len,
                z: self.z * inv_len,
            };
        }
        *self
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Display for Vector3 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "Vector3\n[{},{},{}]", self.x, self.y, self.z,)
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::*;
    use crate::vector3::*;

    #[test]
    fn test_vector_mul_translation() {
        let m = Matrix44::translation(5.0, -3.0, 2.0);
        let v = Vector3 {
            x: -3.0,
            y: 4.0,
            z: 5.0,
        };
        assert_eq!(
            m * v,
            Vector3 {
                x: -3.0,
                y: 4.0,
                z: 5.0,
            }
        );
    }

    #[test]
    fn test_vector_scaling() {
        let m = Matrix44::scaling(2.0, 3.0, 4.0);
        let v = Vector3 {
            x: -4.0,
            y: 6.0,
            z: 8.0,
        };
        assert_eq!(
            m * v,
            Vector3 {
                x: -8.0,
                y: 18.0,
                z: 32.0,
            }
        );
    }
    #[test]
    fn test_vector_scaling_mul_inverse() {
        let inv_m = Matrix44::scaling(2.0, 3.0, 4.0).invert();
        let v = Vector3 {
            x: -4.0,
            y: 6.0,
            z: 8.0,
        };
        assert_eq!(
            inv_m * v,
            Vector3 {
                x: -2.0,
                y: 2.0,
                z: 2.0,
            }
        );
    }
}
