mod layer;
mod activation_functions;
mod perceptron;
use activation_functions::activation_functions::step;
use layer::{Layer, LayerParams};

fn main() {
    let layer_params = LayerParams::new(3, 0.1, step, 4);
    let layer = Layer::new(layer_params);
}
