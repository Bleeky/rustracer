// use image::{DynamicImage, GenericImage};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
mod camera;
mod color;
mod intersection;
mod light;
mod material;
mod matrix;
mod objects;
mod point;
mod ray;
mod vector3;
mod world;

use crate::camera::*;
use crate::color::*;
use crate::intersection::*;
use crate::light::*;
use crate::material::*;
use crate::matrix::*;
use crate::objects::sphere::*;
use crate::objects::*;
use crate::point::*;
use crate::ray::*;
use crate::vector3::*;
use crate::world::*;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 800;

fn main() -> Result<(), Error> {
    let mut input = WinitInputHelper::new();
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Rustracer")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;
    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            draw(pixels.get_frame());
            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
        if input.update(&event) {
            if input.key_released(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            // window.request_redraw();
        }
    });
}

fn draw(frame: &mut [u8]) {
    let floor_material = Material {
        specular: 0.0,
        color: Color {
            red: 1.0,
            green: 0.9,
            blue: 0.9,
        },
        ..Material::default()
    };
    let mut floor = Object::Sphere(Sphere::new(floor_material));
    floor.set_transform(Matrix44::scaling(10.0, 0.01, 10.0));
    let mut right_wall = Object::Sphere(Sphere::new(floor_material));
    right_wall.set_transform(
        Matrix44::scaling(10.0, 0.01, 10.0)
            .rotate_x(std::f64::consts::FRAC_PI_2)
            .rotate_y(std::f64::consts::FRAC_PI_4)
            .translate(0.0, 0.0, 5.0),
    );
    let mut left_wall = Object::Sphere(Sphere::new(floor_material));
    left_wall.set_transform(
        Matrix44::scaling(10.0, 0.01, 10.0)
            .rotate_x(std::f64::consts::FRAC_PI_2)
            .rotate_y(-std::f64::consts::FRAC_PI_4)
            .translate(0.0, 0.0, 5.0),
    );
    let mut middlesphere = Object::Sphere(Sphere::new(Material {
        color: Color {
            red: 0.1,
            green: 1.0,
            blue: 0.5,
        },
        diffuse: 0.7,
        specular: 0.3,
        ..Material::default()
    }));
    middlesphere.set_transform(Matrix44::translation(-0.5, 1.0, 0.5));
    // middlesphere.set_transform(Matrix44::rotation_z(std::f64::consts::FRAC_PI_2));
    let mut rightsphere = Object::Sphere(Sphere::new(Material {
        color: Color {
            red: 0.5,
            green: 1.0,
            blue: 0.1,
        },
        diffuse: 0.7,
        specular: 0.3,
        ..Material::default()
    }));
    rightsphere.set_transform(Matrix44::scaling(0.5, 0.5, 0.5).translate(1.5, 0.5, -0.5));
    let mut leftsphere = Object::Sphere(Sphere::new(Material {
        color: Color {
            red: 1.0,
            green: 0.8,
            blue: 0.1,
        },
        diffuse: 0.7,
        specular: 0.3,
        ..Material::default()
    }));
    leftsphere.set_transform(Matrix44::scaling(0.33, 0.33, 0.33).translate(-1.5, 0.33, -0.75));
    let world = World {
        objects: vec![
            right_wall,
            // left_wall,
            // floor,
            // middlesphere,
            // rightsphere,
            // leftsphere,
        ],
        lights: vec![Light::PointLight(PointLight {
            position: Point {
                x: -10.0,
                y: 10.0,
                z: -10.0,
            },
            intensity: 1.0,
            color: Color {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
            },
        })],
    };
    let mut cam = Camera::new(WIDTH, HEIGHT, std::f64::consts::PI / 3.0);
    cam.transform = view_transform(
        Point {
            x: 0.0,
            y: 1.5,
            // y: 0.0,
            z: -5.0,
        },
        Point {
            x: 0.0,
            y: 1.0,
            // y: 0.0,
            z: 0.0,
        },
        Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
    );

    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = (i % WIDTH as usize) as u32;
        let y = (i / WIDTH as usize) as u32;

        let ray = cam.ray_for_pixel(x, y);
        let color = color_at(&world, &ray);
        pixel.copy_from_slice(&[
            (color.red * 255.0) as u8,
            (color.green * 255.0) as u8,
            (color.blue * 255.0) as u8,
            0xff,
        ]);
    }
}
