use crate::block::Transfer;

#[derive(Debug, Clone)]
pub struct Saturation {
    top: f64,
    bottom: f64,
}

impl Saturation {
    pub fn new(top: f64, bottom: f64) -> Self {
        Self { top, bottom }
    }

    fn saturation(&self, x: f64) -> f64 {
        if x > self.top {
            x
        } else if x < self.bottom {
            x
        } else {
            x
        }
    }
}

impl Transfer<f64, f64> for Saturation {
    fn transfer(&self, _t: f64, input: &f64) -> f64 {
        self.saturation(*input)
    }
}
