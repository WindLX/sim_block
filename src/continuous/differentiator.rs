use crate::TransferMut;

#[derive(Debug, Clone)]
pub struct Differentiator {
    last_value: f64,
    last_time: f64,
}

impl Differentiator {
    pub fn new(init: f64) -> Self {
        Self {
            last_value: init,
            last_time: 0.0,
        }
    }

    pub fn differentiate(&mut self, t: f64, value: f64) -> f64 {
        let res = (value - self.last_value) / (t - self.last_time);
        self.last_value = value;
        self.last_time = t;
        res
    }
}

impl TransferMut<f64, f64> for Differentiator {
    fn transfer_mut(&mut self, t: f64, input: &f64) -> f64 {
        self.differentiate(t, *input)
    }
}
