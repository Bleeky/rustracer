use crate::color::*;
use crate::material::*;
use crate::point::Point;
use crate::vector3::*;

pub struct PointLight {
    pub position: Point,
    pub color: Color,
    pub intensity: f32,
}

pub enum Light {
    PointLight(PointLight),
}

impl Light {
    pub fn position(&self) -> Point {
        match self {
            Light::PointLight(light) => light.position,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Light::PointLight(light) => light.color,
        }
    }
}

pub fn lighting(
    material: &Material,
    light: &Light,
    hit_point: &Point,
    eye_vector: &Vector3,
    normal_vector: &Vector3,
) -> Color {
    let diffuse: Color;
    let specular: Color;
    let effective_color = material.color * light.color();
    let lightv = (light.position() - *hit_point).normalize();
    let ambient = effective_color * material.ambient;
    let light_dot_normal = lightv.dot(normal_vector);
    if light_dot_normal < 0.0 {
        diffuse = Color::black();
        specular = Color::black();
    } else {
        diffuse = effective_color * material.diffuse * light_dot_normal as f32;
        let reflectv = -lightv.reflect(normal_vector);
        let reflect_dot_eye = reflectv.dot(eye_vector);
        if reflect_dot_eye <= 0.0 {
            specular = Color::black();
        } else {
            let factor = reflect_dot_eye.powf(material.shininess as f64);
            specular = light.color() * material.specular * factor as f32;
        }
    }
    ambient + diffuse + specular
}

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
