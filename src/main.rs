#![deny(clippy::all)]
#![forbid(unsafe_code)]

use image::GenericImageView;
use log::error;
use pixels::wgpu::Color;
use pixels::{Error, Pixels, SurfaceTexture};
use std::f32::consts::PI;
use std::time::Instant;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use crate::drawing::{clear, line};
use crate::drawing::{rect_filled, Point};

mod drawing;

const WIDTH: i32 = 1280;
const HEIGHT: i32 = 450;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
    grid: [[u8; 10]; 10],
    player: Player,
}

struct Player {
    x: f32,
    y: f32,
    angle: f32,
}

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

impl World {
    fn new() -> World {
        World {
            grid: [[0; 10]; 10],
            player: Player {
                x: 0.0,
                y: 0.0,
                angle: 0.0,
            },
        }
    }

    fn init(&mut self) {
        let grid_image = image::open("assets/grid.png").unwrap();
        grid_image.pixels().for_each(|pixel| {
            let x = pixel.0 as usize;
            let y = pixel.1 as usize;
            self.grid[y][x] = pixel.2 .0[0];
        });
        self.player.x = HEIGHT as f32 / 2.0;
        self.player.y = HEIGHT as f32 / 2.0;
    }
    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self, input: &WinitInputHelper, delta: f32) {
        if input.key_held(VirtualKeyCode::W) {
            self.player.x += self.player.angle.cos() * 200.0 * delta;
            self.player.y += self.player.angle.sin() * -200.0 * delta;
        }
        if input.key_held(VirtualKeyCode::S) {
            self.player.x -= self.player.angle.cos() * 200.0 * delta;
            self.player.y -= self.player.angle.sin() * -200.0 * delta;
        }
        if input.key_held(VirtualKeyCode::A) {
            self.player.angle += 1.5 * delta;
            self.player.angle = self.player.angle % (2.0 * PI);
        }
        if input.key_held(VirtualKeyCode::D) {
            self.player.angle -= 1.5 * delta;
            self.player.angle = (self.player.angle % (2.0 * PI) + (2.0 * PI)) % (2.0 * PI);
        }
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        clear(frame);
        self.draw_grid(frame);
        self.draw_player(frame);
        // for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        //     let _x = (i % WIDTH as usize) as i16;
        //     let _y = (i / WIDTH as usize) as i16;
        //
        //     let rgba = [0x48, 0xb2, 0xe8, 0xff];
        //
        //     pixel.copy_from_slice(&rgba);
        // }
    }

    fn draw_grid(&self, frame: &mut [u8]) {
        let cell = HEIGHT / self.grid.len() as i32;

        for (y_index, y) in self.grid.iter().enumerate() {
            for (x_index, x) in y.iter().enumerate() {
                if *x > 1 {
                    rect_filled(
                        frame,
                        &Point {
                            x: x_index as i32 * cell,
                            y: y_index as i32 * cell,
                        },
                        &Point {
                            x: x_index as i32 * cell + cell,
                            y: y_index as i32 * cell + cell,
                        },
                        [0, 255, 0, 255],
                    )
                }
            }
        }

        let grid_colour = [0, 200, 0, 255];
        for i in (0..HEIGHT).step_by(cell as usize) {
            line(
                frame,
                &Point { x: 0, y: i },
                &Point { x: HEIGHT, y: i },
                grid_colour,
            );
        }
        for i in (0..HEIGHT).step_by(cell as usize) {
            line(
                frame,
                &Point { x: i, y: 0 },
                &Point { x: i, y: HEIGHT },
                grid_colour,
            );
        }
    }

    fn draw_player(&self, frame: &mut [u8]) {
        let player_colour = [255, 0, 0, 255];
        rect_filled(
            frame,
            &Point {
                x: self.player.x as i32 - 5,
                y: self.player.y as i32 - 5,
            },
            &Point {
                x: self.player.x as i32 + 5,
                y: self.player.y as i32 + 5,
            },
            player_colour,
        );

        line(
            frame,
            &Point {
                x: self.player.x as i32,
                y: self.player.y as i32,
            },
            &Point {
                x: (self.player.x + self.player.angle.cos() * 25.0) as i32,
                y: (self.player.y + self.player.angle.sin() * -25.0) as i32,
            },
            player_colour,
        );
    }
}
