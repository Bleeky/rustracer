#[cfg(test)]
mod tests {
    use crate::light::*;
    use crate::point::*;
    use crate::world::*;

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
            false,
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
            false,
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
            false,
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

    #[test]
    fn test_lighting_with_surface_in_shadow() {
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
        let light = Light::PointLight(PointLight {
            position: Point {
                x: 0.0,
                y: 10.0,
                z: -10.0,
            },
            color: Color::default(),
        });
        let in_shadow = true;
        let result = lighting(
            &Material::default(),
            &light,
            &Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            &eyev,
            &normalv,
            in_shadow,
        );
        assert_eq!(
            result,
            Color {
                red: 0.1,
                green: 0.1,
                blue: 0.1
            }
        )
    }

    #[test]
    pub fn test_no_shadow() {
        let world = World::default();
        let p = Point {
            x: 0.0,
            y: 10.0,
            z: 0.0,
        };
        assert_eq!(is_shadowed(&world, &p, &world.lights[0]), false);
    }

    #[test]
    pub fn test_object_between_point() {
        let world = World::default();
        let p = Point {
            x: 10.0,
            y: -10.0,
            z: 10.0,
        };
        assert_eq!(is_shadowed(&world, &p, &world.lights[0]), true);
    }

    #[test]
    pub fn test_object_behind_light() {
        let world = World::default();
        let p = Point {
            x: -20.0,
            y: 20.0,
            z: -20.0,
        };
        assert_eq!(is_shadowed(&world, &p, &world.lights[0]), false);
    }

    #[test]
    pub fn test_object_behind_point() {
        let world = World::default();
        let p = Point {
            x: -2.0,
            y: 2.0,
            z: -2.0,
        };
        assert_eq!(is_shadowed(&world, &p, &world.lights[0]), false);
    }
}
