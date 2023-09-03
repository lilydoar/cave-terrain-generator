use crate::{terrain::Terrain, SCREEN_HEIGHT, SCREEN_WIDTH};

pub fn clear_frame(frame: &mut [u8], color: &[u8; 4]) {
    for pixel in frame.chunks_exact_mut(4) {
        pixel.copy_from_slice(color);
    }
}

pub fn set_pixel(frame: &mut [u8], x: usize, y: usize, color: &[u8; 4]) {
    debug_assert!(x < SCREEN_WIDTH && y < SCREEN_HEIGHT);

    let index = (y * SCREEN_WIDTH + x) * 4;
    frame[index..index + 4].copy_from_slice(color);
}

pub fn set_row(frame: &mut [u8], y: usize, color: &[u8; 4]) {
    debug_assert!(y < SCREEN_HEIGHT);

    for x in 0..(SCREEN_WIDTH - 1) {
        set_pixel(frame, x, y, color);
    }
}

pub fn set_col(frame: &mut [u8], x: usize, color: &[u8; 4]) {
    debug_assert!(x < SCREEN_WIDTH);

    for y in 0..(SCREEN_HEIGHT - 1) {
        set_pixel(frame, x, y, color);
    }
}

pub fn render_grid(frame: &mut [u8], step_size: usize, color: &[u8; 4]) {
    // Draw rows
    for y in (0..SCREEN_HEIGHT - 1).step_by(step_size) {
        set_row(frame, y, color);
    }

    // Draw coloumns
    for x in (0..SCREEN_WIDTH - 1).step_by(step_size) {
        set_col(frame, x, color);
    }
}

pub fn render_square(frame: &mut [u8], x: usize, y: usize, size: usize, color: &[u8; 4]) {
    let x_0 = x;
    let y_0 = y;

    let mut x_1 = x + size;
    let mut y_1 = y + size;

    if x_0 >= SCREEN_WIDTH || y_0 >= SCREEN_HEIGHT {
        return;
    }

    if x_1 >= SCREEN_WIDTH {
        x_1 = SCREEN_WIDTH - 1;
    }

    if y_1 >= SCREEN_HEIGHT {
        y_1 = SCREEN_WIDTH - 1;
    }

    for row in y_0..y_1 {
        for col in x_0..x_1 {
            set_pixel(frame, col, row, color);
        }
    }
}

pub fn render_scalar_field(frame: &mut [u8], terrain: &Terrain) {
    let field = &terrain.scalar_field;

    let mut square_size = SCREEN_HEIGHT / terrain.height;
    if SCREEN_WIDTH < SCREEN_HEIGHT {
        square_size = SCREEN_WIDTH / terrain.width;
    }

    for row in 0..terrain.height {
        for col in 0..terrain.width {
            let x = (col as f64 * square_size as f64).round() as usize;
            let y = (row as f64 * square_size as f64).round() as usize;

            let alpha = (field[row][col] * 255.0).round() as u8;
            let color = [alpha, alpha, alpha, 255];

            render_square(frame, x, y, square_size, &color)
        }
    }
}

pub fn render_thresholded_field(frame: &mut [u8], terrain: &Terrain) {
    let field = &terrain.thresholded_field;

    let mut square_size = SCREEN_HEIGHT / terrain.height;
    if SCREEN_WIDTH < SCREEN_HEIGHT {
        square_size = SCREEN_WIDTH / terrain.width;
    }

    for row in 0..terrain.height {
        for col in 0..terrain.width {
            let x = (col as f64 * square_size as f64).round() as usize;
            let y = (row as f64 * square_size as f64).round() as usize;

            let alpha = field[row][col] * 255;
            let color = [alpha, alpha, alpha, 255];

            render_square(frame, x, y, square_size, &color)
        }
    }
}
