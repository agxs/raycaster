use crate::grid::Grid;
use crate::renderer::cast_ray;
use crate::{line, rect_filled, Point};
use std::f32::consts::{FRAC_PI_2, PI};
use vecmath::{vec2_len, Vector2};
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
        if input.key_held(VirtualKeyCode::Key1) {
            self.angle = 0.0;
        }
        if input.key_held(VirtualKeyCode::Key2) {
            self.angle = FRAC_PI_2;
        }
        if input.key_held(VirtualKeyCode::Key3) {
            self.angle = PI;
        }
        if input.key_held(VirtualKeyCode::Key4) {
            self.angle = FRAC_PI_2 * 3.0;
        }

        self.x = self.x.clamp(0.0, (grid.tile_size * grid.width) as f32);
        self.y = self.y.clamp(0.0, (grid.tile_size * grid.width) as f32);
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

        let origin: Vector2<f32> = [self.x, self.y];
        let direction: Vector2<f32> = self.normalised_direction();
        let hit = cast_ray(origin, direction, grid);
        let cast_point = match hit {
            None => Point {
                x: (screen_x as f32 + self.angle.cos() * 2.5 * grid.tile_size as f32) as i32,
                y: (screen_y as f32 + self.angle.sin() * -2.5 * grid.tile_size as f32) as i32,
            },
            Some(h) => {
                let length = vec2_len(h);
                Point {
                    x: (screen_x as f32 + self.angle.cos() * length * grid.tile_size as f32) as i32,
                    y: (screen_y as f32 + self.angle.sin() * -length * grid.tile_size as f32)
                        as i32,
                }
            }
        };

        line(
            frame,
            &Point {
                x: screen_x,
                y: screen_y,
            },
            &cast_point,
            player_colour,
        );

        rect_filled(
            frame,
            &Point {
                x: cast_point.x - 5,
                y: cast_point.y - 5,
            },
            &Point {
                x: cast_point.x + 5,
                y: cast_point.y + 5,
            },
            [0, 0, 255, 255],
        );
    }

    pub fn normalised_direction(&self) -> Vector2<f32> {
        [self.angle.cos(), self.angle.sin()]
    }
}
