use crate::color::*;
use crate::point::*;
use crate::scene::*;
use crate::vector3::*;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
    fn surface_normal(&self, hit_point: &Point) -> Vector3;
}

impl Ray {
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);
        let sensor_x =
            ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;

        Ray {
            origin: Point::zero(),
            direction: Vector3 {
                x: sensor_x,
                y: sensor_y,
                z: -1.0,
            }
            // we normalize the direction vector so we can use it as a unit vector for dot products
            .normalize(),
        }
    }
}

pub fn get_color(intersection: &Intersection, ray: &Ray, scene: &Scene) -> Color {
    let hit_point = ray.origin + (ray.direction * intersection.distance);
    let surface_normal = intersection.object.surface_normal(&hit_point);
    // let light_reflected = intersection.object.albedo() / std::f32::consts::PI;
    let light_reflected = 0.18 / std::f32::consts::PI;

    let mut color = Color {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    };
    for light in &scene.lights {
        let direction_to_light = light.direction_from(&hit_point);
        let shadow_ray = Ray {
            origin: hit_point + (surface_normal * scene.shadow_bias),
            direction: direction_to_light,
        };
        let shadow_intersection = scene.trace(&shadow_ray);
        let in_light = shadow_intersection.is_none()
            || shadow_intersection.unwrap().distance > light.distance(&hit_point);

        let light_intensity = if in_light {
            light.intensity(&hit_point)
        } else {
            0.0
        };
        let light_power =
            (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light_intensity;
        let light_color =
            intersection.object.color().clone() * light.color() * light_power * light_reflected;
        color = color + (*intersection.object.color() * light_color);
    }
    color
}
