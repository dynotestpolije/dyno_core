use crate::{ternary, HorsePower, NewtonMeter, Numeric, RotationPerMinute};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct DataFilter {
    pub torque: ExponentialFilter<NewtonMeter>,
    pub horsepower: ExponentialFilter<HorsePower>,
    pub rpm_roda: ExponentialFilter<RotationPerMinute>,
    pub rpm_engine: ExponentialFilter<RotationPerMinute>,
}
impl Default for DataFilter {
    fn default() -> Self {
        Self {
            torque: ExponentialFilter::new(2),
            horsepower: ExponentialFilter::new(2),
            rpm_roda: ExponentialFilter::new(100),
            rpm_engine: ExponentialFilter::new(100),
        }
    }
}

impl DataFilter {
    #[allow(dead_code)]
    #[inline]
    pub fn reset(&mut self) {
        self.torque.reset();
        self.horsepower.reset();
        self.rpm_roda.reset();
        self.rpm_engine.reset();
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExponentialFilter<T: Numeric> {
    period: usize,
    k: T,
    current: T,
    is_new: bool,
}

impl<T: Numeric> Default for ExponentialFilter<T> {
    fn default() -> Self {
        Self::new(9)
    }
}

impl<T: Numeric> ExponentialFilter<T> {
    pub fn new(period: usize) -> Self {
        let period = if cfg!(debug_assertions) {
            if period == 0 {
                panic!("period should not be zero")
            }
            period
        } else {
            ternary!((period == 0)?(1): (period))
        };

        Self {
            period,
            k: T::from_float(2.0) / T::from_u64(period as u64 + 1),
            current: T::from_float(0.0),
            is_new: true,
        }
    }
    #[allow(dead_code)]
    #[inline]
    pub const fn period(&self) -> usize {
        self.period
    }

    #[allow(dead_code)]
    #[inline]
    pub fn reset(&mut self) {
        self.current = T::from_float(0.0);
        self.is_new = true;
    }

    #[allow(dead_code)]
    #[inline]
    pub fn next(&mut self, input: T) -> T {
        if self.is_new {
            self.is_new = false;
            self.current = input;
        } else {
            self.current = self.k * input + (T::from_float(1.0) - self.k) * self.current;
        }
        self.current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let result_ok = std::panic::catch_unwind(move || ExponentialFilter::<crate::Float>::new(1));
        assert!(matches!(result_ok, Ok(_)));
    }

    #[test]
    fn test_next() {
        let mut ema = ExponentialFilter::<crate::Float>::new(3);

        assert_eq!(ema.next(2.0), 2.0);
        assert_eq!(ema.next(5.0), 3.5);
        assert_eq!(ema.next(1.0), 2.25);
        assert_eq!(ema.next(6.25), 4.25);
    }

    #[test]
    fn test_reset() {
        let mut ema = ExponentialFilter::new(5);

        assert_eq!(ema.next(4.0), 4.0);
        ema.next(10.0);
        ema.next(15.0);
        ema.next(20.0);
        assert_ne!(ema.next(4.0), 4.0);

        ema.reset();
        assert_eq!(ema.next(4.0), 4.0);
    }
}
