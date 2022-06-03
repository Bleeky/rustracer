// use image::{DynamicImage, GenericImage};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
mod color;
mod light;
mod matrix;
mod plane;
mod point;
mod rendering;
mod scene;
mod sphere;
mod vector3;

use crate::color::*;
use crate::light::*;
use crate::plane::Plane;
use crate::point::*;
use crate::rendering::*;
use crate::scene::*;
use crate::sphere::*;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

fn main() -> Result<(), Error> {
    let scene = Scene {
        width: WIDTH,
        height: HEIGHT,
        fov: 90.0,
        shadow_bias: 1e-13,
        lights: vec![
            Light::Directional(DirectionalLight {
                direction: vector3::Vector3 {
                    x: 0.0,
                    y: -1.0,
                    z: -1.0,
                },
                color: Color {
                    red: 1.0,
                    green: 1.0,
                    blue: 1.0,
                },
                intensity: 3.0,
            }),
            Light::Spherical(SphericalLight {
                position: Point {
                    x: 5.0,
                    y: 10.0,
                    z: 10.0,
                },
                color: Color {
                    red: 1.0,
                    green: 1.0,
                    blue: 1.0,
                },
                intensity: 30000.0,
            }),
        ],
        objects: vec![
            Object::Plane(Plane {
                origin: Point {
                    x: 0.0,
                    y: -2.0,
                    z: -5.0,
                },
                normal: vector3::Vector3 {
                    x: 0.0,
                    y: -1.0,
                    z: 0.0,
                },
                color: Color {
                    red: 1.0,
                    green: 0.2,
                    blue: 1.0,
                },
            }),
            Object::Sphere(Sphere {
                center: Point {
                    x: 0.0,
                    y: 0.0,
                    z: -5.0,
                },
                radius: 1.0,
                color: Color {
                    red: 0.4,
                    green: 1.0,
                    blue: 0.4,
                },
            }),
            Object::Sphere(Sphere {
                center: Point {
                    x: -3.0,
                    y: -1.0,
                    z: -6.0,
                },
                radius: 2.0,
                color: Color {
                    red: 0.7,
                    green: 1.0,
                    blue: 0.9,
                },
            }),
            Object::Sphere(Sphere {
                center: Point {
                    x: 2.0,
                    y: 1.0,
                    z: -4.0,
                },
                radius: 1.5,
                color: Color {
                    red: 1.0,
                    green: 1.0,
                    blue: 0.2,
                },
            }),
        ],
    };
    // let mut img = DynamicImage::new_rgb8(scene.width, scene.height);
    // for x in 0..scene.width {
    //     for y in 0..scene.height {
    //         let ray = Ray::create_prime(x, y, &scene);
    //         let intersection = scene.trace(&ray);
    //         match intersection {
    //             Some(i) => {
    //                 let color = get_color(&i, &ray, &scene);
    //                 img.put_pixel(
    //                     x,
    //                     y,
    //                     image::Rgba([
    //                         (color.red * 255.0) as u8,
    //                         (color.green * 255.0) as u8,
    //                         (color.blue * 255.0) as u8,
    //                         255,
    //                     ]),
    //                 );
    //             }
    //             None => {
    //                 img.put_pixel(x, y, image::Rgba([0, 0, 0, 255]));
    //             }
    //         }
    //     }
    // }
    // img.save("render.png").unwrap();

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
            draw(pixels.get_frame(), &scene);
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
            window.request_redraw();
        }
    });
}

fn draw(frame: &mut [u8], scene: &Scene) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = (i % WIDTH as usize) as u32;
        let y = (i / WIDTH as usize) as u32;
        let ray = Ray::create_prime(x, y, &scene);
        let intersection = scene.trace(&ray);
        match intersection {
            Some(i) => {
                let color = get_color(&i, &ray, &scene);
                pixel.copy_from_slice(&[
                    (color.red * 255.0) as u8,
                    (color.green * 255.0) as u8,
                    (color.blue * 255.0) as u8,
                    0xff,
                ]);
            }
            None => {
                pixel.copy_from_slice(&[0x48, 0xb2, 0xe8, 0xff]);
            }
        }
    }
}
