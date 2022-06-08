use crate::{HEIGHT, WIDTH};
use line_drawing::Bresenham;

pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub fn line(screen: &mut [u8], p1: &Point, p2: &Point, colour: [u8; 4]) {
    let p1 = (p1.x.clamp(0, WIDTH - 1), p1.y.clamp(0, HEIGHT - 1));
    let p2 = (p2.x.clamp(0, WIDTH - 1), p2.y.clamp(0, HEIGHT - 1));

    for (x, y) in Bresenham::new(p1, p2) {
        let i = (x * 4 + y * WIDTH * 4) as usize;

        screen[i..i + 4].copy_from_slice(&colour);
    }
}

pub fn rect_filled(screen: &mut [u8], lower: &Point, upper: &Point, colour: [u8; 4]) {
    for y in lower.y..upper.y {
        for x in lower.x..upper.x {
            let i = (x * 4 + y * WIDTH * 4) as usize;
            screen[i..i + 4].copy_from_slice(&colour);
        }
    }
}

pub fn clear(screen: &mut [u8]) {
    for (i, byte) in screen.iter_mut().enumerate() {
        *byte = if i % 4 == 3 { 255 } else { 0 };
    }
}
