use std::collections::HashMap;

use crate::strategy::Strategy;
use crate::datasource::DataSource;
use crate::broker::Broker;

struct Order {
    symbol: String,
    quantity: i64
}

impl Order {
    fn new(symbol: String, quantity: i64) -> Order {
        return Order {
            symbol: symbol,
            quantity: quantity
        };
    }
}

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

        let mut order_queue: Vec<Order> = Vec::new();

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
                    let delta = desired_quantity - *current_quantity;

                    if delta != 0 {
                        if desired_quantity.abs() > (*current_quantity).abs() {
                            // will result in a net loss to cash, so de-prioritize
                            // put in back of order queue
                            order_queue.push(Order::new((*symbol).to_string(), delta))
                        } else {
                            // else will result in a gain in cash, so make this top priority
                            order_queue.insert(0, Order::new((*symbol).to_string(), delta));
                        }
                    }
                },
                None => {
                    let delta = desired_quantity as i64;
                    if delta != 0 {
                        // going from no position to a long or short position will cause a loss in cash, so de-prioritize
                        order_queue.push(Order::new((*symbol).to_string(), delta))
                    }
                }
            };


        }

        // execute the orders
        for order in order_queue.iter() {
            self.broker.order_stock((order.symbol).to_string(), order.quantity, self.datasource);
        } 
    }

    pub fn run(&mut self) {
        let mut equity_line:Vec<(f32, f32)> = Vec::new();
        let mut curr_step = 0 as f32;
        equity_line.push((curr_step, self.broker.portfolio_value(self.datasource) as f32));
        while self.step() {
            println!("Running");
            curr_step += 1 as f32;
            if !self.datasource.end() {
                equity_line.push((curr_step, self.broker.portfolio_value(self.datasource) as f32));
                println!("value: {}", self.broker.portfolio_value(self.datasource));
            }
        }
    }

    pub fn step(&mut self) -> bool {
        if self.datasource.end() {
            return false;
        }

        let allocations = self.strategy.process(self.datasource);
        self.handle_orders(allocations);
        self.datasource.step();

        return true;
    }

    pub fn equity(&self) -> f64 {
        return self.broker.portfolio_value(self.datasource);
    }
            
}
