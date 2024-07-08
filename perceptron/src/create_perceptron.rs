use rand::{thread_rng, Rng};
pub struct Perceptron {
    weights: Vec<f64>,
    bias: f64,
    learning_rate: f64,
    activation_function: fn(f64) -> f64
}

impl Perceptron {
    pub fn new(input_size: usize, learning_rate: f64, activation_function: fn(f64) -> f64) -> Perceptron {
        let mut rng = thread_rng();
        let weights: Vec<f64> = (0..input_size).map(|_| rng.gen_range(-1.0..1.0)).collect();
        let bias: f64 = rng.gen_range(-1.0..1.0);
        Perceptron {
            weights,
            bias,
            learning_rate,
            activation_function
        }
    }

    pub fn train(&mut self, train_data: &[(&[f64], f64)], epochs: i32) {
        for _ in 0..epochs {
            for data in train_data {
                self.train_single(data);
            }
        }
    }

    fn train_single(&mut self, train_data: &(&[f64], f64)) {
        let (inputs, target) = train_data;
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

        (self.activation_function)(summation)
    }

    pub fn get_weights(&self) -> &Vec<f64> {
        &self.weights
    }

    pub fn get_bias(&self) -> f64 {
        self.bias
    }
}
