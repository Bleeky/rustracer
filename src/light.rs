use crate::color::*;
use crate::intersection::hit;
use crate::intersection::intersect_world;
use crate::material::*;
use crate::point::Point;
use crate::ray::*;
use crate::vector3::*;
use crate::world::*;

pub struct PointLight {
    pub position: Point,
    pub color: Color,
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
    in_shadow: bool,
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
    if in_shadow {
        return ambient;
    }
    ambient + diffuse + specular
}

pub fn is_shadowed(world: &World, point: &Point, light: &Light) -> bool {
    let v = light.position() - *point;
    let distance = v.length();
    let direction = v.normalize();
    let ray = Ray {
        origin: point.clone(),
        direction,
    };
    let intersections = intersect_world(&ray, world);
    match hit(intersections) {
        Some(x) => {
            if x.distance < distance {
                return true;
            }
            false
        }
        None => false,
    }
}
// pub fn is_shadowed(world: &World, point: &Point) -> bool {
//     world.lights.iter().any(|x| {
//         let v = x.position() - *point;
//         let distance = v.length();
//         let direction = v.normalize();
//         let ray = Ray {
//             origin: point.clone(),
//             direction,
//         };
//         let intersections = intersect_world(&ray, world);
//         match hit(&intersections) {
//             Some(x) => {
//                 if x.distance < distance {
//                     return true;
//                 }
//                 false
//             }
//             None => false,
//         }
//     })
// }

#[cfg(test)]
#[path = "./light_tests.rs"]
mod light_tests;
