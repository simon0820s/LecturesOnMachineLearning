use rand::{thread_rng, Rng};
pub struct Perceptron {
  weight: f64,
  bias: f64,
  learning_rate: f64
}

impl Perceptron {

    pub fn new(learning_rate: f64) -> Perceptron {
      let mut rng = thread_rng();
      let weight: f64 = rng.gen_range(0.0..1.0);
      let bias: f64 = rng.gen_range(0.0..1.0);
      Perceptron {
        weight,
        bias,
        learning_rate
      }
    }

    pub fn predict(&self, input: f64) -> f64 {
      input * self.weight + self.bias
    }

    pub fn train(&mut self, train_data: [f64; 2]){
      let guess = self.predict(train_data[0]);
      let error = train_data[1] - guess;

      if error != 0.0 {
        self.weight += self.learning_rate * error * train_data[0];
        self.bias += self.learning_rate * error
      }
    }

    pub fn get_weight(&self) -> f64 {
      self.weight
    }

    pub fn get_learning_rate(&self) -> f64 {
      self.learning_rate
    }

    pub fn get_bias(&self) -> f64 {
      self.bias
    }

}