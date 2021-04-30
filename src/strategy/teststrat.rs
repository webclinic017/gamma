use std::collections::HashMap;

use super::Strategy;
use crate::datasource::DataSource;
pub struct TestStrat {
    //datasource: RefCell<DataSource>
}

impl Strategy for TestStrat {
    fn process<T: DataSource>(&mut self, data: &mut T) -> HashMap<&'static str, f64> {
        let mut allocations = HashMap::new();


        match data.current_price("GOOG".to_owned()) {
            Some(p) => {
                if p > 280.0 {
                    allocations.insert("GOOG", 1.0);
                    println!("allocation: 100%");
                } else {
                    allocations.insert("GOOG", 0.0);
                    println!("allocation: 0%");

                }
            },
            None => return allocations
        }

        
        return allocations
    }
}