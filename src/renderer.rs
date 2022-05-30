use crate::grid::Grid;
use vecmath::{vec2_scale, Vector2};

pub fn cast_ray(
    origin: Vector2<f32>,
    direction: Vector2<f32>,
    grid: &Grid,
) -> Option<Vector2<f32>> {
    let mut ray_unit_step_size: Vector2<f32> = [
        (1.0 + (direction[1] / direction[0]) * (direction[1] / direction[0])).sqrt(),
        (1.0 + (direction[0] / direction[1]) * (direction[0] / direction[1])).sqrt(),
    ];
    if ray_unit_step_size[0].is_infinite() {
        ray_unit_step_size[0] = f32::MAX;
    }
    if ray_unit_step_size[1].is_infinite() {
        ray_unit_step_size[1] = f32::MAX;
    }
    let mut current_tile: Vector2<i32> = [origin[0] as i32, origin[1] as i32];
    let mut ray_length_1d: Vector2<f32> = [0.0, 0.0];
    let step: Vector2<i32> = [direction[0].signum() as i32, direction[1].signum() as i32];

    if direction[0] < 0.0 {
        ray_length_1d[0] = (origin[0] - current_tile[0] as f32) * ray_unit_step_size[0];
    } else {
        ray_length_1d[0] = (current_tile[0] as f32 + 1.0 - origin[0]) * ray_unit_step_size[0];
    }
    if direction[1] < 0.0 {
        ray_length_1d[1] = (current_tile[1] as f32 + 1.0 - origin[1]) * ray_unit_step_size[1];
    } else {
        ray_length_1d[1] = (origin[1] - current_tile[1] as f32) * ray_unit_step_size[1];
    }

    let max_distance = 2.5;
    let mut distance = 0.0;
    let mut tile_found = false;
    while !tile_found && distance < max_distance {
        if ray_length_1d[0] < ray_length_1d[1] {
            current_tile[0] += step[0];
            distance = ray_length_1d[0];
            ray_length_1d[0] += ray_unit_step_size[0];
        } else {
            current_tile[1] -= step[1];
            distance = ray_length_1d[1];
            ray_length_1d[1] += ray_unit_step_size[1];
        }

        if current_tile[0] >= 0
            && current_tile[0] < grid.width
            && current_tile[1] >= 0
            && current_tile[1] < grid.height
        {
            if grid.tiles[(current_tile[0] + current_tile[1] * grid.width) as usize] > 0 {
                tile_found = true;
            }
        }
    }

    if tile_found {
        return Some(vec2_scale(direction, distance));
    }

    None
}
