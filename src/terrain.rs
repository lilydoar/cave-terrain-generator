use log::debug;
use rand::Rng;

type ScalarField = Vec<Vec<f64>>;
type IndexField = Vec<Vec<u8>>;

pub struct Terrain {
    pub width: usize,
    pub height: usize,
    pub scalar_field: ScalarField,
    pub index_grid: IndexField,
    pub cell_edges: Vec<Vec<f64>>,
}

impl Terrain {
    pub fn new(width: usize, height: usize) -> Self {
        debug!("creating terrain");
        debug!("width: {} height: {}", width, height);

        let mut rng = rand::thread_rng();

        let mut scalar_field = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row = Vec::<f64>::with_capacity(width);
            for _ in 0..width {
                row.push(0.0);
            }
            scalar_field.push(row);
        }

        for row in 0..height {
            for col in 0..width {
                // scalar_field[row][col] = rng.gen();
                scalar_field[row][col] = 1.0;
            }
        }

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

        let mut new_terrain = Self {
            width,
            height,
            scalar_field,
            index_grid,
            cell_edges,
        };

        Self::construct_index_grid(&mut new_terrain, 0.5);

        return new_terrain;
    }

    pub fn modify_scalar_field(&mut self, row: usize, col: usize, new_scalar: f64) {
        debug_assert!(new_scalar >= 0.0 && new_scalar <= 1.0);

        self.scalar_field[row][col] = new_scalar;
        self.construct_index_grid(0.5);
    }

    fn construct_index_grid(&mut self, threshold: f64) {
        debug_assert!(threshold >= 0.0 && threshold <= 1.0);

        let thresholded_field = threshold_field(&self.scalar_field, 0.5);
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

// 0: empty
// 1: (0.0, 0.5), (0.0, 1.0), (0.5, 1.0)
// 2: (1.0, 0.5), (0.5, 1.0), (1.0, 1.0)
// 3: (0.0, 0.5), (0.0, 1.0), (1.0, 1.0), (0.0, 0.5), (1.0, 1.0), (1.0, 0.5)
// 4: (0.5, 0.0), (1.0, 0.5), (1.0, 0.0)
// 5: (0.0, 0.0), (0.0, 0.5), (0.5, 0.0), (0.5, 1.0), (1.0, 1.0), (1.0, 0.5)
// 6: (0.5, 0.0), (0.5, 1.0), (1.0, 1.0), (0.5, 0.0), (1.0, 1.0), (1.0, 0.0)
// 7: (0.0, 1.0), (1.0, 1.0), (1.0, 0.0), (0.0, 0.5), (0.0, 1.0), (1.0, 0.0), (0.0, 0.5), (1.0, 0.0), (0.5, 0.0)
// 8: (0.0, 0.0), (0.0, 0.5), (0.5, 0.0)
// 9: (0.0, 0.0), (0.0, 1.0), (0.5, 1.0), (0.0, 0.0), (0.5, 1.0), (0.5, 0.0)
// 10: (0.0, 0.5), (0.0, 1.0), (0.5, 1.0), (0.5, 0.0), (1.0, 0.5), (1.0, 0.0)
// 11: (0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (0.0, 0.0), (1.0, 1.0), (1.0, 0.5), (0.0, 0.0), (1.0, 0.5), (0.5, 0.0)
// 12: (0.0, 0.0), (0.0, 0.5), (1.0, 0.5), (0.0, 0.0), (1.0, 0.5), (1.0, 0.0)
// 13: (0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (0.0, 1.0), (0.5, 1.0), (1.0, 0.5), (0.0, 1.0), (1.0, 0.5), (1.0, 0.0)
// 14: (0.0, 0.0), (1.0, 1.0), (1.0, 0.0), (0.0, 0.0), (0.0, 0.5), (0.5, 1.0), (0.0, 0.0), (0.5, 1.0), (1.0, 1.0)
// 15: (0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (0.0, 0.0), (1.0, 1.0), (1.0, 0.0)
