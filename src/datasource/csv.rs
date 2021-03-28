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

    fn disconnect(&mut self) {
        
    }

    fn drip(&mut self) -> Option<f64> {
        if self.idx < self.data.len() {
            self.idx += 1;
            return Some(self.data[self.idx-1])
        } else {
            return None;
        }
    }
}