mod perceptron;
use perceptron::Perceptron;

const TRAIN_DATA: [[f64; 2]; 10] = [
    [0.0, 32.0],
    [5.0, 41.0],
    [10.0, 50.0],
    [15.0, 59.0],
    [20.0, 68.0],
    [25.0, 77.0],
    [30.0, 86.0],
    [35.0, 95.0],
    [40.0, 104.0],
    [45.0, 113.0],
];

fn main() {
    println!("True data => {:?}", TRAIN_DATA);
    println!();
    let mut perceptron = Perceptron::new(0.0001);
    println!("Before train => ");
    println!(
        "w: {} lr: {} b: {}",
        perceptron.get_weight(),
        perceptron.get_learning_rate(),
        perceptron.get_bias()
    );
    println!(
        "Predictions before train: [0, {}] [40, {}], [-2, {}]",
        perceptron.predict(0.0),
        perceptron.predict(40.0),
        perceptron.predict(-2.0)
    );
    println!();
    for _ in 0..100000 {
        for i in 0..TRAIN_DATA.len() {
            perceptron.train(TRAIN_DATA[i]);
        }
    }

    println!("After train => ");
    println!(
        "w: {} lr: {} b: {}",
        perceptron.get_weight(),
        perceptron.get_learning_rate(),
        perceptron.get_bias()
    );
    println!(
        "Predictions after train: [0, {}] [40, {}], [-2, {}]",
        perceptron.predict(0.0),
        perceptron.predict(40.0),
        perceptron.predict(-2.0)
    )
}
