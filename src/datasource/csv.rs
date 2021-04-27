use std::{error::Error, ffi::OsString, fs::File};

use csv;

use super::DataSource;
#[derive(Default)]
pub struct Csv {
    pub filepath: &'static str,
    pub field: &'static str,
    pub data: Vec<f64>,
    pub idx: usize
}

impl DataSource for Csv {
    fn connect(&mut self) -> Result<bool, Box<dyn Error>> {
        let file = File::open(OsString::from(self.filepath))?;

        let mut reader = csv::Reader::from_reader(file);
        for record in reader.records() {
            let row = record?;
            self.data.push(row[5].parse::<f64>()?);
        }

        return Ok(true);
    }

    fn current_price(&mut self, symbol: String) -> Option<f64> {
        if self.idx < self.data.len() {
            return Some(self.data[self.idx]);
        }

        return None;
    }

    fn end(&self) -> bool {
        return self.idx >= self.data.len();
    }

    fn step(&mut self) {
        self.idx += 1;
    }
}