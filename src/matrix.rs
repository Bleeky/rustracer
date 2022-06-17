use crate::point::*;
use crate::vector3::*;
use std::fmt::Display;
use std::ops::Mul;

#[derive(Debug, Default, Clone, Copy)]
pub struct Matrix44 {
    pub elements: [[f64; 4]; 4],
}
pub struct Matrix33 {
    pub elements: [[f64; 3]; 3],
}
pub struct Matrix22 {
    pub elements: [[f64; 2]; 2],
}

impl Matrix22 {
    pub fn determinant(&self) -> f64 {
        self.elements[0][0] * self.elements[1][1] - self.elements[0][1] * self.elements[1][0]
    }
}

impl Matrix33 {
    pub fn identity() -> Matrix33 {
        Matrix33 {
            elements: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let mut matrix22 = Matrix22 {
            elements: [[0.0; 2]; 2],
        };
        let mut new_row = 0;
        for i in 0..=2 {
            if i != row {
                let mut new_col = 0;
                for j in 0..=2 {
                    if j != col {
                        matrix22.elements[new_row][new_col] = self.elements[i][j];
                        new_col += 1;
                    }
                }
                new_row += 1;
            }
        }
        let a = if (row + col) % 2 == 0 { 1.0 } else { -1.0 };
        matrix22.determinant() * a
    }

    pub fn determinant(&self) -> f64 {
        let mut det = 0.0;
        for i in 0..=2 {
            det += self.elements[0][i] * self.cofactor(0, i);
        }
        det
    }
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

    pub fn translation(tx: f64, ty: f64, tz: f64) -> Matrix44 {
        Matrix44 {
            elements: [
                [1.0, 0.0, 0.0, tx],
                [0.0, 1.0, 0.0, ty],
                [0.0, 0.0, 1.0, tz],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn translate(self, tx: f64, ty: f64, tz: f64) -> Self {
        Matrix44::translation(tx, ty, tz) * self
    }

    pub fn scaling(tx: f64, ty: f64, tz: f64) -> Matrix44 {
        Matrix44 {
            elements: [
                [tx, 0.0, 0.0, 0.0],
                [0.0, ty, 0.0, 0.0],
                [0.0, 0.0, tz, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn scale(self, tx: f64, ty: f64, tz: f64) -> Self {
        Matrix44::scaling(tx, ty, tz) * self
    }

    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix44 {
        Matrix44 {
            elements: [
                [1.0, xy, xz, 0.0],
                [yx, 1.0, yz, 0.0],
                [zx, zy, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn shear(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        Matrix44::shearing(xy, xz, yx, yz, zx, zy) * self
    }

    pub fn rotation_x(rad: f64) -> Matrix44 {
        let sin = rad.sin();
        let cos = rad.cos();
        Matrix44 {
            elements: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, cos, -sin, 0.0],
                [0.0, sin, cos, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotate_x(self, rad: f64) -> Self {
        Matrix44::rotation_x(rad) * self
    }

    pub fn rotation_y(rad: f64) -> Matrix44 {
        let sin = rad.sin();
        let cos = rad.cos();
        Matrix44 {
            elements: [
                [cos, 0.0, sin, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-sin, 0.0, cos, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotate_y(self, rad: f64) -> Self {
        Matrix44::rotation_y(rad) * self
    }

    pub fn rotation_z(rad: f64) -> Matrix44 {
        let sin = rad.sin();
        let cos = rad.cos();
        Matrix44 {
            elements: [
                [cos, -sin, 0.0, 0.0],
                [sin, cos, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotate_z(self, rad: f64) -> Self {
        Matrix44::rotation_z(rad) * self
    }

    pub fn transpose(&self) -> Matrix44 {
        Matrix44 {
            elements: [
                [
                    self.elements[0][0],
                    self.elements[1][0],
                    self.elements[2][0],
                    self.elements[3][0],
                ],
                [
                    self.elements[0][1],
                    self.elements[1][1],
                    self.elements[2][1],
                    self.elements[3][1],
                ],
                [
                    self.elements[0][2],
                    self.elements[1][2],
                    self.elements[2][2],
                    self.elements[3][2],
                ],
                [
                    self.elements[0][3],
                    self.elements[1][3],
                    self.elements[2][3],
                    self.elements[3][3],
                ],
            ],
        }
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let mut matrix33 = Matrix33 {
            elements: [[0.0; 3]; 3],
        };
        let mut new_row = 0;
        for i in 0..=3 {
            if i != row {
                let mut new_col = 0;
                for j in 0..=3 {
                    if j != col {
                        matrix33.elements[new_row][new_col] = self.elements[i][j];
                        new_col += 1;
                    }
                }
                new_row += 1;
            }
        }
        let a = if (row + col) % 2 == 0 { 1.0 } else { -1.0 };
        matrix33.determinant() * a
    }

    pub fn determinant(&self) -> f64 {
        let mut det = 0.0;
        for i in 0..=3 {
            det += self.elements[0][i] * self.cofactor(0, i);
        }
        det
    }

    pub fn invert(&self) -> Matrix44 {
        let mut matrix44 = Matrix44 {
            elements: [[0.0; 4]; 4],
        };
        let det = self.determinant();
        if det == 0.0 {
            panic!("Matrix is not invertible");
        }
        for i in 0..=3 {
            for j in 0..=3 {
                matrix44.elements[i][j] = self.cofactor(i, j) / det;
            }
        }
        matrix44.transpose()
    }
}

impl Mul for Matrix44 {
    type Output = Matrix44;

    fn mul(self, other: Matrix44) -> Matrix44 {
        let mut result = Matrix44 {
            elements: [[0.0; 4]; 4],
        };
        for i in 0..=3 {
            for j in 0..=3 {
                result.elements[i][j] = self.elements[i][0] * other.elements[0][j]
                    + self.elements[i][1] * other.elements[1][j]
                    + self.elements[i][2] * other.elements[2][j]
                    + self.elements[i][3] * other.elements[3][j];
            }
        }
        result
    }
}

impl Mul<Vector3> for Matrix44 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: (other.x * self.elements[0][0]
                + other.y * self.elements[1][0]
                + other.z * self.elements[2][0]),
            y: other.x * self.elements[0][1]
                + other.y * self.elements[1][1]
                + other.z * self.elements[2][1],
            z: other.x * self.elements[0][2]
                + other.y * self.elements[1][2]
                + other.z * self.elements[2][2],
        }
    }
}

impl Mul<Point> for Matrix44 {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point {
            x: other.x * self.elements[0][0]
                + other.y * self.elements[0][1]
                + other.z * self.elements[0][2]
                + self.elements[0][3],
            y: other.x * self.elements[1][0]
                + other.y * self.elements[1][1]
                + other.z * self.elements[1][2]
                + self.elements[1][3],
            z: other.x * self.elements[2][0]
                + other.y * self.elements[2][1]
                + other.z * self.elements[2][2]
                + self.elements[2][3],
        }
    }
}

impl PartialEq for Matrix44 {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..=3 {
            for j in 0..=3 {
                if (self.elements[i][j] - other.elements[i][j]).abs() > 1e-15 {
                    return false;
                }
            }
        }
        true
    }
}

impl Display for Matrix44 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(
            fmt,
            "Matrix44\n[\n{},{},{},{}\n{},{},{},{}\n{},{},{},{}\n{},{},{},{}\n]",
            self.elements[0][0],
            self.elements[0][1],
            self.elements[0][2],
            self.elements[0][3],
            self.elements[1][0],
            self.elements[1][1],
            self.elements[1][2],
            self.elements[1][3],
            self.elements[2][0],
            self.elements[2][1],
            self.elements[2][2],
            self.elements[2][3],
            self.elements[3][0],
            self.elements[3][1],
            self.elements[3][2],
            self.elements[3][3],
        )
    }
}

#[cfg(test)]
#[path = "./matrix_tests.rs"]
mod matrix_tests;
