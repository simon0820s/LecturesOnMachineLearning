use prettytable::{Cell, Row, Table};
use std::usize;

use crate::layer::Layer;

pub struct Network {
    input_size: usize,
    layers: Vec<Layer>,
    cost_function: fn(&[f64], &[f64]) -> f64,
}

impl Network {
    pub fn new(input_size: usize, cost_function: fn(&[f64], &[f64]) -> f64) -> Network {
        Network {
            input_size,
            layers: Vec::new(),
            cost_function,
        }
    }

    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    pub fn predict(&self, inputs: &[f64]) -> Vec<f64> {
        let mut outputs = inputs.to_vec();
        for (i, layer) in self.layers.iter().enumerate() {
            println!("Layer: {}", i);
            outputs = layer.predict(&outputs);
            println!("Layer: {} outPuts: {:?}", i, outputs);
        }
        outputs
    }

    pub fn train(&mut self, train_data: &[(&[f64], f64)], epochs: i32, learning_rate: f64) {
        for _ in 0..epochs {
            for (inputs, expected_outputs) in train_data {
                let prediction = self.predict(&inputs);
                let error =
                    (self.cost_function)(&[expected_outputs - prediction[0]], &[*expected_outputs]);
                self.back_propagate(inputs, &prediction, expected_outputs, learning_rate);
            }
        }
    }

    fn back_propagate(
        &mut self,
        inputs: &[f64],
        prediction: &[f64],
        expected_output: &f64,
        learning_rate: f64,
    ) {
        
    }

    pub fn get_summary(&self) {
        let mut table = Table::new();

        // Agrega la fila de encabezado
        table.add_row(Row::new(vec![
            Cell::new("Layer (type)"),
            Cell::new("Output Shape"),
        ]));

        for (_, layer) in self.layers.iter().enumerate() {
            table.add_row(Row::new(vec![
                Cell::new(&format!("{}", layer.get_layer_type())),
                Cell::new(&format!("(None, {})", layer.get_layer_output())),
            ]));
        }

        // Agrega la fila de total de parámetros
        table.add_row(Row::new(vec![
            Cell::new("Input size:"),
            Cell::new(&format!("{}", self.input_size)),
        ]));
        table.add_row(Row::new(vec![Cell::new("Total params:"), Cell::new("0")]));
        table.add_row(Row::new(vec![
            Cell::new("Trainable params:"),
            Cell::new("0"),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("Non-trainable params:"),
            Cell::new("0"),
        ]));

        // Imprime la tabla
        table.printstd();
    }
}
