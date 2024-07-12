mod activation_functions;
mod layer;
mod network;
mod perceptron;
use activation_functions::activation_functions::{relu, step};
use layer::{Layer, LayerParams};

fn main() {
    let train_data: &[(&[f64], f64)] = &[
        (&[0.0, 0.0], 0.0),
        (&[1.0, 0.0], 0.0),
        (&[0.0, 1.0], 0.0),
        (&[1.0, 1.0], 1.0),
    ];
    let epochs = 4;
    let layer1_params = LayerParams::new(2, 2, relu);
    let layer2_params = LayerParams::new(2, 1, step);
    let layer1 = Layer::new(&layer1_params);
    let layer2 = Layer::new(&layer2_params);
    let mut network = network::Network::new(2, 0.0001);
    network.add_layer(layer1);
    network.add_layer(layer2);
    network.get_summary();
    println!(
        "Prediction before train: {:?}",
        network.predict(vec![1., 1.])
    );
    network.train(train_data, epochs);
    println!(
        "Prediction after train: {:?}",
        network.predict(vec![1., 1.])
    );
}
