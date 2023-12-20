use rand::{thread_rng, Rng};
pub struct Perceptron {
  weight: f64,
  learning_rate: f64
}

impl Perceptron {

    pub fn new(learning_rate: f64) -> Perceptron {
      let mut rng = thread_rng();
      let weight: f64 = rng.gen_range(0.0..1.0);
      Perceptron {
        weight,
        learning_rate
      }
    }

    pub fn get_weight(&self) -> f64 {
      self.weight
    }

    pub fn get_learning_rate(&self) -> f64 {
      self.learning_rate
    }

}