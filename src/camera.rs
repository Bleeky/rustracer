use crate::matrix::*;
use crate::point::*;
use crate::ray::*;

pub struct Camera {
    hsize: u32,
    vsize: u32,
    pixel_size: f64,
    field_of_view: f64,
    pub transform: Matrix44,
    half_width: f64,
    half_height: f64,
}

impl Camera {
    pub fn new(hsize: u32, vsize: u32, field_of_view: f64) -> Camera {
        let half_width;
        let half_height;
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;
        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }
        let pixel_size = (half_width * 2.0) / hsize as f64;
        Camera {
            hsize,
            vsize,
            pixel_size,
            half_width,
            half_height,
            field_of_view,
            transform: Matrix44::identity(),
        }
    }

    pub fn ray_for_pixel(&self, x: u32, y: u32) -> Ray {
        let xoffset = (x as f64 + 0.5) * self.pixel_size;
        let yoffset = (y as f64 + 0.5) * self.pixel_size;
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;
        let pixel = self.transform.invert()
            * Point {
                x: world_x,
                y: world_y,
                z: -1.0,
            };
        let origin = self.transform.invert()
            * Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        let direction = (pixel - origin).normalize();
        Ray { origin, direction }
    }
}
#[cfg(test)]
#[path = "./camera_tests.rs"]
mod camera_tests;
