use crate::{line, rect_filled, Point, HEIGHT};
use image::GenericImageView;

pub struct Grid {
    pub tiles: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl Grid {
    pub fn init(&mut self) {
        let grid_image = image::open("assets/grid.png").unwrap();
        self.width = grid_image.width();
        self.height = grid_image.height();
        self.tiles
            .resize((grid_image.width() * grid_image.height()) as usize, 0);

        grid_image.pixels().for_each(|pixel| {
            let x = pixel.0 as usize;
            let y = pixel.1 as usize;
            let i = x + (y * grid_image.width() as usize);
            self.tiles[i] = pixel.2 .0[0];
        });
    }

    pub fn draw(&self, frame: &mut [u8]) {
        let cell = HEIGHT / self.height as i32;

        for (i, grid_value) in self.tiles.iter().enumerate() {
            let x_index = i % self.width as usize;
            let y_index = i / self.width as usize;
            if *grid_value > 1 {
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
}
