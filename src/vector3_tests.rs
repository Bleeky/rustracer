#[cfg(test)]
mod tests {
    use crate::matrix::*;
    use crate::point::*;
    use crate::vector3::*;

    #[test]
    fn test_vec_negate() {
        let v = Vector3 {
            x: 1.0,
            y: -2.0,
            z: 3.0,
        };
        assert_eq!(
            -v,
            Vector3 {
                x: -1.0,
                y: 2.0,
                z: -3.0,
            }
        );
    }

    #[test]
    fn test_vec_mul_scalar() {
        let v = Vector3 {
            x: 1.0,
            y: -2.0,
            z: 3.0,
        };
        assert_eq!(
            v * 3.5,
            Vector3 {
                x: 3.5,
                y: -7.0,
                z: 10.5,
            }
        );
    }
    #[test]
    fn test_vec_len() {
        let v = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(v.length(), 14.0_f64.sqrt());
    }
    #[test]
    fn test_vec_len_2() {
        let v = Vector3 {
            x: -1.0,
            y: -2.0,
            z: -3.0,
        };
        assert_eq!(v.length(), 14.0_f64.sqrt());
    }
    #[test]
    fn test_vec_normalize_1() {
        let v = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(
            v.normalize(),
            Vector3 {
                x: 1.0 / 14.0_f64.sqrt(),
                y: 2.0 / 14.0_f64.sqrt(),
                z: 3.0 / 14.0_f64.sqrt(),
            }
        );
    }
    #[test]
    fn test_vec_normalize_2() {
        let v = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let norm = v.normalize();
        assert_eq!(norm.length(), 1.0);
    }

    #[test]
    fn test_vec_cross_product() {
        let a = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = Vector3 {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };
        assert_eq!(
            a.cross(&b),
            Vector3 {
                x: -1.0,
                y: 2.0,
                z: -1.0,
            }
        );
        assert_eq!(
            b.cross(&a),
            Vector3 {
                x: 1.0,
                y: -2.0,
                z: 1.0,
            }
        );
    }

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
    #[test]
    fn test_vector_reflecting() {
        let v = Vector3 {
            x: 1.0,
            y: -1.0,
            z: 0.0,
        };
        let n = Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        assert_eq!(
            v.reflect(&n),
            Vector3 {
                x: 1.0,
                y: 1.0,
                z: 0.0,
            }
        );
    }
    #[test]
    fn test_vector_reflecting_off_slanted() {
        let v = Vector3 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        };
        let n = Vector3 {
            x: 2.0_f64.sqrt() / 2.0,
            y: 2.0_f64.sqrt() / 2.0,
            z: 0.0,
        };
        assert_eq!(
            v.reflect(&n),
            Vector3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            }
        );
    }
}
