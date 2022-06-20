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
    fn test_matrix_mul() {
        let a = Matrix44 {
            elements: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0],
            ],
        };
        let b = Matrix44 {
            elements: [
                [-2.0, 1.0, 2.0, 3.0],
                [3.0, 2.0, 1.0, -1.0],
                [4.0, 3.0, 6.0, 5.0],
                [1.0, 2.0, 7.0, 8.0],
            ],
        };
        assert_eq!(
            a * b,
            Matrix44 {
                elements: [
                    [20.0, 22.0, 50.0, 48.0],
                    [44.0, 54.0, 114.0, 108.0],
                    [40.0, 58.0, 110.0, 102.0],
                    [16.0, 26.0, 46.0, 42.0],
                ],
            }
        );
    }

    #[test]
    fn test_matrix_mul_point() {
        let a = Matrix44 {
            elements: [
                [1.0, 2.0, 3.0, 4.0],
                [2.0, 4.0, 4.0, 2.0],
                [8.0, 6.0, 4.0, 1.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        let b = Point {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(
            a * b,
            Point {
                x: 18.0,
                y: 24.0,
                z: 33.0,
            }
        );
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
    fn test_determinant_2() {
        let m = Matrix44 {
            elements: [
                [6.0, 4.0, 4.0, 4.0],
                [5.0, 5.0, 7.0, 6.0],
                [4.0, -9.0, 3.0, -7.0],
                [9.0, 1.0, 7.0, -6.0],
            ],
        };
        assert_eq!(m.determinant(), -2120.0);
    }

    #[test]
    fn test_determinant_3() {
        let m = Matrix44 {
            elements: [
                [-4.0, 2.0, -2.0, -3.0],
                [9.0, 6.0, 2.0, 6.0],
                [0.0, -5.0, 1.0, -5.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
        };
        assert_eq!(m.determinant(), 0.0);
    }

    #[test]
    fn test_cofactor() {
        let a = Matrix44 {
            elements: [
                [-5.0, 2.0, 6.0, -8.0],
                [1.0, -5.0, 1.0, 8.0],
                [7.0, 7.0, -6.0, -7.0],
                [1.0, -3.0, 7.0, 4.0],
            ],
        };
        let b = a.invert();
        assert_eq!(a.determinant(), 532.0);
        assert_eq!(a.cofactor(2, 3), -160.0);
        assert_eq!(b.elements[3][2], -160.0 / 532.0);
        assert_eq!(a.cofactor(3, 2), 105.0);
        assert_eq!(b.elements[2][3], 105.0 / 532.0);
        assert_eq!(
            b,
            Matrix44 {
                elements: [
                    [
                        0.21804511278195488,
                        0.45112781954887216,
                        0.24060150375939848,
                        -0.045112781954887216
                    ],
                    [
                        -0.8082706766917294,
                        -1.4567669172932332,
                        -0.44360902255639095,
                        0.5206766917293233
                    ],
                    [
                        -0.07894736842105263,
                        -0.2236842105263158,
                        -0.05263157894736842,
                        0.19736842105263158
                    ],
                    [
                        -0.5225563909774437,
                        -0.8139097744360902,
                        -0.3007518796992481,
                        0.30639097744360905
                    ]
                ]
            }
        );
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
    fn test_inversion_2() {
        let m = Matrix44 {
            elements: [
                [8.0, -5.0, 9.0, 2.0],
                [7.0, 5.0, 6.0, 1.0],
                [-6.0, 0.0, 9.0, 6.0],
                [-3.0, 0.0, -9.0, -4.0],
            ],
        };
        let expected = Matrix44 {
            elements: [
                [
                    -0.15384615384615385,
                    -0.15384615384615385,
                    -0.28205128205128205,
                    -0.5384615384615384,
                ],
                [
                    -0.07692307692307693,
                    0.12307692307692308,
                    0.02564102564102564,
                    0.03076923076923077,
                ],
                [
                    0.358974358974359,
                    0.358974358974359,
                    0.4358974358974359,
                    0.9230769230769231,
                ],
                [
                    -0.6923076923076923,
                    -0.6923076923076923,
                    -0.7692307692307693,
                    -1.9230769230769231,
                ],
            ],
        };
        assert_eq!(m.invert(), expected);
    }
    #[test]
    fn test_inversion_3() {
        let m = Matrix44 {
            elements: [
                [9.0, 3.0, 0.0, 9.0],
                [-5.0, -2.0, -6.0, -3.0],
                [-4.0, 9.0, 6.0, 4.0],
                [-7.0, 6.0, 6.0, 2.0],
            ],
        };
        let expected = Matrix44 {
            elements: [
                [
                    -0.040740740740740744,
                    -0.07777777777777778,
                    0.14444444444444443,
                    -0.2222222222222222,
                ],
                [
                    -0.07777777777777778,
                    0.03333333333333333,
                    0.36666666666666664,
                    -0.3333333333333333,
                ],
                [
                    -0.029012345679012345,
                    -0.14629629629629629,
                    -0.10925925925925926,
                    0.12962962962962962,
                ],
                [
                    0.17777777777777778,
                    0.06666666666666667,
                    -0.26666666666666666,
                    0.3333333333333333,
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
    fn test_mul_inverse_2() {
        let a = Matrix44 {
            elements: [
                [3.0, -9.0, 7.0, 3.0],
                [3.0, -8.0, 2.0, -9.0],
                [-4.0, 4.0, 4.0, 1.0],
                [-6.0, 5.0, -1.0, 1.0],
            ],
        };
        let b = Matrix44 {
            elements: [
                [8.0, 2.0, 2.0, 2.0],
                [3.0, -1.0, 7.0, 0.0],
                [7.0, 0.0, 5.0, 4.0],
                [6.0, -2.0, 0.0, 5.0],
            ],
        };
        let c = a * b;
        assert_eq!(a, c * b.invert());
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
    fn test_transpose() {
        let m = Matrix44 {
            elements: [
                [0.0, 9.0, 3.0, 0.0],
                [9.0, 8.0, 0.0, 8.0],
                [1.0, 8.0, 5.0, 3.0],
                [0.0, 0.0, 5.0, 8.0],
            ],
        };
        assert_eq!(
            m.transpose(),
            Matrix44 {
                elements: [
                    [0.0, 9.0, 1.0, 0.0],
                    [9.0, 8.0, 8.0, 0.0],
                    [3.0, 0.0, 5.0, 5.0],
                    [0.0, 8.0, 3.0, 8.0]
                ]
            }
        );
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
    fn test_transformations_sequence() {
        let p = Point {
            x: 1.0,
            y: 0.0,
            z: 1.0,
        };
        let a = Matrix44::rotation_x(std::f64::consts::PI / 2.0);
        let b = Matrix44::scaling(5.0, 5.0, 5.0);
        let c = Matrix44::translation(10.0, 5.0, 7.0);
        let p2 = a * p;
        assert_eq!(
            p2,
            Point {
                x: 1.0,
                y: -1.0,
                z: 0.0,
            }
        );
        let p3 = b * p2;
        assert_eq!(
            p3,
            Point {
                x: 5.0,
                y: -5.0,
                z: 0.0,
            }
        );
        let p4 = c * p3;
        assert_eq!(
            p4,
            Point {
                x: 15.0,
                y: 0.0,
                z: 7.0,
            }
        );
    }

    #[test]
    fn test_chain_transformations() {
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
            * Matrix44::translation(10.0, 5.0, 7.0)
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

    #[test]
    fn test_transformation_default_orientation() {
        let from = Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let to = Point {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        let up = Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        assert_eq!(view_transform(from, to, up), Matrix44::identity());
    }

    #[test]
    fn test_transformation_move_world() {
        let from = Point {
            x: 0.0,
            y: 0.0,
            z: 8.0,
        };
        let to = Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let up = Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        assert_eq!(
            view_transform(from, to, up),
            Matrix44::translation(0.0, 0.0, -8.0)
        );
    }

    #[test]
    fn test_transformation_arbitrary() {
        let from = Point {
            x: 1.0,
            y: 3.0,
            z: 2.0,
        };
        let to = Point {
            x: 4.0,
            y: -2.0,
            z: 8.0,
        };
        let up = Vector3 {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        };
        assert_eq!(
            view_transform(from, to, up),
            Matrix44 {
                elements: [
                    [
                        -0.50709255283711,
                        0.50709255283711,
                        0.6761234037828132,
                        -2.3664319132398464
                    ],
                    [
                        0.7677159338596802,
                        0.6060915267313265,
                        0.12121830534626529,
                        -2.8284271247461903
                    ],
                    [
                        -0.3585685828003181,
                        0.5976143046671968,
                        -0.7171371656006362,
                        0.0
                    ],
                    [0.0, 0.0, 0.0, 1.0],
                ]
            }
        );
    }
}
