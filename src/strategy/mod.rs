pub mod teststrat;

use std::collections::HashMap;

use crate::datasource::DataSource;

pub trait Strategy {
    fn process<T: DataSource>(&mut self, data: &mut T) -> HashMap<&'static str, f64>;
}