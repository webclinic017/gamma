use std::{collections::HashMap, error::Error};

pub mod backtest;
pub mod etrade;

pub trait Broker {
    fn connect(&mut self) -> Result<(), Box<dyn Error>>;
    fn disconnect(&mut self);
    fn cash_available(&self) -> f64;
    fn positions(&self) -> HashMap<String, i64>;
    fn order_stock(&mut self, symbol: String, quantity: i64) -> bool;
    fn order_portfolio(&mut self, allocations: &HashMap<&'static str, i64>) -> bool;
}

//pub trait BrokerSlice {}