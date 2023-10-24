use crate::{
    font::{get_char_symbol, scale_symbol},
    terrain::Terrain,
    window::Screen,
};

pub fn clear_frame(screen: &mut Screen, color: &[u8; 4]) {
    for pixel in screen.frame_mut().chunks_exact_mut(4) {
        pixel.copy_from_slice(color);
    }
}

pub fn set_pixel(screen: &mut Screen, x: usize, y: usize, color: &[u8; 4]) {
    debug_assert!(x < screen.width() && y < screen.height());

    let index = (y * screen.width() + x) * 4;
    screen.frame_mut()[index..index + 4].copy_from_slice(color);
}

pub fn set_row(screen: &mut Screen, y: usize, color: &[u8; 4]) {
    debug_assert!(y < screen.height());

    for x in 0..(screen.width() - 1) {
        set_pixel(screen, x, y, color);
    }
}

pub fn set_col(screen: &mut Screen, x: usize, color: &[u8; 4]) {
    debug_assert!(x < screen.width());

    for y in 0..(screen.height() - 1) {
        set_pixel(screen, x, y, color);
    }
}

pub fn render_grid(screen: &mut Screen, step_size: usize, color: &[u8; 4]) {
    // Draw rows
    for y in (0..screen.height() - 1).step_by(step_size) {
        set_row(screen, y, color);
    }

    // Draw coloumns
    for x in (0..screen.width() - 1).step_by(step_size) {
        set_col(screen, x, color);
    }
}

pub fn render_square(screen: &mut Screen, x: usize, y: usize, size: usize, color: &[u8; 4]) {
    let x_0 = x;
    let y_0 = y;

    let mut x_1 = x + size;
    let mut y_1 = y + size;

    if x_0 >= screen.width() || y_0 >= screen.height() {
        return;
    }

    if x_1 >= screen.width() {
        x_1 = screen.width() - 1;
    }

    if y_1 >= screen.height() {
        y_1 = screen.width() - 1;
    }

    for row in y_0..y_1 {
        for col in x_0..x_1 {
            set_pixel(screen, col, row, color);
        }
    }
}

pub fn render_scalar_field(screen: &mut Screen, terrain: &Terrain) {
    let field = &terrain.scalar_field;

    let mut square_size = screen.height() / terrain.height;
    if screen.width() < screen.height() {
        square_size = screen.width() / terrain.width;
    }

    for row in 0..terrain.height {
        for col in 0..terrain.width {
            let x = (col as f64 * square_size as f64).round() as usize;
            let y = (row as f64 * square_size as f64).round() as usize;

            let alpha = (field[row][col] * 255.0).round() as u8;
            let color = [alpha, alpha, alpha, 255];

            render_square(screen, x, y, square_size, &color)
        }
    }
}

fn barycentric(
    x: f64,
    y: f64,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    x3: f64,
    y3: f64,
) -> (f64, f64, f64) {
    let lambda1 = ((y2 - y3) * (x - x3) + (x3 - x2) * (y - y3))
        / ((y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3));
    let lambda2 = ((y3 - y1) * (x - x3) + (x1 - x3) * (y - y3))
        / ((y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3));
    let lambda3 = 1.0 - lambda1 - lambda2;

    (lambda1, lambda2, lambda3)
}

pub fn render_triangle(
    screen: &mut Screen,
    x_0: usize,
    y_0: usize,
    x_1: usize,
    y_1: usize,
    x_2: usize,
    y_2: usize,
    color: &[u8; 4],
) {
    // TODO Fix error with pixels missing on the left edge of the triangle.
    // I think this is a problem with rounding and floating point arithemtic.

    let col_start = x_0.min(x_1).min(x_2);
    let row_start = y_0.min(y_1).min(y_2);
    if col_start >= screen.width() || row_start >= screen.height() {
        return;
    }

    let mut col_end = x_0.max(x_1).max(x_2);
    if col_end >= screen.width() {
        col_end = screen.width() - 1;
    }

    let mut row_end = y_0.max(y_1).max(y_2);
    if row_end >= screen.height() {
        row_end = screen.height() - 1;
    }

    for row in row_start..row_end {
        for col in col_start..col_end {
            let (l1, l2, l3) = barycentric(
                col as f64, row as f64, x_0 as f64, y_0 as f64, x_1 as f64, y_1 as f64, x_2 as f64,
                y_2 as f64,
            );

            if l1 < 0.0 || l1 > 1.0 {
                continue;
            }
            if l2 < 0.0 || l2 > 1.0 {
                continue;
            }
            if l3 < 0.0 || l3 > 1.0 {
                continue;
            }

            set_pixel(screen, col, row, color);
        }
    }
}

pub fn render_terrain(screen: &mut Screen, terrain: &Terrain, color: &[u8; 4]) {
    let square_size = screen.height() / (terrain.height - 1);

    for row in 0..terrain.height - 1 {
        for col in 0..terrain.width - 1 {
            let index = terrain.index_grid[row][col] as usize;
            let triangle_list = &terrain.cell_edges[index];

            let offset_x = (col * square_size) as f64;
            let offset_y = (row * square_size) as f64;

            for triangle in triangle_list.chunks(6) {
                let x_0 = (offset_x + triangle[0] * square_size as f64).round() as usize;
                let y_0 = (offset_y + triangle[1] * square_size as f64).round() as usize;
                let x_1 = (offset_x + triangle[2] * square_size as f64).round() as usize;
                let y_1 = (offset_y + triangle[3] * square_size as f64).round() as usize;
                let x_2 = (offset_x + triangle[4] * square_size as f64).round() as usize;
                let y_2 = (offset_y + triangle[5] * square_size as f64).round() as usize;

                render_triangle(screen, x_0, y_0, x_1, y_1, x_2, y_2, color);
            }
        }
    }
}

pub fn render_terrain_grid(screen: &mut Screen, terrain: &Terrain, color: &[u8; 4]) {
    let mut square_size = screen.height() / (terrain.height - 1);
    if screen.width() < screen.height() {
        square_size = screen.width() / (terrain.width - 1);
    }

    render_grid(screen, square_size, color);
}

pub fn render_character(
    screen: &mut Screen,
    c: char,
    x: usize,
    y: usize,
    scale: usize,
    color: &[u8; 4],
) {
    if c == ' ' || c == '\n' {
        return;
    }

    let symbol = get_char_symbol(c);
    let scaled_symbol = scale_symbol(symbol, scale);

    for row in 0..scaled_symbol.len() {
        for col in 0..scaled_symbol[0].len() {
            if scaled_symbol[row][col] > 0 {
                set_pixel(screen, x + col, y + row, color);
            }
        }
    }
}

pub fn render_string(
    screen: &mut Screen,
    input_str: &str,
    x: usize,
    y: usize,
    scale: usize,
    color: &[u8; 4],
) {
    let horizontal_spacing = 6;
    let vertical_spacing = 8;

    let mut x_offset = 0;
    let mut y_offset = 0;

    for c in input_str.chars() {
        render_character(
            screen,
            c,
            (x + horizontal_spacing * x_offset) * scale,
            (y + vertical_spacing * y_offset) * scale,
            scale,
            color,
        );

        x_offset += 1;

        if c == '\n' {
            y_offset += 1;
            x_offset = 0;
        }
    }
}
