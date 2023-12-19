use crate::block::TransferMut;

#[derive(Debug, Clone)]
pub struct Integrator {
    init: f64,
    is_derivative: bool,
    last_time: f64,
    last_value: f64,
    past: f64,
}

impl Integrator {
    pub fn new(init: f64, is_derivative: bool) -> Self {
        Self {
            init,
            last_value: init,
            past: init,
            last_time: 0.0,
            is_derivative,
        }
    }

    fn integrate(&mut self, t: f64, value: f64) -> f64 {
        self.past += (t - self.last_time) * (value + self.last_value) * 0.5;
        self.last_value = value;
        self.last_time = t;
        self.past
    }

    fn derivative_add(&mut self, t: f64, derivative: f64) -> f64 {
        let delta_t = t - self.last_time;
        self.past += derivative.clone() * delta_t;
        self.last_value = derivative;
        self.last_time = t;
        self.past.clone()
    }

    pub fn past(&self) -> f64 {
        self.past
    }

    pub fn reset(&mut self) {
        self.last_value = self.init;
        self.past = self.init;
        self.last_time = 0.0;
    }
}

impl TransferMut<f64, f64> for Integrator {
    fn transfer_mut(&mut self, t: f64, input: &f64) -> f64 {
        if self.is_derivative {
            self.derivative_add(t, *input)
        } else {
            self.integrate(t, *input)
        }
    }
}

#[cfg(feature = "vector")]
pub(crate) mod vector {
    use crate::{block::TransferMut, model::Vector};

    #[derive(Debug, Clone)]
    pub struct VectorIntegrator {
        init: Vector,
        last_time: f64,
        last_value: Vector,
        past: Vector,
        is_derivative: bool,
    }

    impl VectorIntegrator {
        pub fn new(init: impl Into<Vector>, is_derivative: bool) -> Self {
            let init = init.into();
            Self {
                init: init.clone(),
                last_value: init.clone(),
                past: init,
                last_time: 0.0,
                is_derivative,
            }
        }

        fn integrate(&mut self, t: f64, value: &Vector) -> Vector {
            let value = value.clone();
            self.past += (value.clone() + self.last_value.clone()) * (t - self.last_time) * 0.5;
            self.last_value = value;
            self.last_time = t;
            self.past.clone()
        }

        fn derivative_add(&mut self, t: f64, derivative: &Vector) -> Vector {
            let derivative = derivative.clone();
            let delta_t = t - self.last_time;
            self.past += derivative.clone() * delta_t;
            self.last_value = derivative;
            self.last_time = t;
            self.past.clone()
        }

        pub fn past(&self) -> Vector {
            self.past.clone()
        }

        pub fn reset(&mut self) {
            self.last_value = self.init.clone();
            self.past = self.init.clone();
            self.last_time = 0.0;
        }
    }

    impl TransferMut<Vector, Vector> for VectorIntegrator {
        fn transfer_mut(&mut self, t: f64, input: &Vector) -> Vector {
            if self.is_derivative {
                self.derivative_add(t, &input)
            } else {
                self.integrate(t, &input)
            }
        }
    }
}

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

    pub fn differentiate(&mut self, value: f64, t: f64) -> f64 {
        let res = (value - self.last_value) / (t - self.last_time);
        self.last_value = value;
        self.last_time = t;
        res
    }
}

#[cfg(test)]
mod continuous_tests {
    #[cfg(feature = "vector")]
    use super::vector::VectorIntegrator;

    use super::Integrator;
    use crate::{block::TransferMut, utils::test_logger_init};
    use log::{info, trace};
    use std::time::{Duration, SystemTime};

    #[test]
    fn test_integrator() {
        test_logger_init();
        let mut i = Integrator::new(0.0, false);
        let start_time = SystemTime::now();
        let mut r;
        loop {
            let current_time = SystemTime::now();
            let delta_time = current_time.duration_since(start_time).unwrap();
            r = i.transfer_mut(delta_time.as_secs_f64(), &delta_time.as_secs_f64());
            trace!("time: {:?} \n{:?}\n", delta_time, r);
            if delta_time > Duration::from_secs_f32(1.0) {
                break;
            }
        }
        assert!((r - 0.5).abs() < 1e-5);
    }

    #[cfg(feature = "vector")]
    #[test]
    fn test_vector_integrator() {
        use crate::{block::TransferMut, model::Vector};

        test_logger_init();
        let mut i = VectorIntegrator::new(vec![0.0, 0.0], false);
        let start_time = SystemTime::now();
        let mut r;
        loop {
            let current_time = SystemTime::now();
            let delta_time = current_time.duration_since(start_time).unwrap();
            r = i.transfer_mut(
                delta_time.as_secs_f64(),
                &Vector::from(vec![
                    delta_time.as_secs_f64(),
                    2.0 * delta_time.as_secs_f64(),
                ]),
            );
            trace!("time: {:?} \n{:?}\n", delta_time, r);
            info!("{:?}", i.past());
            if delta_time > Duration::from_secs_f32(1.0) {
                break;
            }
        }

        assert!((r[0] - 0.5).abs() < 1e-3);
        assert!((r[1] - 1.0).abs() < 1e-3);
    }
}
