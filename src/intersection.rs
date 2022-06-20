use crate::color::*;
use crate::light::lighting;
use crate::objects::*;
use crate::point::*;
use crate::ray::*;
use crate::vector3::*;
use crate::world::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Intersection<'a> {
    pub object: &'a Object,
    pub distance: f64,
}

pub fn intersect_world<'a>(ray: &Ray, world: &'a World) -> Vec<Intersection<'a>> {
    let mut t: Vec<Intersection> = world
        .objects
        .iter()
        .filter_map(|s| {
            s.intersect(ray).map(|d| {
                (
                    Intersection {
                        object: s,
                        distance: d.0,
                    },
                    Intersection {
                        object: s,
                        distance: d.1,
                    },
                )
            })
        })
        .flat_map(|x| vec![x.0, x.1])
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
    }
}

// Hit() retrieves an array of intersections from a ray and then return the closest one (above zero distance)
pub fn hit<'a>(intersections: &[Intersection<'a>]) -> Option<Intersection<'a>> {
    intersections
        .iter()
        .cloned()
        .filter(|x| x.distance.is_sign_positive())
        .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
}

pub fn shade_hit(world: &World, computations: &Computations) -> Color {
    world.lights.iter().fold(
        Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        },
        |acc, x| {
            acc + lighting(
                computations.object.material(),
                &x,
                &computations.point,
                &computations.eyev,
                &computations.normalv,
            )
        },
    )
}

pub fn color_at(world: &World, ray: &Ray) -> Color {
    match hit(&intersect_world(ray, world)) {
        Some(x) => shade_hit(world, &prepare_computations(&x, ray)),
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
