mod create_perceptron;
use create_perceptron::Perceptron;

fn main() {
    let mut p = Perceptron::new(2, 0.1);
    println!("Init weights: {:?}", p.get_weights());
    let training_data = vec![(vec![0.0, 0.0], 0.0), (vec![0.0, 1.0], 1.0), (vec![1.0, 0.0], 1.0), (vec![1.0, 1.0], 1.0)];

    for _ in 0..1000 {
        for &(ref inputs, target) in &training_data {
            p.train(&inputs, target)
        }
    }

    println!("Trained weights: {:?}", p.get_weights());

    println!("Predicción para [0.0, 0.0]: {}", p.predict(&[0.0, 0.0]));
    println!("Predicción para [0.0, 1.0]: {}", p.predict(&[0.0, 1.0]));
    println!("Predicción para [1.0, 0.0]: {}", p.predict(&[1.0, 0.0]));
    println!("Predicción para [1.0, 1.0]: {}", p.predict(&[1.0, 1.0]));
}