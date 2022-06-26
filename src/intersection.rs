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
    pub normalv: Vector3,
    pub object: Object,
    pub inside: bool,
    pub over_point: Point,
}

pub fn prepare_computations(intersection: &Intersection, ray: &Ray) -> Computations {
    let mut inside: bool = false;
    let point = ray.position(intersection.distance);
    let eyev = -ray.direction;
    let mut normalv = intersection.object.normal_at(&point);
    if normalv.dot(&eyev) < 0.0 {
        inside = true;
        normalv = -normalv;
    }
    Computations {
        distance: intersection.distance,
        object: intersection.object.clone(),
        point,
        eyev,
        normalv,
        inside,
        // over_point: point + normalv * f64::EPSILON,
        over_point: point + normalv * 1e-11,
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

pub fn shade_hit(world: &World, computations: &Computations, hit_object: &Object) -> Color {
    world.lights.iter().fold(Color::black(), |sum, light| {
        sum + lighting(
            computations.object.material(),
            hit_object,
            light,
            &computations.point,
            &computations.eyev,
            &computations.normalv,
            is_shadowed(world, &computations.over_point, light),
        )
    })
}

pub fn color_at(world: &World, ray: &Ray) -> Color {
    match hit(intersect_world(ray, world)) {
        Some(intersection) => shade_hit(
            world,
            &prepare_computations(&intersection, ray),
            &intersection.object,
        ),
        None => Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        },
    }
}

#[cfg(test)]
#[path = "./intersection_tests.rs"]
mod intersection_tests;
