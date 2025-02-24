extern crate rand;
extern crate textplots;

use rand::Rng;
use textplots::{Chart, Plot, Shape};

struct LinearRegression {
    weights: f64,
    bias: f64,
}

impl LinearRegression {
    fn new() -> Self {
        Self {
            weights: rand::thread_rng().gen_range(-1.0..1.0),
            bias: rand::thread_rng().gen_range(-1.0..1.0),
        }
    }

    fn forward(&self, x: f64) -> f64 {
        self.weights * x + self.bias
    }

    fn train(&mut self, data: &Vec<(f64, f64)>, learning_rate: f64, epochs: usize) {
        for epoch in 0..epochs {
            let mut total_loss = 0.0;
            for (x, y) in data {
                let output = self.forward(*x);
                let error = output - *y;
                self.weights -= learning_rate * error * *x;
                self.bias -= learning_rate * error;
                total_loss += mse_loss(output, *y);
            }
            if epoch % 100 == 0 {
                println!("Epoch {}: Average Loss = {}", epoch, total_loss / data.len() as f64);
            }
        }
    }
}

fn generate_synthetic_data(size: usize) -> Vec<(f64, f64)> {
    let mut rng = rand::thread_rng();
    let mut data = Vec::with_capacity(size);

    for _ in 0..size {
        let x = rng.gen_range(-10.0..10.0);
        let y = 2.0 * x + 1.0 + rng.gen_range(-0.5..0.5); // Added noise
        data.push((x, y));
    }

    data
}

fn mse_loss(output: f64, target: f64) -> f64 {
    (output - target).powi(2)
}

fn main() {
    // 1. Generate Synthetic Data
    let data = generate_synthetic_data(100);

    // 2. Define the Model
    let mut model = LinearRegression::new();

    // 3. Train the Model
    model.train(&data, 0.01, 1000);

    // 4. Evaluate the Model
    println!("Final model: y = {}x + {}", model.weights, model.bias);

    // Test on unseen data
    let test_inputs = vec![-5.0, 0.0, 5.0, 10.0];
    for &x in &test_inputs {
        let y_true = 2.0 * x + 1.0;
        let y_pred = model.forward(x);
        println!("x = {}, y_true = {:.2}, y_pred = {:.2}", x, y_true, y_pred);
    }

    // Plot the results
    let points: Vec<(f32, f32)> = data.iter().map(|&(x, y)| (x as f32, y as f32)).collect();
    let model_line: Vec<(f32, f32)> = vec![
        (-10.0, model.forward(-10.0) as f32),
        (10.0, model.forward(10.0) as f32),
    ];

    Chart::new(180, 60, -10.0, 10.0)
        .lineplot(&Shape::Points(&points))
        .lineplot(&Shape::Lines(&model_line))
        .display();
}
