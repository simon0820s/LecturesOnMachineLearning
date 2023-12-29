use crate::activation_functions::activation_functions;
use rand::{thread_rng, Rng};
pub struct Layer {
  activation_function: fn(f64) -> f64 ,
  bias: Vec<f64>,
  weights: Vec<Vec<f64>>
}

impl Layer {

  pub fn new(n_neurons_a: u32 , n_neurons_c: u32 ) -> Layer {
    let mut rng = thread_rng();
    let activation_function = activation_functions::step; 
    let bias: Vec<f64> = (0..n_neurons_c).map(|_| rng.gen_range(-1.0..1.0)).collect();
    let weights: Vec<Vec<f64>> = (0..n_neurons_a)
    .map(|_| (0..n_neurons_c)
    .map(|_| rng.gen_range(-1.0..1.0))
    .collect()).collect();

    Layer {
      activation_function,
      bias,
      weights
    }
  }

  pub fn getBias(&self) -> &Vec<f64> {
    &self.bias
  }

  pub fn getWeights(&self) -> &Vec<Vec<f64>> {
    &self.weights
  }

}