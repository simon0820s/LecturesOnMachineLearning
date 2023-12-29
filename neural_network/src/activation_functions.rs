pub mod activation_functions {

    pub fn step(summation: f64) -> f64 {
        if summation > 0.0 {
            1.0
        } else {
            0.0
        }
    }
}