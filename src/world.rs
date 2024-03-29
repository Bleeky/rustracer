use crate::color::*;
use crate::light::*;
use crate::material::*;
use crate::matrix::*;
use crate::objects::sphere::*;
use crate::objects::*;
use crate::point::*;

pub struct World {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}

impl World {
    pub fn new(objects: Vec<Object>, lights: Vec<Light>) -> Self {
        World { objects, lights }
    }
}

impl Default for World {
    fn default() -> Self {
        let sphere1 = Sphere::new(&Material {
            color: Color {
                red: 0.8,
                green: 1.0,
                blue: 0.6,
            },
            diffuse: 0.7,
            specular: 0.2,
            ..Material::default()
        });
        let mut sphere2 = Sphere::new(&Material {
            color: Color::white(),
            ..Material::default()
        });
        sphere2.set_transform(Matrix44::scaling(0.5, 0.5, 0.5));
        World {
            objects: vec![Object::Sphere(sphere1), Object::Sphere(sphere2)],
            lights: vec![Light::PointLight(PointLight {
                position: Point {
                    x: -10.0,
                    y: 10.0,
                    z: -10.0,
                },
                color: Color::white(),
            })],
        }
    }
}
