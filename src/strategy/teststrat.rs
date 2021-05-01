use std::collections::HashMap;

use super::Strategy;
use crate::{datasource::DataSource, indicator::Indicator};
use crate::indicator::sma;

pub struct TestStrat {
    //datasource: RefCell<DataSource>
    pub fast_sma: sma::Sma,
    pub slow_sma: sma::Sma
}

impl Strategy for TestStrat {
    fn process<T: DataSource>(&mut self, data: &mut T) -> HashMap<&'static str, f64> {
        let mut allocations = HashMap::new();

        

        match data.current_price("GOOG".to_owned()) {
            Some(p) => {
                self.fast_sma.load(p);
                self.slow_sma.load(p);

                if self.slow_sma.ready() && self.slow_sma.ready() {
                    if self.fast_sma.current() > self.slow_sma.current() {
                        allocations.insert("GOOG", 1.0);
                    } else {
                        allocations.insert("GOOG", 0.0);
                    }
                }
            },
            None => return allocations
        }

        
        return allocations
    }
}