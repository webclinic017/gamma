use super::Indicator;
struct Sma {
    length: u8,
    data: Vec<f64>, 
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