#![deny(clippy::all)]
#![forbid(unsafe_code)]

use log::error;
use pixels::wgpu::Color;
use pixels::{Error, Pixels, SurfaceTexture};
use std::time::Instant;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use crate::drawing::{clear, line};
use crate::drawing::{rect_filled, Point};
use crate::world::World;

mod drawing;
mod grid;
mod player;
mod renderer;
mod world;

const WIDTH: i32 = 1280;
const HEIGHT: i32 = 450;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("RayCasting")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
    };
    pixels.set_clear_color(Color {
        r: 0.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    });

    let mut world = World::new();
    world.init();

    let mut current_frame_time = Instant::now();
    let mut previous_frame_time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            previous_frame_time = current_frame_time;
            current_frame_time = Instant::now();
            let delta = (current_frame_time - previous_frame_time).as_micros() as f32 / 1000000.0;
            world.update(&input, delta);
            window.request_redraw();
        }

        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
    });
}
