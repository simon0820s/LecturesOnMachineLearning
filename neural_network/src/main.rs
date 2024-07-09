mod layer;
mod activation_functions;
mod perceptron;
mod network;
use activation_functions::activation_functions::step;
use layer::{Layer, LayerParams};

fn main() {
    let layer1_params = LayerParams::new(2, 0.1, step, 1);
    let layer2_params = LayerParams::new(2, 0.1, step, 1);
    let layer1 = Layer::new(layer1_params);
    let layer2 = Layer::new(layer2_params);
    let mut network = network::Network::new(2);
    println!("layer 1 weights => {:?}", layer1.get_weights());
    println!("layer 2 weights => {:?}", layer2.get_weights());
    network.add_layer(layer1);
    network.add_layer(layer2);
}