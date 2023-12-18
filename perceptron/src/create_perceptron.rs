use rand::{thread_rng, Rng};

struct Perceptron {
  weights: Vec<f64>,
  learning_rate: f64
}