mod perceptron;
use perceptron::Perceptron;
fn main() {
    let perceptron = Perceptron::new(0.01);
    println!("w: {} lr: {}", perceptron.get_weight(), perceptron.get_learning_rate());
}
