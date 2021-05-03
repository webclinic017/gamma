pub trait Indicator {
    fn ready(&self) -> bool;
    fn load(&mut self, data: f64);
    fn current(&mut self) -> f64;
}

pub struct Sma {
    pub length: u8,
    pub data: Vec<f64>, 
}

impl Indicator for Sma {
    fn ready(&self) -> bool {
        return self.data.len() >= self.length as usize;
    }

    fn load(&mut self, data: f64) {
        self.data.push(data);

        while self.data.len() > self.length as usize {
            self.data.remove(0);
        }
    }

    fn current(&mut self) -> f64 {
        return self.data.iter().sum::<f64>() / self.length as f64;
    }
}