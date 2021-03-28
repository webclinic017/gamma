use std::error::Error;

pub mod csv;

pub trait DataSource {
    fn connect(&mut self) -> Result<bool, Box<dyn Error>>;
    fn disconnect(&mut self);
    fn drip(&mut self) -> Option<f64>; // None on end of stream
}