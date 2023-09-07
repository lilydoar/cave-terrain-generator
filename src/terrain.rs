use log::debug;
use noise::{NoiseFn, Perlin};
// use rand::Rng;

type ScalarField = Vec<Vec<f64>>;
type IndexField = Vec<Vec<u8>>;

pub struct Terrain {
    pub width: usize,
    pub height: usize,
    pub scalar_field: ScalarField,
    threshold: f64,
    pub index_grid: IndexField,
    pub cell_edges: Vec<Vec<f64>>,
}

impl Terrain {
    pub fn new(width: usize, height: usize) -> Self {
        debug!("creating terrain");
        debug!("width: {} height: {}", width, height);

        // let mut rng = rand::thread_rng();

        let mut scalar_field = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row = Vec::<f64>::with_capacity(width);
            for _ in 0..width {
                row.push(0.0);
            }
            scalar_field.push(row);
        }

        let perlin = Perlin::new(1);

        for row in 0..height {
            for col in 0..width {
                // scalar_field[row][col] = rng.gen();
                // scalar_field[row][col] = 1.0;
                scalar_field[row][col] =
                    perlin.get([col as f64 + 0.5, row as f64 + 0.5]) * 0.5 + 0.5;

                // let distance_from_center = ((row * row + col * col) as f64).sqrt();
            }
        }
        debug!("scalar_field: {:?}", scalar_field);

        let mut index_grid = Vec::with_capacity(height - 1);
        for _ in 0..height - 1 {
            let mut row = Vec::<u8>::with_capacity(width - 1);
            for _ in 0..width - 1 {
                row.push(0);
            }
            index_grid.push(row);
        }

        let cell_edges = vec![
            vec![],
            vec![0.0, 0.5, 0.0, 1.0, 0.5, 1.0],
            vec![1.0, 0.5, 0.5, 1.0, 1.0, 1.0],
            vec![0.0, 0.5, 0.0, 1.0, 1.0, 1.0, 0.0, 0.5, 1.0, 1.0, 1.0, 0.5],
            vec![0.5, 0.0, 1.0, 0.5, 1.0, 0.0],
            vec![0.0, 0.5, 0.0, 1.0, 0.5, 1.0, 0.5, 0.0, 1.0, 0.5, 1.0, 0.0],
            vec![0.5, 0.0, 0.5, 1.0, 1.0, 1.0, 0.5, 0.0, 1.0, 1.0, 1.0, 0.0],
            vec![
                0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 0.5, 1.0, 0.0,
                0.5, 0.0,
            ],
            vec![0.0, 0.0, 0.0, 0.5, 0.5, 0.0],
            vec![0.0, 0.0, 0.0, 1.0, 0.5, 1.0, 0.0, 0.0, 0.5, 1.0, 0.5, 0.0],
            vec![0.0, 0.0, 0.0, 0.5, 0.5, 0.0, 0.5, 1.0, 1.0, 1.0, 1.0, 0.5],
            vec![
                0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.5, 0.0, 0.0, 1.0, 0.5,
                0.5, 0.0,
            ],
            vec![0.0, 0.0, 0.0, 0.5, 1.0, 0.5, 0.0, 0.0, 1.0, 0.5, 1.0, 0.0],
            vec![
                0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.5, 1.0, 1.0, 0.5, 0.0, 1.0, 1.0, 0.5,
                1.0, 0.0,
            ],
            vec![
                0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 0.0, 0.0, 0.5, 1.0,
                1.0, 1.0,
            ],
            vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0],
        ];

        let threshold = 0.33;
        debug_assert!(threshold >= 0.0 && threshold <= 1.0);

        let mut new_terrain = Self {
            width,
            height,
            scalar_field,
            threshold,
            index_grid,
            cell_edges,
        };

        Self::construct_index_grid(&mut new_terrain);

        return new_terrain;
    }

    pub fn modify_scalar_field(&mut self, row: usize, col: usize, new_scalar: f64) {
        debug_assert!(new_scalar >= 0.0 && new_scalar <= 1.0);

        self.scalar_field[row][col] = new_scalar;
        self.construct_index_grid();
    }

    fn construct_index_grid(&mut self) {
        let thresholded_field = threshold_field(&self.scalar_field, self.threshold);
        debug!("thresholded_field: {:?}", thresholded_field);

        for row in 0..self.height - 1 {
            for col in 0..self.width - 1 {
                // Compose 4 bits at corners of each cell to build a binary index
                // Start top left and rotate clockwise
                // Build most significant bit to least significant bit
                let mut index = 0;
                index |= thresholded_field[row][col] << 3;
                index |= thresholded_field[row][col + 1] << 2;
                index |= thresholded_field[row + 1][col + 1] << 1;
                index |= thresholded_field[row + 1][col] << 0;

                self.index_grid[row][col] = index;
            }
        }
        debug!("index_grid: {:?}", self.index_grid);
    }
}

fn threshold_field(scalar_field: &ScalarField, threshold: f64) -> IndexField {
    let mut new_field = Vec::with_capacity(scalar_field.len());
    for _ in 0..scalar_field.len() {
        let mut row = Vec::<u8>::with_capacity(scalar_field[0].len());
        for _ in 0..scalar_field[0].len() {
            row.push(0);
        }
        new_field.push(row);
    }

    for row in 0..scalar_field.len() {
        for col in 0..scalar_field[0].len() {
            if scalar_field[row][col] > threshold {
                new_field[row][col] = 1;
            }
        }
    }

    new_field
}

// Perlin noise implementation
// https://en.wikipedia.org/wiki/Perlin_noise

// fn interpolate(a0: f64, a1: f64, w: f64) -> f64 {
//     (a1 - a0) * w + a0
// }
//
// fn cubic_interpolation(a0: f64, a1: f64, w: f64) -> f64 {
//     (a1 - a0) * (3.0 - w * 2.0) * w * w + a0
// }
// fn random_gradient(x: i32, y: i32) -> (f64, f64) {
//     // let w: usize = 8 * std::mem::size_of::<usize>();
//     let w = 8 * std::mem::size_of::<u32>() as u32;
//     let s = w / 2;
//
//     let mut a = x as u32;
//     let mut b = y as u32;
//
//     // Problems with overflow in this series of calculations
//     a *= 3284147443;
//     b ^= a << s | a >> w - s;
//     b *= 1911520717;
//     a ^= b << s | b >> w - s;
//     a *= 2048419325;
//
//     let random = a as f64 * (3.14159265 / !(!0 >> 1) as f64);
//     let gradient_x = random.cos();
//     let gradient_y = random.sin();
//
//     (gradient_x, gradient_y)
// }
//
// fn dot_grid_gradient(ix: i64, iy: i64, x: f64, y: f64) -> f64 {
//     let (gradient_x, gradient_y) = random_gradient(ix as i32, iy as i32);
//
//     let dx = x - gradient_x as f64;
//     let dy = y - gradient_y as f64;
//
//     dx * gradient_x + dy * gradient_y
// }
//
// fn perlin(x: f64, y: f64) -> f64 {
//     let x0 = x.floor() as i64;
//     let x1 = x0 + 1;
//     let y0 = y.floor() as i64;
//     let y1 = y0 + 1;
//
//     let sx = x - x0 as f64;
//     let sy = y - y0 as f64;
//
//     let n0 = dot_grid_gradient(x0, y0, x, y);
//     let n1 = dot_grid_gradient(x1, y0, x, y);
//     let ix0 = interpolate(n0, n1, sx);
//
//     let n0 = dot_grid_gradient(x0, y1, x, y);
//     let n1 = dot_grid_gradient(x1, y1, x, y);
//     let ix1 = interpolate(n0, n1, sx);
//
//     interpolate(ix0, ix1, sy) * 0.5 + 0.5
// }
