// use image::{DynamicImage, GenericImage};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
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

use crate::color::*;
use crate::intersection::*;
use crate::light::*;
use crate::material::*;
use crate::matrix::*;
use crate::objects::sphere::*;
use crate::objects::*;
use crate::point::*;
use crate::ray::*;
use crate::world::*;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;

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
    let wall_z = 10.0;
    let wall_size = 7.0;
    let pixel_size = wall_size / WIDTH as f32;
    let half = wall_size / 2.0;
    let world = World::default();
    let ray_origin = Point {
        x: 0.0,
        y: 0.0,
        z: -5.0,
    };
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = (i % WIDTH as usize) as u32;
        let y = (i / WIDTH as usize) as u32;
        let world_y = half - pixel_size * y as f32;
        let world_x = -half + pixel_size * x as f32;
        let position = Point {
            x: world_x as f64,
            y: world_y as f64,
            z: wall_z,
        };
        let ray = Ray {
            origin: ray_origin,
            direction: (position - ray_origin).normalize(),
        };
        let color = color_at(&world, &ray);
        pixel.copy_from_slice(&[
            (color.red * 255.0) as u8,
            (color.green * 255.0) as u8,
            (color.blue * 255.0) as u8,
            0xff,
        ]);
    }
}
