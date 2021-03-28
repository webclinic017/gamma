pub mod teststrat;

use std::collections::HashMap;

pub trait Strategy {
    fn process(&mut self, data: f64) -> HashMap<&'static str, f64>;
    //fn process(data: Vec<f64>) -> HashMap<&str, f64>;
}