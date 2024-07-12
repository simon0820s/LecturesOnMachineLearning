use crate::perceptron::Perceptron;

pub struct LayerParams {
    input_size: usize,
    neurons_amount: usize,
    activation_function: fn(f64) -> f64,
}

impl LayerParams {
    pub fn new(
        input_size: usize,
        neurons_amount: usize,
        activation_function: fn(f64) -> f64,
    ) -> LayerParams {
        LayerParams {
            neurons_amount,
            activation_function,
            input_size,
        }
    }
}
pub struct Layer {
    perceptrons: Vec<Perceptron>,
    layer_type: String,
}

impl Layer {
    pub fn new(layer_params: &LayerParams) -> Layer {
        Layer {
            perceptrons: (0..layer_params.neurons_amount)
                .map(|_| Perceptron::new(layer_params.input_size, layer_params.activation_function))
                .collect(),
            layer_type: "Dense".to_string(),
        }
    }

    pub fn predict(&self, inputs: &[f64]) -> Vec<f64> {
        self.perceptrons
            .iter()
            .map(|perceptron| perceptron.predict(inputs))
            .collect()
    }

    pub fn get_layer_type(&self) -> &str {
        &self.layer_type
    }

    pub fn get_layer_output(&self) -> usize {
        self.perceptrons.len()
    }
}
