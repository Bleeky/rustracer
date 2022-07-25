use patterns::Pattern;
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
mod patterns;
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
use crate::objects::cube::*;
use crate::objects::plane::*;
use crate::objects::sphere::*;
use crate::objects::*;
use crate::patterns::blend::*;
use crate::patterns::checker::*;
use crate::patterns::gradient::*;
use crate::patterns::perturbed::*;
use crate::patterns::radial_gradient::*;
use crate::patterns::ring::*;
use crate::patterns::solid_color::*;
use crate::patterns::stripe::*;
use crate::point::*;
use crate::vector3::*;
use crate::world::*;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;
// const WIDTH: u32 = 1200;
// const HEIGHT: u32 = 800;
const MAX_RECURSION: i32 = 5;
// const WIDTH: u32 = 2560;
// const HEIGHT: u32 = 1440;

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
    let middlesphere = Object::Sphere(Sphere::new(&Material {
        color: Color {
            red: 0.1,
            green: 1.0,
            blue: 0.5,
        },
        diffuse: 1.0,
        specular: 0.0,
        // refractive_index: 1.59,
        // transparency: 1.0,
        pattern: Some(
            Pattern::Checker(Checker::new(
                Pattern::SolidColor(SolidColor::new(Color::cyan())),
                Pattern::SolidColor(SolidColor::new(Color::cyan() - 0.08)),
            ))
            .set_transform(
                Matrix44::scaling(0.5, 0.5, 0.5)
                    .rotate_z(std::f64::consts::FRAC_PI_6)
                    .rotate_x(-std::f64::consts::FRAC_PI_6),
            ),
        ),
        ..Material::default()
    }))
    .set_transform(Matrix44::scaling(2.0, 2.0, 2.0).translate(-0.5, 2.0, 3.5));
    let rightsphere = Object::Sphere(Sphere::new(&Material {
        color: Color {
            red: 0.5,
            green: 1.0,
            blue: 0.1,
        },
        diffuse: 0.7,
        specular: 0.3,
        reflective: 0.3,
        pattern: Some(Pattern::Perturbed(Perturbed::new(
            Pattern::Ring(Ring::new(
                Pattern::SolidColor(SolidColor::new(Color::yellow() + 0.2)),
                Pattern::SolidColor(SolidColor::new(Color::orange())),
            ))
            .set_transform(Matrix44::rotation_x(std::f64::consts::FRAC_PI_2).scale(0.1, 0.1, 0.1)),
            0.4,
        ))),
        ..Material::default()
    }))
    .set_transform(Matrix44::scaling(0.5, 0.5, 0.5).translate(1.5, 0.5, -0.5));
    let rightsphere_2 = Object::Sphere(Sphere::new(&Material {
        color: Color {
            red: 0.4,
            green: 0.4,
            blue: 0.4,
        },
        diffuse: 0.1,
        refractive_index: 1.59,
        transparency: 0.9,
        specular: 1.0,
        shininess: 300.0,
        ..Material::default()
    }))
    .set_transform(Matrix44::scaling(0.3, 0.3, 0.3).translate(0.0, 0.3, -1.5));
    let leftsphere = Object::Sphere(Sphere::new(&Material {
        color: Color {
            red: 1.0,
            green: 0.8,
            blue: 0.1,
        },
        diffuse: 0.7,
        specular: 0.8,
        pattern: Some(
            Pattern::RadialGradient(RadialGradient::new(
                Pattern::SolidColor(SolidColor::new(Color::green() + 0.56)),
                Pattern::SolidColor(SolidColor::new(Color::green() + 0.23)),
            ))
            .set_transform(Matrix44::rotation_x(-std::f64::consts::FRAC_PI_4).scale(0.2, 0.2, 0.2)),
        ),
        ..Material::default()
    }))
    .set_transform(Matrix44::scaling(0.33, 0.33, 0.33).translate(-1.5, 0.33, -0.75));
    let plane = Object::Plane(Plane::new(Material {
        specular: 0.0,
        diffuse: 0.5,
        ambient: 0.01,
        reflective: 0.05,
        color: Color {
            red: 0.0,
            green: 0.9,
            blue: 0.9,
        },
        pattern: Some(Pattern::Blend(Blend::new(
            Pattern::Stripe(Stripe::new(
                Pattern::SolidColor(SolidColor::new(Color::gray())),
                Pattern::SolidColor(SolidColor::new(Color::gray() + 0.08)),
            ))
            .set_transform(Matrix44::scaling(0.2, 0.2, 0.2).rotate_y(std::f64::consts::FRAC_PI_4)),
            Pattern::Stripe(Stripe::new(
                Pattern::SolidColor(SolidColor::new(Color::braun() + 0.2)),
                Pattern::SolidColor(SolidColor::new(Color::braun() + 0.1)),
            )),
            0.7,
        ))),
        // pattern: Some(Pattern::Checker(Checker::new(
        //     Pattern::Stripe(Stripe::new(
        //         Pattern::SolidColor(SolidColor::new(Color::green() - 0.2)),
        //         Pattern::SolidColor(SolidColor::new(Color::green() + 0.08)),
        //     ))
        //     .set_transform(Matrix44::scaling(0.2, 0.2, 0.2).rotate_y(std::f64::consts::FRAC_PI_4)),
        //     Pattern::Stripe(Stripe::new(
        //         Pattern::SolidColor(SolidColor::new(Color::yellow() - 0.4)),
        //         Pattern::SolidColor(SolidColor::new(Color::yellow() - 0.2)),
        //     ))
        //     .set_transform(Matrix44::scaling(0.2, 0.2, 0.2).rotate_y(-std::f64::consts::FRAC_PI_4)),
        // ))),
        ..Material::default()
    }));
    let cube = Object::Cube(Cube::new(&Material {
        color: Color {
            red: 1.0,
            green: 0.3,
            blue: 0.5,
        },
        diffuse: 0.7,
        specular: 0.8,
        reflective: 0.3,
        pattern: Some(
            Pattern::Stripe(Stripe::new(
                Pattern::SolidColor(SolidColor::new(Color::red())),
                Pattern::SolidColor(SolidColor::new(Color::red() + 0.28)),
            ))
            .set_transform(Matrix44::scaling(0.2, 0.2, 0.2).rotate_y(std::f64::consts::FRAC_PI_4)),
        ),
        ..Material::default()
    }))
    .set_transform(Matrix44::scaling(0.1, 0.1, 0.1).translate(1.0, 0.1, -2.0));

    let world = World {
        objects: vec![
            cube,
            plane,
            middlesphere,
            rightsphere,
            leftsphere,
            rightsphere_2,
        ],
        lights: vec![
            Light::PointLight(PointLight {
                position: Point {
                    x: -10.0,
                    y: 10.0,
                    z: -10.0,
                },
                color: Color::white(),
            }),
            // Light::PointLight(PointLight {
            //     position: Point {
            //         x: 10.0,
            //         y: 3.0,
            //         z: -30.0,
            //     },
            //     color: Color {
            //         red: 0.7,
            //         green: 0.7,
            //         blue: 0.7,
            //     },
            // }),
        ],
    };
    let mut cam = Camera::new(WIDTH, HEIGHT, std::f64::consts::PI / 3.0);
    cam.transform = view_transform(
        Point {
            x: 0.0,
            y: 1.5,
            z: -5.0,
        },
        Point {
            x: 0.0,
            y: 1.0,
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
        let color = color_at(&world, &ray, MAX_RECURSION);
        pixel.copy_from_slice(&[
            (color.red * 255.0) as u8,
            (color.green * 255.0) as u8,
            (color.blue * 255.0) as u8,
            0xff,
        ]);
    }
}
