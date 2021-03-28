use std::collections::HashMap;

use super::Strategy;
pub struct TestStrat {
    
}

impl Strategy for TestStrat {
    fn process(&mut self, data: f64) -> HashMap<&'static str, f64> {
        let mut allocations = HashMap::new();
        allocations.insert("GOOG", 1.0);

        println!("strategy | data: {}", data);
        return allocations
    }
}