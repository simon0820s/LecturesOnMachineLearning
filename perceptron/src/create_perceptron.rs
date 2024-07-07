use crate::activation_functions::activation_functions;
use rand::{thread_rng, Rng};
pub struct Perceptron {
    weights: Vec<f64>,
    bias: f64,
    learning_rate: f64,
}

impl Perceptron {
    pub fn new(input_size: usize, learning_rate: f64) -> Perceptron {
        let mut rng = thread_rng();
        let weights: Vec<f64> = (0..input_size).map(|_| rng.gen_range(-1.0..1.0)).collect();
        let bias: f64 = rng.gen_range(-1.0..1.0);
        Perceptron {
            weights,
            bias,
            learning_rate,
        }
    }

    pub fn train(&mut self, inputs: &[f64], target: f64) {
        let guess = self.predict(inputs);
        let error = target - guess;

        if error != 0.0 {
            println!("updating weights");
            for i in 0..inputs.len() {
                self.weights[i] += self.learning_rate * error * inputs[i];
                self.bias += self.learning_rate * error;
            }
        }
    }

    pub fn predict(&self, inputs: &[f64]) -> f64 {
        let mut summation = 0.0;
        for i in 0..inputs.len() {
            summation += inputs[i] * self.weights[i] + self.bias;
        }

        activation_functions::step(summation)
    }

    pub fn get_weights(&self) -> &Vec<f64> {
        &self.weights
    }

    pub fn get_bias(&self) -> f64 {
        self.bias
    }
}
