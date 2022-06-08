use crate::drawing::pixel;
use crate::grid::Grid;
use crate::player::Player;
use crate::ray::{cast_ray, HitSide};
use crate::{line, rect_filled, Point, HEIGHT, WIDTH};
use image::GenericImageView;
use vecmath::{vec2_len, Vector2};

pub struct Viewport {
    x_offset: i32,
    y_offset: i32,
    width: i32,
    height: i32,
    fov: f32,
    texture: Vec<u8>,
    tex_width: usize,
    tex_height: usize,
}

impl Viewport {
    pub fn new() -> Viewport {
        Viewport {
            x_offset: HEIGHT + 20,
            y_offset: 1,
            width: WIDTH - 10 - (HEIGHT + 20),
            height: HEIGHT - 1,
            fov: 2.0 * (0.66 / 1.0 as f32).atan(), // 66 degrees
            texture: Vec::new(),
            tex_width: 0,
            tex_height: 0,
        }
    }

    pub fn init(&mut self) {
        let grid_image = image::open("assets/brick_2.png").unwrap();
        self.tex_width = grid_image.width() as usize;
        self.tex_height = grid_image.height() as usize;
        self.texture
            .resize((grid_image.width() * grid_image.height() * 4) as usize, 0);

        grid_image.pixels().for_each(|pixel| {
            let x = pixel.0 as usize;
            let y = pixel.1 as usize;
            let i = x * 4 + (y * grid_image.width() as usize) * 4;
            self.texture[i..i + 4].copy_from_slice(&pixel.2 .0);
        });
    }

    pub fn draw(&self, frame: &mut [u8], player: &Player, grid: &Grid) {
        let grid_colour = [0, 200, 0, 255];
        // sky
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
        // floor
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
        // border
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

        // walls
        let origin: Vector2<f32> = [player.x, player.y];
        let increment = (1.0 * (self.fov).sin()) / (self.width as f32 / 2.0);

        for x in 0..self.width {
            let ray_angle = -(increment * (x - self.width / 2) as f32).atan() + player.angle;

            let ray_direction = [ray_angle.cos(), ray_angle.sin()];
            let hit = cast_ray(origin, ray_direction, grid);

            match hit {
                None => (),
                Some((h, s)) => {
                    let direction_angle = (player.angle - ray_angle).cos();
                    let distance = vec2_len(h) * direction_angle;
                    let line_height = (self.height as f32 / distance) as i32;

                    // how far into a map file is the hit
                    // lets us sample the texture at the right x coordinate
                    let x_texture = match s {
                        HitSide::X => origin[0] + h[0],
                        HitSide::Y => origin[1] - h[1],
                    }
                    .fract();

                    let height_offset = (self.height - line_height) / 2;
                    for y in 0..line_height {
                        pixel(
                            frame,
                            x + self.x_offset,
                            self.y_offset + height_offset + y,
                            self.sample_texture(x_texture, y as f32 / line_height as f32, s),
                        );
                    }
                }
            }
        }
    }

    fn sample_texture(&self, x: f32, y: f32, side: HitSide) -> [u8; 4] {
        let grid_x = x * self.tex_width as f32;
        let grid_y = y * self.tex_height as f32;
        let start = grid_x as usize * 4 + grid_y as usize * self.tex_width * 4;

        let mut c: [u8; 4] = [0; 4];
        c.copy_from_slice(&self.texture[start..start + 4]);
        // darken the x direction tile sides
        match side {
            HitSide::X => {
                c[0..3].iter_mut().for_each(|c| {
                    *c = (*c as f32 * 0.6) as u8;
                });
            }
            _ => {}
        }
        return c;
    }
}
