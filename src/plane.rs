use crate::color::*;
use crate::point::Point;
use crate::ray::*;
use crate::vector3::*;

pub struct Plane {
    pub origin: Point,
    pub normal: Vector3,
    pub color: Color,
}
