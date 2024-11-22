use minifb::{Key, Window, WindowOptions};
use maze::{is_walkable,handle_input,draw_square};
use crate::constants::{WIDTH, HEIGHT, GRID_SIZE};
use crate::constants::GHOST_MOVE_FRAMES;
use std::{thread, time};


mod constants; 
                                     
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
    let mut ghost_pos = (3, 1);  // Ghost starts at row 3, column 1
    let mut buffer = vec![0; WIDTH * HEIGHT]; // Framebuffer, reused each frame
    let mut keys_pressed = [false; 4]; // Track which keys are pressed (Up, Down, Left, Right)

    let mut window = Window::new(
        "Maze Game - Use Arrow Keys to Move",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    let path = vec![
        (0, 1), (0, 1), (0, 1), (0, 1), (0, 1), // Right
        (1, 0), (1, 0), (1, 0), (1, 0), // Down
        (0, -1), (0, -1), (0, -1), (0, -1), (0, -1), // Left
        (-1, 0), (-1, 0), (-1, 0), (-1, 0), // Up
    ];

    let mut path_index = 0;
    let mut frame_count = 0;
    let frame_time = time::Duration::from_millis(26); // ~60 FPS

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame_start = time::Instant::now();
        frame_count += 1;

        // Move the ghost every GHOST_MOVE_FRAMES
        if frame_count >= GHOST_MOVE_FRAMES {
            frame_count = 0;
            // dr (delta row): Tells how much to change the row index (Y-axis movement)
            // dc (delta column): Tells how much to change the column index (X-axis movement)
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

                // Fill the corresponding pixels in the framebuffer
                draw_square(&mut buffer, c, r, color);
            }
        }

        // Draw the player (in red)
        let (pr, pc) = player_pos;
        draw_square(&mut buffer, pc, pr, 0xAA0000); // Player color (Red)

        // Draw the ghost (in purple)
        let (gr, gc) = ghost_pos;
        draw_square(&mut buffer, gc, gr, 0x800080); // Ghost color (Purple)

        // Handle input for movement (minimized copies)
        handle_input(&mut player_pos, &map, &mut keys_pressed, &window);

        // Update the window
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        // Sleep for the remaining time to maintain consistent frame rate
    let elapsed = frame_start.elapsed();
    if elapsed < frame_time {
        thread::sleep(frame_time - elapsed);
    }

    }
}
