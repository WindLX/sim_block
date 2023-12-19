use crate::block::Source;

#[derive(Debug, Clone)]
pub struct Step {
    init: f64,
    end: f64,
    step_time: f64,
}

impl Step {
    pub fn new(init: f64, end: f64, step_time: f64) -> Self {
        Self {
            init,
            end,
            step_time,
        }
    }

    pub fn init(&self) -> f64 {
        self.init
    }

    pub fn end(&self) -> f64 {
        self.end
    }

    pub fn step_time(&self) -> f64 {
        self.step_time
    }
}

impl Source<f64> for Step {
    fn output(&self, t: f64) -> f64 {
        if t < self.step_time {
            self.init
        } else {
            self.end
        }
    }
}
