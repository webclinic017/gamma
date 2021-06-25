use std::{error::Error, ffi::OsString, fs::File, cmp};

use polars_core::prelude::*;
use polars_io::prelude::*;

use super::DataSource;
#[derive(Default)]
pub struct CsvRing {
    pub data_dir_path: &'static str,
    pub field: &'static str,
    pub data: Vec<f64>,
    pub idx: usize,
    pub end: bool
}

impl CsvRing {
    fn add_series() {
        let file = File::open(OsString::from(self.filepath))?;

        let mut reader = csv::Reader::from_reader(file);
        for record in reader.records() {
            let row = record?;
            self.data.push(row[5].parse::<f64>()?);
        }
    }
}

impl DataSource for CsvRing {
    fn connect(&mut self) -> Result<bool, Box<dyn Error>> {
        return Ok(true);
    }

    fn current_price(&self, symbol: String) -> Option<f64> {
        if self.idx < self.data.len() {
            return Some(self.data[self.idx]);
        }

        return None;
    }

    fn history(&self, symbol: String, lookback: usize) -> Option<&[f64]> {
        if self.idx < self.data.len() && self.idx - lookback + 1 > 0 {
            return Some(&self.data[self.idx-lookback+1..self.idx])
        }

        return None;
    }

    fn end(&self) -> bool {
        return self.end;
    }

    fn step(&mut self) {
        self.idx = cmp::min(
            self.idx + 1,
            self.data.len()-1
        );

        if self.idx == self.data.len()-1 {
            self.end = true;
        }
    }
}