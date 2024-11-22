use minifb::{Key, Window, WindowOptions, KeyRepeat};
use std::time::{Instant, Duration};

const WIDTH: usize = 800; // Total width in pixels
const HEIGHT: usize = 800; // Total height in pixels
const GRID_SIZE: usize = 10; // Logical grid size (10x10)
const CELL_PIXELS: usize = WIDTH / GRID_SIZE; // Each cell is 80x80 pixels
const GHOST_MOVE_FRAMES: usize = 10; // Frames before the ghost moves

fn main() {
    let map = vec![
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 0, 0, 0, 1, 0, 0, 0, 0, 1,
        1, 1, 1, 0, 1, 0, 1, 1, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 1, 0, 1,
        1, 0, 1, 1, 1, 1, 0, 0, 0, 1,
        1, 0, 1, 0, 0, 1, 0, 1, 0, 1,
        1, 0, 1, 0, 1, 1, 0, 1, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 1, 0, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 0, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ];

    let mut player_pos = (1, 1); // Player starts at (1, 1)
    let mut ghost_pos = (3, 1); // Ghost starts at (2, 2)
    let mut buffer = vec![0; WIDTH * HEIGHT]; // Framebuffer
    let mut keys_pressed = [false; 4]; // Track which keys are pressed
    let path = vec![
        (0, 1), (0, 1), (0, 1), (0, 1), (0, 1),
        (1, 0), (1, 0), (1, 0), (1, 0),
        (0, -1), (0, -1), (0, -1), (0, -1), (0, -1),
        (-1, 0), (-1, 0), (-1, 0), (-1, 0),
    ];
    let mut path_index = 0; // Current position in the path
    let mut frame_count = 0; // Frame counter

    let mut window = Window::new(
        "Maze Game - Use Arrow Keys to Move",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        frame_count += 1;

        // Move the ghost every GHOST_MOVE_FRAMES
        if frame_count >= GHOST_MOVE_FRAMES {
            frame_count = 0;
            let (dr, dc) = path[path_index];
            let new_pos = (
                (ghost_pos.0 as isize + dr) as usize,
                (ghost_pos.1 as isize + dc) as usize,
            );

            // Ensure the new position is walkable
            if is_walkable(&map, new_pos.0, new_pos.1) {
                ghost_pos = new_pos;
            }
            path_index = (path_index + 1) % path.len();
        }

        // Draw the map
        for r in 0..GRID_SIZE {
            for c in 0..GRID_SIZE {
                let index = r * GRID_SIZE + c;
                let color = match map[index] {
                    1 => 0x000000, // Wall (Black)
                    0 => 0xFFFFFF, // Path (White)
                    _ => 0x00FF00, // Player (Green)
                };

                for y in 0..CELL_PIXELS {
                    for x in 0..CELL_PIXELS {
                        let px = c * CELL_PIXELS + x;
                        let py = r * CELL_PIXELS + y;
                        buffer[py * WIDTH + px] = color;
                    }
                }
            }
        }

        // Draw the player (red square)
        let (pr, pc) = player_pos;
        for y in 0..CELL_PIXELS {
            for x in 0..CELL_PIXELS {
                let px = pc * CELL_PIXELS + x;
                let py = pr * CELL_PIXELS + y;
                buffer[py * WIDTH + px] = 0xAA0000; // Player color (Red)
            }
        }

        // Draw the ghost (purple square)
        let (gr, gc) = ghost_pos;
        for y in 0..CELL_PIXELS {
            for x in 0..CELL_PIXELS {
                let px = gc * CELL_PIXELS + x;
                let py = gr * CELL_PIXELS + y;
                buffer[py * WIDTH + px] = 0x800080; // Ghost color (Purple)
            }
        }

        // Handle input for movement
        handle_input(&mut player_pos, &map, &mut keys_pressed, &window);

        // Update the window
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn handle_input(player_pos: &mut (usize, usize), map: &[usize], keys_pressed: &mut [bool; 4], window: &Window) {
    if window.is_key_pressed(Key::Up, KeyRepeat::No) && is_walkable(map, player_pos.0 - 1, player_pos.1) && !keys_pressed[0] {
        player_pos.0 -= 1;
        keys_pressed[0] = true;
    } else if !window.is_key_down(Key::Up) {
        keys_pressed[0] = false;
    }

    if window.is_key_pressed(Key::Down, KeyRepeat::No) && is_walkable(map, player_pos.0 + 1, player_pos.1) && !keys_pressed[1] {
        player_pos.0 += 1;
        keys_pressed[1] = true;
    } else if !window.is_key_down(Key::Down) {
        keys_pressed[1] = false;
    }

    if window.is_key_pressed(Key::Left, KeyRepeat::No) && is_walkable(map, player_pos.0, player_pos.1 - 1) && !keys_pressed[2] {
        player_pos.1 -= 1;
        keys_pressed[2] = true;
    } else if !window.is_key_down(Key::Left) {
        keys_pressed[2] = false;
    }

    if window.is_key_pressed(Key::Right, KeyRepeat::No) && is_walkable(map, player_pos.0, player_pos.1 + 1) && !keys_pressed[3] {
        player_pos.1 += 1;
        keys_pressed[3] = true;
    } else if !window.is_key_down(Key::Right) {
        keys_pressed[3] = false;
    }
}

fn is_walkable(map: &[usize], row: usize, col: usize) -> bool {
    if row < GRID_SIZE && col < GRID_SIZE {
        let index = row * GRID_SIZE + col;
        map.get(index) == Some(&0)
    } else {
        false
    }
}

