pub mod activation_functions {

    pub fn step(x: f64) -> f64 {
        if x > 0.0 {
            1.0
        } else {
            0.0
        }
    }

    pub fn relu(x: f64) -> f64 {
        0.0f64.max(x)
    }
}