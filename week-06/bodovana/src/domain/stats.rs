#[derive(Debug, Copy, Clone)]
pub struct Stat {
    pub height: f64,
    pub weight: f64,
    pub bmi: f64,
    pub pulse: f64,
    pub pressure: f64,
}

impl Stat {
    /// Create a new Stat form pulse and pressure.
    ///
    /// # Examples
    /// ```
    /// use crate::bodovana::domain::stats::Stat;
    /// let stat = Stat::new(180.0, 100.0, 100.0, 100.0);
    /// assert_eq!(stat.height, 180.0);
    /// assert_eq!(stat.weight, 100.0);
    /// assert_eq!(stat.pulse, 100.0);
    /// assert_eq!(stat.pressure, 100.0);
    /// ```
    pub fn new(height: f64, weight: f64, pulse: f64, pressure: f64) -> Stat {
        let bmi = weight / ((height/100.0).powf(2.0));
        Self { height: height, weight: weight, bmi: bmi, pulse: pulse, pressure: pressure }
    }
}

impl std::ops::Sub for Stat {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(
            self.height - other.height,
            self.weight - other.weight,
            self.pulse - other.pulse,
            self.pressure - other.pressure,
        )
    }
}
