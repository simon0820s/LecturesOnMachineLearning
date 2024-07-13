pub mod functions{

    pub fn step(x: f64) -> f64 {
        if x > 0.0 {
            1.0
        } else {
            0.0
        }
    }

    pub fn derivated_step(x: f64) -> f64 {
        0.0
    }

    pub fn relu(x: f64) -> f64 {
        0.0f64.max(x)
    }

    pub fn derivated_relu(x: f64) -> f64 {
        if x > 0.0 {
            1.0
        } else {
            0.0
        }
    }

    pub fn mean_squared_error(y_pred: &[f64], y_true: &[f64]) -> f64 {
        y_true
            .iter()
            .zip(y_pred.iter())
            .map(|(y, y_hat)| (y - y_hat).powi(2))
            .sum::<f64>()
            / y_true.len() as f64
    }
}
