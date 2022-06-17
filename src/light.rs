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
#[path = "./light_tests.rs"]
mod light_tests;
