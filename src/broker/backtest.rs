use std::error::Error;
use std::collections::HashMap;


use super::Broker;

pub struct Backtest {
    cash: f64,
    positions: HashMap<String, i64>
}

struct BacktestSlice {

}

impl Backtest {
    pub fn new(initial_balance: f64) -> Backtest {
        return Backtest {
            cash: initial_balance,
            positions: HashMap::new()
        }
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

    fn order_stock(&mut self, symbol: String, quantity: i64) -> bool {
        return true;
    }

    fn order_portfolio(&mut self, allocations: &HashMap<&'static str, i64>) -> bool {
        return true;
    }
}