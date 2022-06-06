use crate::clear;
use crate::grid::Grid;
use crate::player::Player;
use crate::viewport::Viewport;
use winit_input_helper::WinitInputHelper;

pub struct World {
    grid: Grid,
    player: Player,
    viewport: Viewport,
}

impl World {
    pub fn new() -> World {
        World {
            grid: Grid {
                tiles: Vec::new(),
                width: 0,
                height: 0,
                tile_size: 0,
            },
            player: Player {
                x: 0.0,
                y: 0.0,
                angle: 0.0,
            },
            viewport: Viewport::new(),
        }
    }

    pub fn init(&mut self) {
        self.grid.init();
        self.player.x = self.grid.width as f32 / 2.0;
        self.player.y = self.grid.height as f32 / 2.0;
    }
    /// Update everything in the world
    pub fn update(&mut self, input: &WinitInputHelper, delta: f32) {
        self.player.update(input, &self.grid, delta);
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    pub fn draw(&self, frame: &mut [u8]) {
        clear(frame);
        self.grid.draw(frame);
        self.player.draw(frame, &self.grid);
        self.viewport.draw(frame, &self.player, &self.grid);
    }
}
