use std::{borrow::Borrow, collections::HashMap};

use crate::strategy::Strategy;
use crate::datasource::DataSource;
use crate::broker::Broker;

pub struct Engine<'a, T1, T2, T3> {
    pub strategy: &'a mut T1,
    pub datasource: &'a mut T2,
    pub broker: &'a mut T3
}

impl<'a, T1: Strategy, T2: DataSource, T3: Broker> Engine<'a, T1, T2, T3> {
    fn handle_orders(&mut self, allocations: HashMap<&'static str, f64>) {
        let cash = self.broker.cash_available();
        let positions = self.broker.positions();
        
        let mut portfolio_value: f64 = cash;

        // determine portfolio value
        for (symbol, quantity) in positions.iter() {
            match self.datasource.current_price(symbol.clone()) {
                Some(price) => {
                    portfolio_value += price * (*quantity) as f64;
                },
                None => {
                    eprintln!("Unable to get price for {}", symbol)
                }
            };
        }

        // determine target quantities
        for (symbol, desired_percent) in allocations.iter() {
            let desired_quantity = match self.datasource.current_price((*symbol).to_string()) {
                Some(price) => {
                    ((portfolio_value * desired_percent) / price) as i64
                },
                None => {
                    eprintln!("Unable to get price for {}", symbol);
                    0 as i64
                }
            };

            // determine order delta, if exists
            match positions.get(*symbol) {
                Some(current_quantity) => {
                    let order_quantity = desired_quantity - *current_quantity;
                    if order_quantity == 0 {continue;}
                    self.broker.order_stock((*symbol).to_string(), order_quantity);
                },
                None => {
                    if desired_quantity == 0 {continue;}
                    self.broker.order_stock((*symbol).to_string(), desired_quantity);
                }
            }

        }
    }

    pub fn run(&mut self) {
        while self.step() {
            println!("Running");
        }
    }

    pub fn step(&mut self) -> bool {
        if self.datasource.end() {
            return false;
        }

        let mut allocations = self.strategy.process(self.datasource);
        self.handle_orders(allocations);
        self.datasource.step();

        return true;
    }
            
}
