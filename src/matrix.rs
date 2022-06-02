use std::ops::{Add, Mul};

pub struct Matrix44 {
    elements: [[f64; 4]; 4],
}
pub struct Matrix22 {
    elements: [[f64; 2]; 2],
}
pub struct Matrix33 {
    elements: [[f64; 3]; 3],
}

impl Matrix44 {
    pub fn identity() -> Matrix44 {
        Matrix44 {
            elements: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}

impl Mul for Matrix44 {
    type Output = Matrix44;

    fn mul(self, other: Matrix44) -> Matrix44 {
        let mut result = Matrix44 {
            elements: [[0.0; 4]; 4],
        };
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = self[i][0] * other[0][j]
                    + self[i][1] * other[1][j]
                    + self[i][2] * other[2][j]
                    + self[i][3] * other[3][j];
            }
        }
        result
    }
}

impl Mul<Vector3> for Matrix44 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: other.x * self[0][0] + other.y * self[1][0] + other.z * self[2][0],
            y: other.x * self[0][1] + other.y * self[1][1] + other.z * self[2][1],
            z: other.x * self[0][2] + other.y * self[1][2] + other.z * self[2][2],
        }
    }
}
