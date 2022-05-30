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
        if input.key_held(VirtualKeyCode::W) {
            let x = self.x as usize;
            let y = self.y as usize;
            let projected_x = (self.x + self.angle.cos().signum() * 0.25) as usize;
            let projected_y = (self.y + self.angle.sin().signum() * -0.25) as usize;
            if grid.tiles[projected_x % grid.width as usize + y * grid.width as usize] == 0 {
                self.x += self.angle.cos() * 2.0 * delta;
            }
            if grid.tiles[x % grid.width as usize + projected_y * grid.width as usize] == 0 {
                self.y += self.angle.sin() * -2.0 * delta;
            }
        }
        if input.key_held(VirtualKeyCode::S) {
            let x = self.x as usize;
            let y = self.y as usize;
            let projected_x = (self.x - self.angle.cos().signum() * 0.25) as usize;
            let projected_y = (self.y - self.angle.sin().signum() * -0.25) as usize;
            if grid.tiles[projected_x % grid.width as usize + y * grid.width as usize] == 0 {
                self.x -= self.angle.cos() * 2.0 * delta;
            }
            if grid.tiles[x % grid.width as usize + projected_y * grid.width as usize] == 0 {
                self.y -= self.angle.sin() * -2.0 * delta;
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

        self.x = self
            .x
            .clamp(0.0, (grid.tile_size * grid.width as i32) as f32);
        self.y = self
            .y
            .clamp(0.0, (grid.tile_size * grid.width as i32) as f32);
    }

    pub fn draw(&self, frame: &mut [u8], grid: &Grid) {
        let player_colour = [255, 0, 0, 255];
        let screen_x = (self.x * grid.tile_size as f32) as i32;
        let screen_y = (self.y * grid.tile_size as f32) as i32;
        rect_filled(
            frame,
            &Point {
                x: screen_x - 5,
                y: screen_y - 5,
            },
            &Point {
                x: screen_x + 5,
                y: screen_y + 5,
            },
            player_colour,
        );

        line(
            frame,
            &Point {
                x: screen_x,
                y: screen_y,
            },
            &Point {
                x: (screen_x as f32 + self.angle.cos() * 25.0) as i32,
                y: (screen_y as f32 + self.angle.sin() * -25.0) as i32,
            },
            player_colour,
        );
    }
}
