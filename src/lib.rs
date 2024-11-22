use crate::constants::HEIGHT; // Total height in pixels
const CELL_PIXELS: usize = WIDTH / GRID_SIZE; // Each cell is 80x80 pixels
use crate::constants::GRID_SIZE;
use crate::constants::WIDTH;
use minifb::{Key, KeyRepeat, Window};

mod constants;

// Helper function to check if a cell is walkable
pub fn is_walkable(map: &[usize], row: usize, col: usize) -> bool {
    if row < GRID_SIZE && col < GRID_SIZE {
        let index = row * GRID_SIZE + col;
        map.get(index) == Some(&0) // Walkable if the cell is `0` (path) rather than `1` (wall)
    } else {
        false
    }
}

// Function to handle the movement input
pub fn handle_input(
    player_pos: &mut (usize, usize),
    map: &[usize],
    keys_pressed: &mut [bool; 4],
    window: &Window,
) {
    // Up
    if window.is_key_pressed(Key::Up, KeyRepeat::No)
        && is_walkable(map, player_pos.0 - 1, player_pos.1)
        && !keys_pressed[0]
    {
        player_pos.0 -= 1;
        keys_pressed[0] = true;
    } else if !window.is_key_down(Key::Up) {
        keys_pressed[0] = false;
    }

    // Down
    if window.is_key_pressed(Key::Down, KeyRepeat::No)
        && is_walkable(map, player_pos.0 + 1, player_pos.1)
        && !keys_pressed[1]
    {
        player_pos.0 += 1;
        keys_pressed[1] = true;
    } else if !window.is_key_down(Key::Down) {
        keys_pressed[1] = false;
    }

    // Left
    if window.is_key_pressed(Key::Left, KeyRepeat::No)
        && is_walkable(map, player_pos.0, player_pos.1 - 1)
        && !keys_pressed[2]
    {
        player_pos.1 -= 1;
        keys_pressed[2] = true;
    } else if !window.is_key_down(Key::Left) {
        keys_pressed[2] = false;
    }

    // Right
    if window.is_key_pressed(Key::Right, KeyRepeat::No)
        && is_walkable(map, player_pos.0, player_pos.1 + 1)
        && !keys_pressed[3]
    {
        player_pos.1 += 1;
        keys_pressed[3] = true;
    } else if !window.is_key_down(Key::Right) {
        keys_pressed[3] = false;
    }
}

// Function to draw a square (player, ghost, etc.)
pub fn draw_square(buffer: &mut Vec<u32>, x: usize, y: usize, color: u32) {
    for dy in 0..CELL_PIXELS {
        for dx in 0..CELL_PIXELS {
            let px = x * CELL_PIXELS + dx;
            let py = y * CELL_PIXELS + dy;
            buffer[py * WIDTH + px] = color;
        }
    }
}
