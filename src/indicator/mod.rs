pub mod sma;

pub trait Indicator {
    fn ready(&self) -> bool;
    fn load(&mut self, data: f64);
    fn current(&mut self) -> f64;
}