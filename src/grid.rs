use crate::{line, rect_filled, Point, HEIGHT};
use image::GenericImageView;

pub struct Grid {
    pub tiles: Vec<u8>,
    pub width: i32,
    pub height: i32,
    pub tile_size: i32,
}

impl Grid {
    pub fn init(&mut self) {
        let grid_image = image::open("assets/grid.png").unwrap();
        self.width = grid_image.width() as i32;
        self.height = grid_image.height() as i32;
        self.tiles
            .resize((grid_image.width() * grid_image.height()) as usize, 0);
        self.tile_size = HEIGHT / self.height;

        grid_image.pixels().for_each(|pixel| {
            let x = pixel.0 as usize;
            let y = pixel.1 as usize;
            let i = x + (y * grid_image.width() as usize);
            self.tiles[i] = pixel.2 .0[0];
        });
    }

    pub fn draw(&self, frame: &mut [u8]) {
        for (i, grid_value) in self.tiles.iter().enumerate() {
            let x_index = i % self.width as usize;
            let y_index = i / self.width as usize;
            if *grid_value > 1 {
                rect_filled(
                    frame,
                    &Point {
                        x: x_index as i32 * self.tile_size,
                        y: y_index as i32 * self.tile_size,
                    },
                    &Point {
                        x: x_index as i32 * self.tile_size + self.tile_size,
                        y: y_index as i32 * self.tile_size + self.tile_size,
                    },
                    [0, 255, 0, 255],
                )
            }
        }

        let grid_colour = [0, 200, 0, 255];
        for i in (0..HEIGHT).step_by(self.tile_size as usize) {
            line(
                frame,
                &Point { x: 0, y: i },
                &Point { x: HEIGHT, y: i },
                grid_colour,
            );
        }
        for i in (0..HEIGHT).step_by(self.tile_size as usize) {
            line(
                frame,
                &Point { x: i, y: 0 },
                &Point { x: i, y: HEIGHT },
                grid_colour,
            );
        }
    }
}
