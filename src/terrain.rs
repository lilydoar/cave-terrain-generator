use log::debug;
use rand::Rng;

type ScalarField = Vec<Vec<f64>>;

pub struct Terrain {
    pub width: usize,
    pub height: usize,
    pub scalar_field: ScalarField,
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

        // Random initialization
        for row in 0..height {
            for col in 0..width {
                scalar_field[row][col] = rng.gen();
            }
        }

        Self {
            width,
            height,
            scalar_field,
        }
    }
}
