use prettytable::{Cell, Row, Table};
use std::usize;

use crate::layer::Layer;

pub struct Network {
    input_size: usize,
    layers: Vec<Layer>,
    learning_rate: f64,
}

impl Network {
    pub fn new(input_size: usize, learning_rate: f64) -> Network {
        Network {
            input_size,
            layers: Vec::new(),
            learning_rate
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

    pub fn train(&mut self, train_data: &[(&[f64], f64)], epochs: i32) {
        for _ in 0..epochs {
            for data in train_data {
                let prediction = self.predict(data.0);
                let error = data.1 - prediction[0];
            }
        }
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
        table.add_row(Row::new(vec![Cell::new("Input size:"), Cell::new(&format!("{}", self.input_size))]));
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
