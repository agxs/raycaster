use crate::{line, rect_filled, Point};
use std::f32::consts::PI;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
}

impl Player {
    pub fn update(&mut self, input: &WinitInputHelper, delta: f32) {
        if input.key_held(VirtualKeyCode::W) {
            self.x += self.angle.cos() * 200.0 * delta;
            self.y += self.angle.sin() * -200.0 * delta;
        }
        if input.key_held(VirtualKeyCode::S) {
            self.x -= self.angle.cos() * 200.0 * delta;
            self.y -= self.angle.sin() * -200.0 * delta;
        }
        if input.key_held(VirtualKeyCode::A) {
            self.angle += 1.5 * delta;
            self.angle = self.angle % (2.0 * PI);
        }
        if input.key_held(VirtualKeyCode::D) {
            self.angle -= 1.5 * delta;
            self.angle = (self.angle % (2.0 * PI) + (2.0 * PI)) % (2.0 * PI);
        }
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
