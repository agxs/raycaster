use crate::grid::Grid;
use crate::{line, rect_filled, Point, HEIGHT};
use std::f32::consts::PI;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
}

impl Player {
    pub fn update(&mut self, input: &WinitInputHelper, grid: &Grid, delta: f32) {
        let cell_size = (HEIGHT / grid.height as i32) as f32;
        if input.key_held(VirtualKeyCode::W) {
            let x = (self.x / cell_size) as usize;
            let y = (self.y / cell_size) as usize;
            let projected_x = ((self.x + self.angle.cos().signum() * 6.0) / cell_size) as usize;
            let projected_y = ((self.y + self.angle.sin().signum() * -6.0) / cell_size) as usize;
            if grid.tiles[projected_x % grid.width as usize + y * grid.width as usize] == 0 {
                self.x += self.angle.cos() * 200.0 * delta;
            }
            if grid.tiles[x % grid.width as usize + projected_y * grid.width as usize] == 0 {
                self.y += self.angle.sin() * -200.0 * delta;
            }
        }
        if input.key_held(VirtualKeyCode::S) {
            let x = (self.x / cell_size) as usize;
            let y = (self.y / cell_size) as usize;
            let projected_x = ((self.x - self.angle.cos().signum() * 6.0) / cell_size) as usize;
            let projected_y = ((self.y - self.angle.sin().signum() * -6.0) / cell_size) as usize;
            if grid.tiles[projected_x % grid.width as usize + y * grid.width as usize] == 0 {
                self.x -= self.angle.cos() * 200.0 * delta;
            }
            if grid.tiles[x % grid.width as usize + projected_y * grid.width as usize] == 0 {
                self.y -= self.angle.sin() * -200.0 * delta;
            }
        }
        if input.key_held(VirtualKeyCode::A) {
            self.angle += 1.5 * delta;
            self.angle = self.angle % (2.0 * PI);
        }
        if input.key_held(VirtualKeyCode::D) {
            self.angle -= 1.5 * delta;
            self.angle = (self.angle % (2.0 * PI) + (2.0 * PI)) % (2.0 * PI);
        }

        self.x = self.x.clamp(0.0, cell_size * grid.width as f32);
        self.y = self.y.clamp(0.0, cell_size * grid.height as f32);
    }

    pub fn draw(&self, frame: &mut [u8]) {
        let player_colour = [255, 0, 0, 255];
        rect_filled(
            frame,
            &Point {
                x: self.x as i32 - 5,
                y: self.y as i32 - 5,
            },
            &Point {
                x: self.x as i32 + 5,
                y: self.y as i32 + 5,
            },
            player_colour,
        );

        line(
            frame,
            &Point {
                x: self.x as i32,
                y: self.y as i32,
            },
            &Point {
                x: (self.x + self.angle.cos() * 25.0) as i32,
                y: (self.y + self.angle.sin() * -25.0) as i32,
            },
            player_colour,
        );
    }
}
