use std::{borrow::Borrow, cell::RefCell, error::Error};
use std::collections::HashMap;


use crate::datasource::DataSource;

use super::Broker;

pub struct Backtest {
    cash: f64,
    positions: HashMap<String, i64>,
}


impl Backtest {
    pub fn new(initial_balance: f64) -> Backtest {
        return Backtest {
            cash: initial_balance,
            positions: HashMap::new()
        };
    }
}

impl Broker for Backtest {
    fn connect(&mut self) -> Result<(), Box<dyn Error>>{
        return Ok(());
    }

    fn disconnect(&mut self) {

    }

    fn cash_available(&self) -> f64 {
        return self.cash;
    }

    fn positions(&self) -> HashMap<String, i64> {
        return self.positions.clone();
    }

    fn order_stock<T: DataSource>(&mut self, symbol: String, quantity: i64, datasource: &T) -> bool {
        if quantity == 0 {
            return true;
        }

        let price = match datasource.current_price(symbol.clone()) {
            Some(p) => p,
            None => { return false; }
        };

        let total_cost = price * quantity as f64;
        self.cash -= total_cost;
        if let Some(current_hold) = self.positions.get_mut(&symbol) {
            // symbol is held (long or short), so update quantity
            *current_hold += quantity;
        } else {
            // symbol is not held
            self.positions.insert(symbol.to_owned(), quantity);
        }

        return true;
    }

    fn order_portfolio<T: DataSource>(&mut self, allocations: &HashMap<&'static str, i64>, datasource: &T) -> bool {
        return true;
    }
}