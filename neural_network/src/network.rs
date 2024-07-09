use std::usize;

use crate::layer::Layer;


pub struct Network {
  input_size: usize,
  layers: Vec<Layer>,
}

impl Network {
  pub fn new(input_size: usize) -> Network {
    Network {
      input_size,
      layers: Vec::new(),
    }
  }

  pub fn add_layer(&mut self, layer: Layer) {
    self.layers.push(layer);
  }
}