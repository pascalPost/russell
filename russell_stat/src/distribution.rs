/// Defines the Probability Distribution trait
pub trait Distribution {
    /// Implements the Probability Density Function (CDF)
    fn pdf(&self, x: f64) -> f64;

    /// Implements the Cumulative Density Function (CDF)
    fn cdf(&self, x: f64) -> f64;

    /// Returns the Mean
    fn mean(&self) -> f64;

    /// Returns the Variance
    fn variance(&self) -> f64;

    /// Generates a pseudo-random number belonging to this probability distribution
    fn sample(&self) -> f64;
}
