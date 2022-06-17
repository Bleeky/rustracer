#[cfg(test)]
mod tests {
    use crate::matrix::*;
    #[test]
    fn test_determinant_2x2() {
        let m = Matrix22 {
            elements: [[1.0, 5.0], [-3.0, 2.0]],
        };
        assert_eq!(m.determinant(), 17.0);
    }

    #[test]
    fn test_identity_mul() {
        let m = Matrix44 {
            elements: [
                [0.0, 1.0, 2.0, 4.0],
                [1.0, 2.0, 4.0, 8.0],
                [2.0, 4.0, 8.0, 16.0],
                [4.0, 8.0, 16.0, 32.0],
            ],
        };
        assert_eq!(m * Matrix44::identity(), m);
    }

    #[test]
    fn test_determinant() {
        let m = Matrix33 {
            elements: [[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]],
        };
        assert_eq!(m.determinant(), -196.0);
        let m = Matrix44 {
            elements: [
                [-2.0, -8.0, 3.0, 5.0],
                [-3.0, 1.0, 7.0, 3.0],
                [1.0, 2.0, -9.0, 6.0],
                [-6.0, 7.0, 7.0, -9.0],
            ],
        };
        assert_eq!(m.determinant(), -4071.0);
    }

    #[test]
    fn test_inversion() {
        let m = Matrix44 {
            elements: [
                [-5.0, 2.0, 6.0, -8.0],
                [1.0, -5.0, 1.0, 8.0],
                [7.0, 7.0, -6.0, -7.0],
                [1.0, -3.0, 7.0, 4.0],
            ],
        };
        let expected = Matrix44 {
            elements: [
                [
                    0.21804511278195488,
                    0.45112781954887216,
                    0.24060150375939848,
                    -0.045112781954887216,
                ],
                [
                    -0.8082706766917294,
                    -1.4567669172932332,
                    -0.44360902255639095,
                    0.5206766917293233,
                ],
                [
                    -0.07894736842105263,
                    -0.2236842105263158,
                    -0.05263157894736842,
                    0.19736842105263158,
                ],
                [
                    -0.5225563909774437,
                    -0.8139097744360902,
                    -0.3007518796992481,
                    0.30639097744360905,
                ],
            ],
        };
        assert_eq!(m.invert(), expected);
    }

    #[test]
    fn test_mul_inverse() {
        let a = Matrix44 {
            elements: [
                [-5.0, 2.0, 6.0, -8.0],
                [1.0, -5.0, 1.0, 8.0],
                [7.0, 7.0, -6.0, -7.0],
                [1.0, -3.0, 7.0, 4.0],
            ],
        };
        let b = Matrix44 {
            elements: [
                [-2.0, -8.0, 3.0, 5.0],
                [-3.0, 1.0, 7.0, 3.0],
                [1.0, 2.0, -9.0, 6.0],
                [-6.0, 7.0, 7.0, -9.0],
            ],
        };
        let c = a * b;
        assert_eq!(c * b.invert(), a);
    }

    #[test]
    #[should_panic]
    fn test_non_invertible() {
        let a = Matrix44 {
            elements: [
                [-5.0, 2.0, 6.0, -8.0],
                [1.0, -5.0, 1.0, 8.0],
                [7.0, 7.0, -6.0, -7.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
        };
        a.invert();
    }

    #[test]
    fn test_transpose_identity() {
        let m = Matrix44::identity();
        assert_eq!(m.transpose(), Matrix44::identity());
    }

    #[test]
    fn test_scaling() {
        let m = Matrix44::scaling(2.0, 3.0, 4.0);
        let p = Point {
            x: -4.0,
            y: 6.0,
            z: 8.0,
        };
        assert_eq!(
            m * p,
            Point {
                x: -8.0,
                y: 18.0,
                z: 32.0,
            }
        );
    }

    #[test]
    fn chain_transformations() {
        let p = Point {
            x: 1.0,
            y: 0.0,
            z: 1.0,
        };
        let t = Matrix44::translation(10.0, 5.0, 7.0)
            * Matrix44::scaling(5.0, 5.0, 5.0)
            * Matrix44::rotation_x(std::f64::consts::FRAC_PI_2);

        assert_eq!(
            t * p,
            Point {
                x: 15.0,
                y: 0.0,
                z: 7.0,
            }
        );

        let t = Matrix44::identity()
            .rotate_x(std::f64::consts::FRAC_PI_2)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0);
        assert_eq!(
            t * p,
            Point {
                x: 15.0,
                y: 0.0,
                z: 7.0,
            }
        );
    }
}
