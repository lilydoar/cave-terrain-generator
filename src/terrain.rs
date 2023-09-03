use log::debug;
use rand::Rng;

type ScalarField = Vec<Vec<f64>>;

pub struct Terrain {
    pub width: usize,
    pub height: usize,
    pub scalar_field: ScalarField,
    pub thresholded_field: ScalarField,
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

        let thresholded_field = threshold(&scalar_field, 0.5);

        Self {
            width,
            height,
            scalar_field,
            thresholded_field,
        }
    }
}

fn threshold(scalar_field: &ScalarField, threshold: f64) -> ScalarField {
    let mut new_field = Vec::with_capacity(scalar_field.len());
    for _ in 0..scalar_field.len() {
        let mut row = Vec::<f64>::with_capacity(scalar_field[0].len());
        for _ in 0..scalar_field[0].len() {
            row.push(0.0);
        }
        new_field.push(row);
    }

    for row in 0..scalar_field.len() {
        for col in 0..scalar_field[0].len() {
            if scalar_field[row][col] > threshold {
                new_field[row][col] = 1.0;
            }
        }
    }

    new_field
}
