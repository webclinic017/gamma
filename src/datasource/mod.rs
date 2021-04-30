use std::error::Error;

pub mod csv;

pub trait DataSource {
    fn connect(&mut self) -> Result<bool, Box<dyn Error>>;
    fn current_price(&self, symbol: String) -> Option<f64>;
    fn end(&self) -> bool;
    fn step(&mut self);
}