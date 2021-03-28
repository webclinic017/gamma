use std::collections::HashMap;

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
        
    }

    pub fn run(&mut self) {
        while self.step() {
            println!("Running");
        }
    }

    pub fn step(&mut self) -> bool{
        match self.datasource.drip() {
            Some(d) => {
                let mut allocations = self.strategy.process(d);
                self.handle_orders(allocations);
                return true;
            },
            None => {
                println!("Out of data");
                return false;
            }
        }
    }
}
