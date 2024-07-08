use crate::perceptron::Perceptron;

pub struct LayerParams {
  neurons_amount: usize,
  learning_rate: f64,
  activation_function: fn(f64) -> f64,
  input_size: usize,
}

impl LayerParams {
    pub fn new(neurons_amount: usize, learning_rate: f64, activation_function: fn(f64) -> f64, input_size: usize) -> LayerParams {
      LayerParams {
        neurons_amount,
        learning_rate,
        activation_function,
        input_size
      }
    } 
}
pub struct Layer {
  perceptrons: Vec<Perceptron>,
}

impl Layer {
    pub fn new(layer_params: LayerParams) -> Layer {
      Layer {
        perceptrons: (0..layer_params.neurons_amount).map(|_| Perceptron::new(layer_params.input_size, layer_params.learning_rate, layer_params.activation_function)).collect()
      }
    }
}
