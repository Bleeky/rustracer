#[cfg(test)]
mod tests {
    use crate::light::*;
    use crate::objects::sphere::*;
    use crate::patterns::stripe::*;
    use crate::patterns::*;
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
        });
        let sphere = Object::Sphere(Sphere::new(Material::default()));
        let res = lighting(
            &Material::default(),
            &sphere,
            &plight,
            &Point::zero(),
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
        let sphere = Object::Sphere(Sphere::new(Material::default()));
        let res = lighting(
            &Material::default(),
            &sphere,
            &plight,
            &Point::zero(),
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
        let sphere = Object::Sphere(Sphere::new(Material::default()));
        let res = lighting(
            &Material::default(),
            &sphere,
            &plight,
            &Point::zero(),
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
        let sphere = Object::Sphere(Sphere::new(Material::default()));
        let result = lighting(
            &Material::default(),
            &sphere,
            &light,
            &Point::zero(),
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

    #[test]
    pub fn lighting_with_pattern() {
        let mut m = Material::default();
        m.ambient = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;
        m.pattern = Some(Pattern::Stripe(Stripe::new(Color::white(), Color::black())));
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
                y: 0.0,
                z: -10.0,
            },
            color: Color::white(),
        });
        let sphere = Object::Sphere(Sphere::new(Material::default()));
        let c1 = lighting(
            &m,
            &sphere,
            &light,
            &Point {
                x: 0.9,
                y: 0.0,
                z: 0.0,
            },
            &eyev,
            &normalv,
            false,
        );
        let c2 = lighting(
            &m,
            &sphere,
            &light,
            &Point {
                x: 1.1,
                y: 0.0,
                z: 0.0,
            },
            &eyev,
            &normalv,
            false,
        );
        assert_eq!(c1, Color::white());
        assert_eq!(c2, Color::black());
    }
}
