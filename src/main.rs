use minifb::{Key, Window, WindowOptions, KeyRepeat};

const WIDTH: usize = 800; // Total width in pixels
const HEIGHT: usize = 800; // Total height in pixels
const GRID_SIZE: usize = 10; // Logical grid size (10x10)
const CELL_PIXELS: usize = WIDTH / GRID_SIZE; // Each cell is 80x80 pixels

fn main() {
    // Logical map representation (1D array)
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

    let mut player_pos = (1, 1); // Player starts at row 1, column 1
    let mut buffer = vec![0; WIDTH * HEIGHT]; // Framebuffer, reused each frame
    let mut keys_pressed = [false; 4]; // Track which keys are pressed (Up, Down, Left, Right)

    let mut window = Window::new(
        "Maze Game - Use Arrow Keys to Move",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Draw the map
        for r in 0..GRID_SIZE {
            for c in 0..GRID_SIZE {
                let index = r * GRID_SIZE + c;
                let color = match map[index] {
                    1 => 0x000000, // Wall (Black)
                    0 => 0xFFFFFF, // Path (White)
                    _ => 0x00FF00, // Player (Green)
                };

                // Fill the corresponding pixels in the framebuffer
                for y in 0..CELL_PIXELS {
                    for x in 0..CELL_PIXELS {
                        let px = c * CELL_PIXELS + x;
                        let py = r * CELL_PIXELS + y;
                        buffer[py * WIDTH + px] = color;
                    }
                }
            }
        }

        // Draw the player (in red)
        let (pr, pc) = player_pos;
        for y in 0..CELL_PIXELS {
            for x in 0..CELL_PIXELS {
                let px = pc * CELL_PIXELS + x;
                let py = pr * CELL_PIXELS + y;
                buffer[py * WIDTH + px] = 0xAA0000; // Player color (Red)
            }
        }

        // Handle input for movement (minimized copies)
        handle_input(&mut player_pos, &map, &mut keys_pressed, &window);

        // Update the window
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn handle_input(player_pos: &mut (usize, usize), map: &[usize], keys_pressed: &mut [bool; 4], window: &Window) {
    // Up
    if window.is_key_pressed(Key::Up, KeyRepeat::No) && is_walkable(map, player_pos.0 - 1, player_pos.1) && !keys_pressed[0] {
        player_pos.0 -= 1; // Move player up by one logical unit (one grid cell)
        keys_pressed[0] = true;
    } else if !window.is_key_down(Key::Up) {
        keys_pressed[0] = false;
    }

    // Down
    if window.is_key_pressed(Key::Down, KeyRepeat::No) && is_walkable(map, player_pos.0 + 1, player_pos.1) && !keys_pressed[1] {
        player_pos.0 += 1;
        keys_pressed[1] = true;
    } else if !window.is_key_down(Key::Down) {
        keys_pressed[1] = false;
    }

    // Left
    if window.is_key_pressed(Key::Left, KeyRepeat::No) && is_walkable(map, player_pos.0, player_pos.1 - 1) && !keys_pressed[2] {
        player_pos.1 -= 1;
        keys_pressed[2] = true;
    } else if !window.is_key_down(Key::Left) {
        keys_pressed[2] = false;
    }

    // Right
    if window.is_key_pressed(Key::Right, KeyRepeat::No) && is_walkable(map, player_pos.0, player_pos.1 + 1) && !keys_pressed[3] {
        player_pos.1 += 1;
        keys_pressed[3] = true;
    } else if !window.is_key_down(Key::Right) {
        keys_pressed[3] = false;
    }
}

// Helper function to check if a cell is walkable
fn is_walkable(map: &[usize], row: usize, col: usize) -> bool {
    if row < GRID_SIZE && col < GRID_SIZE {
        let index = row * GRID_SIZE + col;
        map.get(index) == Some(&0) // Walkable if the cell is `0` (path) rather than `1` (wall)
    } else {
        false
    }
}

