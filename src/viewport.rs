use crate::grid::Grid;
use crate::player::Player;
use crate::renderer::cast_ray;
use crate::{line, rect_filled, Point, HEIGHT, WIDTH};
use vecmath::{vec2_len, Vector2};

pub struct Viewport {
    x_offset: i32,
    y_offset: i32,
    width: i32,
    height: i32,
    fov: f32,
}

impl Viewport {
    pub fn new() -> Viewport {
        Viewport {
            x_offset: HEIGHT + 20,
            y_offset: 1,
            width: WIDTH - 10 - (HEIGHT + 20),
            height: HEIGHT - 1,
            fov: 2.0 * (0.66 / 1.0 as f32).atan(), // 66 degrees
        }
    }

    pub fn draw(&self, frame: &mut [u8], player: &Player, grid: &Grid) {
        let grid_colour = [0, 200, 0, 255];
        rect_filled(
            frame,
            &Point {
                x: self.x_offset,
                y: self.y_offset,
            },
            &Point {
                x: self.x_offset + self.width,
                y: self.y_offset + self.height / 2,
            },
            [0, 128, 175, 255],
        );
        rect_filled(
            frame,
            &Point {
                x: self.x_offset,
                y: self.y_offset + self.height / 2,
            },
            &Point {
                x: self.x_offset + self.width,
                y: self.y_offset + self.height,
            },
            [128, 128, 128, 255],
        );
        line(
            frame,
            &Point {
                x: self.x_offset,
                y: self.y_offset,
            },
            &Point {
                x: self.x_offset + self.width,
                y: self.y_offset,
            },
            grid_colour,
        );
        line(
            frame,
            &Point {
                x: self.x_offset,
                y: self.y_offset + self.height,
            },
            &Point {
                x: self.x_offset + self.width,
                y: self.y_offset + self.height,
            },
            grid_colour,
        );
        line(
            frame,
            &Point {
                x: self.x_offset,
                y: self.y_offset,
            },
            &Point {
                x: self.x_offset,
                y: self.y_offset + self.height,
            },
            grid_colour,
        );
        line(
            frame,
            &Point {
                x: self.x_offset + self.width,
                y: self.y_offset,
            },
            &Point {
                x: self.x_offset + self.width,
                y: self.y_offset + self.height,
            },
            grid_colour,
        );

        let origin: Vector2<f32> = [player.x, player.y];
        let increment = (1.0 * (self.fov).sin()) / (self.width as f32 / 2.0);

        for x in 0..self.width {
            let angle = -(increment * (x - self.width / 2) as f32).atan() + player.angle;

            let direction = [angle.cos(), angle.sin()];
            let hit = cast_ray(origin, direction, grid);

            match hit {
                None => (),
                Some(h) => {
                    let direction_angle = (player.angle - angle).cos();
                    let distance = (vec2_len(h) * direction_angle).max(1.0);
                    let line_height = self.height as f32 / distance;
                    let colour = [0, (200.0 / distance) as u8, 0, 255];
                    let height_offset = (self.height - line_height as i32) / 2;
                    line(
                        frame,
                        &Point {
                            x: x + self.x_offset,
                            y: self.y_offset + height_offset,
                        },
                        &Point {
                            x: x + self.x_offset,
                            y: self.y_offset + height_offset + line_height as i32,
                        },
                        colour,
                    );
                }
            }
        }
    }
}
