use crate::matrix::*;
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
            return Vector3 {
                x: self.x / length,
                y: self.y / length,
                z: self.z / length,
            };
        }
        *self
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn reflect(&self, normal: &Vector3) -> Vector3 {
        *self - *normal * 2.0 * self.dot(normal)
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
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

impl Mul<Matrix44> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: Matrix44) -> Vector3 {
        Vector3 {
            x: self.x * other.elements[0][0]
                + self.y * other.elements[0][1]
                + self.z * other.elements[0][2],
            y: self.x * other.elements[1][0]
                + self.y * other.elements[1][1]
                + self.z * other.elements[1][2],
            z: self.x * other.elements[2][0]
                + self.y * other.elements[2][1]
                + self.z * other.elements[2][2],
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
        (self.x - other.x).abs() < 1e-15
            && (self.y - other.y).abs() < 1e-15
            && (self.z - other.z).abs() < 1e-15
    }
}

impl Display for Vector3 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "Vector3\n[{},{},{}]", self.x, self.y, self.z,)
    }
}

#[cfg(test)]
#[path = "./vector3_tests.rs"]
mod vector3_tests;
