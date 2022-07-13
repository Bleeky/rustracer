use crate::color::*;
use crate::light::*;
use crate::objects::*;
use crate::point::*;
use crate::ray::*;
use crate::vector3::*;
use crate::world::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Intersection {
    pub object: Object,
    pub distance: f64,
}

pub fn intersect_world<'a>(ray: &Ray, world: &'a World) -> Vec<Intersection> {
    let mut t: Vec<Intersection> = world
        .objects
        .iter()
        .filter_map(|object| object.intersect(ray))
        .flat_map(|intersections| intersections)
        .collect();
    t.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    t
}

pub struct Computations {
    pub distance: f64,
    pub point: Point,
    pub eyev: Vector3,
    pub reflectv: Vector3,
    pub normalv: Vector3,
    pub object: Object,
    pub inside: bool,
    pub over_point: Point,
    pub under_point: Point,
    pub n1: f32,
    pub n2: f32,
}

pub fn prepare_computations(
    intersection: &Intersection,
    ray: &Ray,
    intersections_collection: &Vec<Intersection>,
) -> Computations {
    let mut containers: Vec<Object> = vec![];
    let mut n1 = 1.0;
    let mut n2 = 1.0;
    for inter in intersections_collection {
        if inter == intersection {
            if containers.is_empty() {
                n1 = 1.0;
            } else {
                n1 = containers.last().unwrap().material().refractive_index;
            }
        }

        if containers.contains(&inter.object) {
            containers.retain(|obj| obj != &inter.object);
        } else {
            containers.push(inter.object.clone());
        }

        if inter == intersection {
            if containers.is_empty() {
                n2 = 1.0;
            } else {
                n2 = containers.last().unwrap().material().refractive_index;
            }
            break;
        }
    }

    let mut inside: bool = false;
    let point = ray.position(intersection.distance);
    let eyev = -ray.direction;
    let mut normalv = intersection.object.normal_at(&point);
    if normalv.dot(&eyev) < 0.0 {
        inside = true;
        normalv = -normalv;
    }
    let reflectv = ray.direction.reflect(&normalv);
    Computations {
        distance: intersection.distance,
        object: intersection.object.clone(),
        point,
        eyev,
        normalv,
        inside,
        reflectv,
        // over_point: point + normalv * f64::EPSILON,
        over_point: point + normalv * 1e-11,
        under_point: point - normalv * 1e-11,
        n1,
        n2,
    }
}

// Hit() retrieves an array of intersections from a ray and then return the closest one (above zero distance)
pub fn hit(intersections: Vec<Intersection>) -> Option<Intersection> {
    intersections
        .iter()
        .cloned()
        .filter(|intersection| intersection.distance.is_sign_positive())
        .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
}

pub fn shade_hit(world: &World, computations: &Computations, remaining: i32) -> Color {
    world.lights.iter().fold(Color::black(), |sum, light| {
        let lighting = lighting(
            computations.object.material(),
            &computations.object,
            light,
            &computations.point,
            &computations.eyev,
            &computations.normalv,
            is_shadowed(world, &computations.over_point, light),
        );
        let reflected_color = reflected_color(world, &computations, remaining);
        let refracted_color = refracted_color(world, computations, remaining);
        if computations.object.material().reflective > 0.0
            && computations.object.material().transparency > 0.0
        {
            let reflectance = schlick(&computations) as f32;
            return sum
                + lighting
                + reflected_color * reflectance
                + refracted_color * (1.0 - reflectance);
        }
        sum + lighting + reflected_color + refracted_color
    })
}

pub fn color_at(world: &World, ray: &Ray, remaining: i32) -> Color {
    let intersections = intersect_world(ray, world);
    match hit(intersections.to_vec()) {
        Some(intersection) => shade_hit(
            world,
            &prepare_computations(&intersection, ray, &intersections.to_vec()),
            remaining,
        ),
        None => Color::black(),
    }
}

pub fn reflected_color(world: &World, comps: &Computations, remaining: i32) -> Color {
    if remaining <= 0 {
        return Color::black();
    }
    if comps.object.material().reflective == 0.0 {
        return Color::black();
    }
    let reflected_ray = Ray {
        origin: comps.over_point,
        direction: comps.reflectv,
    };
    let color = color_at(world, &reflected_ray, remaining - 1);
    color * comps.object.material().reflective
}

pub fn refracted_color(world: &World, comps: &Computations, remaining: i32) -> Color {
    if comps.object.material().transparency == 0.0 || remaining <= 0 {
        return Color::black();
    }
    let n_ratio = (comps.n1 / comps.n2) as f64;
    let cos_i = comps.eyev.dot(&comps.normalv);
    let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));
    if sin2_t > 1.0 {
        return Color::black();
    }
    let cos_t = (1.0 - sin2_t).sqrt();
    let direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;
    let refract_ray = Ray {
        origin: comps.under_point,
        direction: direction,
    };
    color_at(world, &refract_ray, remaining - 1) * comps.object.material().transparency
}

pub fn schlick(comps: &Computations) -> f64 {
    let mut cos = comps.eyev.dot(&comps.normalv);
    if comps.n1 > comps.n2 {
        let n = (comps.n1 / comps.n2) as f64;
        let sin2_t = n.powi(2) * (1.0 - cos.powi(2));
        if sin2_t > 1.0 {
            return 1.0;
        }
        cos = (1.0 - sin2_t).sqrt();
    }
    let r0 = ((comps.n1 - comps.n2) / (comps.n1 + comps.n2)).powi(2) as f64;
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}

#[cfg(test)]
#[path = "./intersection_tests.rs"]
mod intersection_tests;
