
use ndarray::{Array2, Axis};
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
use std::f64::consts::E;

/// Function to normalize data
fn normalize_data(data: &Array2<f64>) -> Array2<f64> {
    let max = data.iter().cloned().fold(f64::MIN, f64::max);
    let min = data.iter().cloned().fold(f64::MAX, f64::min);
    (data - min) / (max - min)
}

/// Function to calculate the sigmoid activation
fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + E.powf(-x))
}

/// Function to train a simple linear model using gradient descent
pub fn train_model(data: Array2<f64>, target: Array2<f64>, learning_rate: f64, epochs: usize) {
    // Normalize the data
    let data = normalize_data(&data);

    // Initialize weights randomly
    let mut weights = Array2::random((data.ncols(), 1), Uniform::new(0.0, 1.0));

    for epoch in 0..epochs {
        // Calculate predictions: data * weights
        let predictions = data.dot(&weights);

        // Compute the error
        let error = &target - &predictions;

        // Update weights using gradient descent
        let gradient = data.t().dot(&error) * learning_rate;
        weights = weights + gradient;

        // Optionally, print the error for every 100 epochs
        if epoch % 100 == 0 {
            let mean_squared_error = error.mapv(|e| e.powi(2)).mean().unwrap_or(0.0);
            println!("Epoch {}: Mean Squared Error = {}", epoch, mean_squared_error);
        }
    }

    println!("Training complete. Final weights: {:?}", weights);
}
