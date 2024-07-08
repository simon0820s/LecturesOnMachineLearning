mod activation_functions;
mod create_perceptron;
use create_perceptron::Perceptron;

fn main() {
    let epochs = 20;
    let mut p = Perceptron::new(2, 0.1);
    println!("Init weights: {:?}", p.get_weights());
    println!("Init bias: {:?}", p.get_bias());
    let training_data: &[(&[f64], f64)] = &[
        (&[0.0, 0.0], 0.0),
        (&[0.0, 1.0], 1.0),
        (&[1.0, 0.0], 1.0),
        (&[1.0, 1.0], 1.0),
    ];
    p.train(&training_data, epochs);
    println!("Trained weights: {:?}", p.get_weights());
    println!("Trained bias: {:?}", p.get_bias());

    println!("Predicción para [0.0, 0.0]: {}", p.predict(&[0.0, 0.0]));
    println!("Predicción para [0.0, 1.0]: {}", p.predict(&[0.0, 1.0]));
    println!("Predicción para [1.0, 0.0]: {}", p.predict(&[1.0, 0.0]));
    println!("Predicción para [1.0, 1.0]: {}", p.predict(&[1.0, 1.0]));
}
