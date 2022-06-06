use crate::grid::Grid;
use crate::renderer::cast_ray;
use crate::{line, rect_filled, Point, HEIGHT, WIDTH};
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};
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
            [255, 255, 0, 255],
        );

        // Screen width, should be from constant or at least defined in one place...
        let width = WIDTH - 10 - (HEIGHT + 20);
        // To avoid distortion we can't send rays out with equal angles, this bunches
        // them up in the centre. Instead we use divide up the "opposite" side of the
        // triangle into equal portions and figure out the angles using tan θ = op/adj
        let increment = (1.0 * (FRAC_PI_4).sin()) / (width as f32 / 2.0);
        for x in 0..width {
            let angle = -(increment * (x - width / 2) as f32).atan() + self.angle;

            let direction = [angle.cos(), angle.sin()];
            self.cast_ray(screen_x, screen_y, grid, frame, direction);
        }
    }

    fn cast_ray(
        &self,
        screen_x: i32,
        screen_y: i32,
        grid: &Grid,
        frame: &mut [u8],
        direction: Vector2<f32>,
    ) -> f32 {
        let origin: Vector2<f32> = [self.x, self.y];
        let hit = cast_ray(origin, direction, grid);
        let mut length = 0.0;
        let cast_point = match hit {
            None => Point {
                x: (screen_x as f32 + direction[0] * 10.0 * grid.tile_size as f32) as i32,
                y: (screen_y as f32 + direction[1] * -10.0 * grid.tile_size as f32) as i32,
            },
            Some(h) => {
                length = vec2_len(h);
                Point {
                    x: (screen_x as f32 + direction[0] * length * grid.tile_size as f32) as i32,
                    y: (screen_y as f32 + direction[1] * -length * grid.tile_size as f32) as i32,
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
            [255, 0, 0, 255],
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

        return length;
    }
}
