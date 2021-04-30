use std::{collections::HashMap, error::Error};

use crate::datasource::DataSource;

pub mod backtest;
pub mod etrade;

pub trait Broker {
    fn connect(&mut self) -> Result<(), Box<dyn Error>>;
    fn disconnect(&mut self);
    fn cash_available(&self) -> f64;
    fn positions(&self) -> HashMap<String, i64>;
    fn order_stock<T: DataSource>(&mut self, symbol: String, quantity: i64, datasource: &T) -> bool;
    fn order_portfolio<T: DataSource>(&mut self, allocations: &HashMap<&'static str, i64>, datasource: &T) -> bool;
}

//pub trait BrokerSlice {}