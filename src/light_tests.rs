#[cfg(test)]
mod tests {
    use crate::light::*;
    use crate::point::*;

    #[test]
    fn test_lighting_1() {
        let eyev = Vector3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        let normalv = Vector3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        let plight = Light::PointLight(PointLight {
            position: Point {
                x: 0.0,
                y: 0.0,
                z: -10.0,
            },
            color: Color::default(),
            intensity: 1.0,
        });
        let res = lighting(
            &Material::default(),
            &plight,
            &Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            &eyev,
            &normalv,
        );
        assert_eq!(
            res,
            Color {
                red: 1.9,
                green: 1.9,
                blue: 1.9,
            }
        );
    }

    #[test]
    fn test_lighting_2() {
        let eyev = Vector3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        let normalv = Vector3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        let plight = Light::PointLight(PointLight {
            position: Point {
                x: 0.0,
                y: 0.0,
                z: 10.0,
            },
            color: Color::default(),
            intensity: 1.0,
        });
        let res = lighting(
            &Material::default(),
            &plight,
            &Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            &eyev,
            &normalv,
        );
        assert_eq!(
            res,
            Color {
                red: 0.1,
                green: 0.1,
                blue: 0.1,
            }
        );
    }

    #[test]
    fn test_lighting_3() {
        let eyev = Vector3 {
            x: 0.0,
            y: -((2.0 as f64).sqrt() / 2.0),
            z: -((2.0 as f64).sqrt() / 2.0),
        };
        let normalv = Vector3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        let plight = Light::PointLight(PointLight {
            position: Point {
                x: 0.0,
                y: 10.0,
                z: -10.0,
            },
            color: Color::default(),
            intensity: 1.0,
        });
        let res = lighting(
            &Material::default(),
            &plight,
            &Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            &eyev,
            &normalv,
        );
        assert_eq!(
            res,
            Color {
                red: 1.636396,
                green: 1.636396,
                blue: 1.636396,
            }
        );
    }
}
