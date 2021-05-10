use std::collections::HashMap;

use super::Strategy;
use crate::{datasource::DataSource, indicator::Indicator};
use crate::indicator::Sma;

pub struct TestStrat {
    //datasource: RefCell<DataSource>
    pub previous: f64
}

impl Strategy for TestStrat {
    fn process<T: DataSource>(&mut self, data: &mut T) -> HashMap<&'static str, f64> {
        let mut allocations = HashMap::new();

        

        match data.current_price("GOOG".to_owned()) {
            Some(p) => {

                if p > self.previous {
                    allocations.insert("GOOG", 1.0);
                } else {
                    allocations.insert("GOOG", 0.0);
                }

                self.previous = p;
                return allocations;
            },
            None => return allocations
        }
    }
}