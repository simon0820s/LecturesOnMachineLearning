use crate::activation_functions::activation_functions;
use rand::{thread_rng, Rng};
pub struct Perceptron {
    weights: Vec<f64>,
    learning_rate: f64,
}

impl Perceptron {

    pub fn new(input_size: usize, learning_rate: f64) -> Perceptron {
        let mut rng = thread_rng();
        let weights: Vec<f64> = (0..input_size).map(|_| rng.gen_range(-1.0..1.0)).collect();
        Perceptron {
            weights,
            learning_rate,
        }
    }

    pub fn train(&mut self, inputs: &[f64], target: f64) {
        let guess = self.predict(inputs);
        let error = target - guess;

        if error != 0.0 {
            for i in 0..inputs.len() {
                self.weights[i] += self.learning_rate * error * inputs[i]
            }
        }
    }

    pub fn predict(&self, inputs: &[f64]) -> f64 {
        let mut summation = 0.0;
        for i in 0..inputs.len() {
            summation += inputs[i] * self.weights[i];
        }

        activation_functions::step(summation)
    }

    pub fn get_weights(&self) -> &Vec<f64> {
        &self.weights
    }
}
