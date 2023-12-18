use rand::{thread_rng, Rng};

struct Perceptron {
  weights: Vec<f64>,
  learning_rate: f64
}

impl Perceptron {

  fn new(input_size: usize, learning_rate: f64) -> Perceptron {
    let mut rng = thread_rng();
    let weights: Vec<f64> = (0..input_size).map(|_| rng.gen_range(-1.0..1.0)).collect();
    Perceptron {
      weights,
      learning_rate,
    }
  }

  fn predict(&self, inputs: &[f64]) -> f64 {
    let mut sum = 0.0;
    for i in 0..inputs.len() {
      sum += inputs[i] * self.weights[i];
    }

    if sum > 0.0 {
      1.0
    } else {
      -1.0
    }
  }
}