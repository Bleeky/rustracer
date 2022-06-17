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
#[path = "./point_tests.rs"]
mod point_tests;
