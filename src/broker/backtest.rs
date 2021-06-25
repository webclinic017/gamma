use std::error::Error;
use std::collections::HashMap;

use log;


use crate::datasource::DataSource;

use super::Broker;

pub struct Backtest {
    cash: f64,
    positions: HashMap<String, i64>,
}


impl Backtest {
    pub fn new(initial_balance: f64) -> Backtest {
        if initial_balance < 0.0 {
            log::error!("Initial balance should be greater than or equal to 0.0. Initial balance was {}", initial_balance);
            panic!("Initial balance");
        }

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

    fn portfolio_value<T: DataSource>(&self, datasource: &T) -> f64 {
        let mut total_portfolio_value = self.cash_available();
        for (symbol, position) in self.positions.iter() {
            match datasource.current_price((*symbol.to_owned()).to_string()) {
                Some(price) => total_portfolio_value += price * position.abs() as f64,
                None => { log::error!("Error getting price") }
            }
        }

            return total_portfolio_value;
    }

    fn positions(&self) -> HashMap<String, i64> {
        return self.positions.clone();
    }

    fn order_stock<T: DataSource>(&mut self, symbol: String, quantity: i64, datasource: &T) -> bool {
        if quantity == 0 {
            log::warn!("order_stock called with quantity 0");
            return true;
        }

        let price = match datasource.current_price(symbol.clone()) {
            Some(p) => p,
            None => { return false; }
        };

        if let Some(current_hold) = self.positions.get_mut(&symbol) {
            let current = *current_hold;
            // result share distance from 0 less current distance from 0 times price
            // debits are negative, credits are positive
            let cash_delta = (current.abs() - (current + quantity).abs()) as f64 * price;

            if self.cash + cash_delta < 0.0 {
                // should never occur when resultant magnitude is less than original
                log::error!("Insufficient cash - have: {}", self.cash);
                return false;
            }
            
            // symbol is held (long or short), so update quantity and cash
            *current_hold += quantity;
            self.cash += cash_delta
        } else {
            // symbol is not held
            let cash_delta = -quantity as f64 * price;

            if self.cash + cash_delta < 0.0 {
                // should never occur when resultant magnitude is less than original
                log::error!("Insufficient cash - have: {}", self.cash);
                return false;
            }

            self.cash += cash_delta;

            self.positions.insert(symbol.to_owned(), quantity);
        }

        return true;
    }

    fn order_portfolio<T: DataSource>(&mut self, allocations: &HashMap<&'static str, i64>, datasource: &T) -> bool {
        return true;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{broker::{Broker, backtest::Backtest}, datasource::csvring::CsvRing};

    #[test]
    fn test_new() {
        let backtest = Backtest::new(0.0);
        assert_eq!(backtest.cash_available(), 0.0)
    }

    #[test]
    #[should_panic]
    fn test_new_negative_cash() {
        let backtest = Backtest::new(-1000.0);
        assert_eq!(backtest.cash_available(), 0.0);
    }

    #[test]
    fn test_connect() {
        let mut backtest = Backtest::new(0.0);
        let opt = backtest.connect();
        assert!(opt.is_ok())
    }

    #[test]
    fn test_disconnet() {
        let mut backtest = Backtest::new(0.0);
        backtest.disconnect();
        assert!(true);
    }

    #[test]
    fn test_cash_available() {
        let backtest = Backtest::new(1000.0);
        assert_eq!(backtest.cash_available(), 1000.0);
    }

    #[test]
    fn test_portfolio_value_no_postions() {
        let datasource = Csv::default();
        let backtest = Backtest::new(1000.0);

        assert_eq!(backtest.portfolio_value(&datasource), 1000.0)
    }

    #[test]
    fn test_portfolio_value_one_postion() {
        let mut datasource = Csv::default();
        datasource.data.push(20.0);

        let mut backtest = Backtest::new(1000.0);
        backtest.positions.insert("GOOG".to_owned(), 2);

        assert_eq!(backtest.portfolio_value(&datasource), 1040.0)
    }

    #[test]
    fn test_portfolio_value_multi_postion() {
        let mut datasource = Csv::default();
        datasource.data.push(20.0);

        let mut backtest = Backtest::new(1000.0);
        backtest.positions.insert("GOOG".to_owned(), 2);
        backtest.positions.insert("AAPL".to_owned(), 3);

        assert_eq!(backtest.portfolio_value(&datasource), 1100.0)
    }

    #[test]
    fn test_portfolio_value_short_position() {
        let mut datasource = Csv::default();
        datasource.data.push(20.0);

        let mut backtest = Backtest::new(1000.0);
        backtest.positions.insert("GOOG".to_owned(), -2);

        assert_eq!(backtest.portfolio_value(&datasource), 1040.0)
    }

    #[test]
    fn test_positions() {
        let mut backtest = Backtest::new(1000.0);
        backtest.positions.insert("GOOG".to_owned(), 2);
        backtest.positions.insert("AAPL".to_owned(), 3);

        let mut same_positions = HashMap::new();
        same_positions.insert("GOOG".to_owned(), 2);
        same_positions.insert("AAPL".to_owned(), 3);

        assert_eq!(backtest.positions(), same_positions)
    }

    #[test]
    fn test_order_zero_quantity() {
        let mut datasource = Csv::default();
        datasource.data.push(20.0);

        let mut backtest = Backtest::new(1000.0);
        let effective_positions = HashMap::new();


        // no effect with zero quantity
        assert_eq!(backtest.order_stock("GOOG".to_owned(), 0, &datasource), true);

        // cash is unaffected
        assert_eq!(backtest.cash_available(), 1000.0);
        
        // positions have not moved
        assert_eq!(backtest.positions(), effective_positions);
    }

    #[test]
    fn test_order_basic() {
        let mut datasource = Csv::default();
        datasource.data.push(20.0);

        let mut backtest = Backtest::new(1000.0);
        let mut effective_positions = HashMap::new();

        effective_positions.insert("GOOG".to_owned(), 1);


        // no effect with zero quantity
        assert_eq!(backtest.order_stock("GOOG".to_owned(), 1, &datasource), true);

        // cash is unaffected
        assert_eq!(backtest.cash_available(), 980.0);
        
        // positions have not moved
        assert_eq!(backtest.positions(), effective_positions);
    }

    #[test]
    fn test_order_insufficient_cash() {
        let mut datasource = Csv::default();
        datasource.data.push(20.0);

        let mut backtest = Backtest::new(15.0);
        let effective_positions = HashMap::new();


        // no effect with zero quantity
        assert_eq!(backtest.order_stock("GOOG".to_owned(), 1, &datasource), false);

        // cash is unaffected
        assert_eq!(backtest.cash_available(), 15.0);
        
        // positions have not moved
        assert_eq!(backtest.positions(), effective_positions);
    }

    #[test]
    fn test_order_no_data() {
        let datasource = Csv::default();

        let mut backtest = Backtest::new(1000.0);
        let effective_positions = HashMap::new();


        // no effect with zero quantity
        assert_eq!(backtest.order_stock("GOOG".to_owned(), 1, &datasource), false);

        // cash is unaffected
        assert_eq!(backtest.cash_available(), 1000.0);
        
        // positions have not moved
        assert_eq!(backtest.positions(), effective_positions);
    }

    #[test]
    fn test_order_sell() {
        let mut datasource = Csv::default();
        datasource.data.push(20.0);

        let mut backtest = Backtest::new(1000.0);
        backtest.positions.insert("GOOG".to_owned(), 1);

        let mut effective_positions = HashMap::new();
        effective_positions.insert("GOOG".to_owned(), 0);

        assert_eq!(backtest.order_stock("GOOG".to_owned(), -1, &datasource), true);
        assert_eq!(backtest.cash_available(), 1020.0);
        assert_eq!(backtest.positions(), effective_positions);
    }

    #[test]
    fn test_order_sell_into_short_position() {
        let mut datasource = Csv::default();
        datasource.data.push(20.0);

        let mut backtest = Backtest::new(1000.0);
        backtest.positions.insert("GOOG".to_owned(), 1);

        let mut effective_positions = HashMap::new();
        effective_positions.insert("GOOG".to_owned(), -1);

        assert_eq!(backtest.order_stock("GOOG".to_owned(), -2, &datasource), true);

        // cash is unaffected
        assert_eq!(backtest.cash_available(), 1000.0);
        
        // positions have mirroed long/short
        assert_eq!(backtest.positions(), effective_positions);
    }

    #[test]
    fn test_order_buy_into_long_position() {
        let mut datasource = Csv::default();
        datasource.data.push(20.0);

        let mut backtest = Backtest::new(1000.0);
        backtest.positions.insert("GOOG".to_owned(), -1);

        let mut effective_positions = HashMap::new();
        effective_positions.insert("GOOG".to_owned(), 1);

        assert_eq!(backtest.order_stock("GOOG".to_owned(), 2, &datasource), true);

        // cash is unaffected
        assert_eq!(backtest.cash_available(), 1000.0);
        
        // positions have mirroed long/short
        assert_eq!(backtest.positions(), effective_positions);
    }
}