use crate::{gamma, Distribution, StrError};

const FRECHET_MIN_DELTA_X: f64 = 1e-15;

/// Defines the Frechet / Type II Extreme Value Distribution (largest value)
pub struct DistributionFrechet {
    location: f64,
    scale: f64,
    shape: f64,
}

impl DistributionFrechet {
    /// Creates a new Frechet distribution
    ///
    /// # Input
    ///
    /// * `location` -- location parameter
    /// * `shape` -- shape parameter
    pub fn new(location: f64, scale: f64, shape: f64) -> Result<Self, StrError> {
        Ok(DistributionFrechet { location, scale, shape })
    }
}

impl Distribution for DistributionFrechet {
    /// Implements the Probability Density Function (CDF)
    fn pdf(&self, x: f64) -> f64 {
        if x - self.location < FRECHET_MIN_DELTA_X {
            return 0.0;
        }
        let z = (x - self.location) / self.scale;
        f64::exp(-f64::powf(z, -self.shape)) * f64::powf(z, -1.0 - self.shape) * self.shape / self.scale
    }

    /// Implements the Cumulative Density Function (CDF)
    fn cdf(&self, x: f64) -> f64 {
        if x - self.location < FRECHET_MIN_DELTA_X {
            return 0.0;
        }
        let z = (x - self.location) / self.scale;
        f64::exp(-f64::powf(z, -self.shape))
    }

    /// Returns the Mean
    fn mean(&self) -> f64 {
        if self.shape > 1.0 {
            return self.location + self.scale * gamma(1.0 - 1.0 / self.shape);
        }
        f64::INFINITY
    }

    /// Returns the Variance
    fn variance(&self) -> f64 {
        if self.shape > 2.0 {
            return self.scale
                * self.scale
                * (gamma(1.0 - 2.0 / self.shape) - f64::powf(gamma(1.0 - 1.0 / self.shape), 2.0));
        }
        f64::INFINITY
    }

    /// Generates a pseudo-random number belonging to this probability distribution
    fn sample(&self) -> f64 {
        0.0
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::{Distribution, DistributionFrechet, StrError};
    use russell_chk::assert_approx_eq;

    // Data from the following R-code (run with Rscript frechet.R):
    /*
    # needs r-cran-evd
    library(evd)
    X <- seq(0, 4, 0.5)
    L <- c(0, 0.5)  # location
    C <- c(1, 2)    # scale
    A <- c(1, 2, 3) # shape
    Y <- matrix(ncol=5)
    first <- TRUE
    for (l in L) {
        for (c in C) {
            for (a in A) {
                pdf <- dfrechet(X, l, c, a)
                cdf <- pfrechet(X, l, c, a)
                for (i in 1:length(X)) {
                    if (first) {
                        Y <- rbind(c(X[i], l, c, a, pdf[i], cdf[i]))
                        first <- FALSE
                    } else {
                        Y <- rbind(Y, c(X[i], l, c, a, pdf[i], cdf[i]))
                    }
                }
            }
        }
    }
    write.table(format(Y, digits=15), "/tmp/frechet.dat", row.names=FALSE, col.names=c("x","location","scale","shape","pdf","cdf"), quote=FALSE)
    print("file </tmp/frechet.dat> written")
    */

    #[test]
    fn frechet_works() -> Result<(), StrError> {
        #[rustfmt::skip]
        // x, location, scale, shape, pdf, cdf
        let data = [
            [0.00000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 1.00000000000000e+00, 0.00000000000000e+00, 0.00000000000000e+00],
            [5.00000000000000e-01, 0.00000000000000e+00, 1.00000000000000e+00, 1.00000000000000e+00, 5.41341132946451e-01, 1.35335283236613e-01],
            [1.00000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 1.00000000000000e+00, 3.67879441171442e-01, 3.67879441171442e-01],
            [1.50000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 1.00000000000000e+00, 2.28185386236708e-01, 5.13417119032592e-01],
            [2.00000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 1.00000000000000e+00, 1.51632664928158e-01, 6.06530659712633e-01],
            [2.50000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 1.00000000000000e+00, 1.07251207365702e-01, 6.70320046035639e-01],
            [3.00000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 1.00000000000000e+00, 7.96145900637543e-02, 7.16531310573789e-01],
            [3.50000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 1.00000000000000e+00, 6.13450851490029e-02, 7.51477293075286e-01],
            [4.00000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 1.00000000000000e+00, 4.86750489419628e-02, 7.78800783071405e-01],
            [0.00000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 2.00000000000000e+00, 0.00000000000000e+00, 0.00000000000000e+00],
            [5.00000000000000e-01, 0.00000000000000e+00, 1.00000000000000e+00, 2.00000000000000e+00, 2.93050222219747e-01, 1.83156388887342e-02],
            [1.00000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 2.00000000000000e+00, 7.35758882342885e-01, 3.67879441171442e-01],
            [1.50000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 2.00000000000000e+00, 3.79958748699232e-01, 6.41180388429955e-01],
            [2.00000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 2.00000000000000e+00, 1.94700195767851e-01, 7.78800783071405e-01],
            [2.50000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 2.00000000000000e+00, 1.09074404987675e-01, 8.52143788966211e-01],
            [3.00000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 2.00000000000000e+00, 6.62843938381015e-02, 8.94839316814370e-01],
            [3.50000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 2.00000000000000e+00, 4.29905748010600e-02, 9.21610447297725e-01],
            [4.00000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 2.00000000000000e+00, 2.93566582129211e-02, 9.39413062813476e-01],
            [0.00000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 3.00000000000000e+00, 0.00000000000000e+00, 0.00000000000000e+00],
            [5.00000000000000e-01, 0.00000000000000e+00, 1.00000000000000e+00, 3.00000000000000e+00, 1.61022061393206e-02, 3.35462627902512e-04],
            [1.00000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 3.00000000000000e+00, 1.10363832351433e+00, 3.67879441171442e-01],
            [1.50000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 3.00000000000000e+00, 4.40632343233130e-01, 7.43567079205906e-01],
            [2.00000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 3.00000000000000e+00, 1.65468169234612e-01, 8.82496902584595e-01],
            [2.50000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 3.00000000000000e+00, 7.20387839639600e-02, 9.38004999530729e-01],
            [3.00000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 3.00000000000000e+00, 3.56903868259736e-02, 9.63640444301286e-01],
            [3.50000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 3.00000000000000e+00, 1.95307877314814e-02, 9.76946277985140e-01],
            [4.00000000000000e+00, 0.00000000000000e+00, 1.00000000000000e+00, 3.00000000000000e+00, 1.15370676211571e-02, 9.84496437005408e-01],
            [0.00000000000000e+00, 0.00000000000000e+00, 2.00000000000000e+00, 1.00000000000000e+00, 0.00000000000000e+00, 0.00000000000000e+00],
            [5.00000000000000e-01, 0.00000000000000e+00, 2.00000000000000e+00, 1.00000000000000e+00, 1.46525111109873e-01, 1.83156388887342e-02],
            [1.00000000000000e+00, 0.00000000000000e+00, 2.00000000000000e+00, 1.00000000000000e+00, 2.70670566473225e-01, 1.35335283236613e-01],
            [1.50000000000000e+00, 0.00000000000000e+00, 2.00000000000000e+00, 1.00000000000000e+00, 2.34308567213979e-01, 2.63597138115727e-01],
            [2.00000000000000e+00, 0.00000000000000e+00, 2.00000000000000e+00, 1.00000000000000e+00, 1.83939720585721e-01, 3.67879441171442e-01],
            [2.50000000000000e+00, 0.00000000000000e+00, 2.00000000000000e+00, 1.00000000000000e+00, 1.43785268517511e-01, 4.49328964117222e-01],
            [3.00000000000000e+00, 0.00000000000000e+00, 2.00000000000000e+00, 1.00000000000000e+00, 1.14092693118354e-01, 5.13417119032592e-01],
            [3.50000000000000e+00, 0.00000000000000e+00, 2.00000000000000e+00, 1.00000000000000e+00, 9.21988770624913e-02, 5.64718122007759e-01],
            [4.00000000000000e+00, 0.00000000000000e+00, 2.00000000000000e+00, 1.00000000000000e+00, 7.58163324640792e-02, 6.06530659712633e-01],
            [0.00000000000000e+00, 0.00000000000000e+00, 2.00000000000000e+00, 2.00000000000000e+00, 0.00000000000000e+00, 0.00000000000000e+00],
            [5.00000000000000e-01, 0.00000000000000e+00, 2.00000000000000e+00, 2.00000000000000e+00, 7.20225118203259e-06, 1.12535174719259e-07],
            [1.00000000000000e+00, 0.00000000000000e+00, 2.00000000000000e+00, 2.00000000000000e+00, 1.46525111109873e-01, 1.83156388887342e-02],
            [1.50000000000000e+00, 0.00000000000000e+00, 2.00000000000000e+00, 2.00000000000000e+00, 4.00624155036601e-01, 1.69013315406066e-01],
            [2.00000000000000e+00, 0.00000000000000e+00, 2.00000000000000e+00, 2.00000000000000e+00, 3.67879441171442e-01, 3.67879441171442e-01],
            [2.50000000000000e+00, 0.00000000000000e+00, 2.00000000000000e+00, 2.00000000000000e+00, 2.69973721110041e-01, 5.27292424043049e-01],
            [3.00000000000000e+00, 0.00000000000000e+00, 2.00000000000000e+00, 2.00000000000000e+00, 1.89979374349616e-01, 6.41180388429955e-01],
            [3.50000000000000e+00, 0.00000000000000e+00, 2.00000000000000e+00, 2.00000000000000e+00, 1.34609406946660e-01, 7.21422290354756e-01],
            [4.00000000000000e+00, 0.00000000000000e+00, 2.00000000000000e+00, 2.00000000000000e+00, 9.73500978839256e-02, 7.78800783071405e-01],
            [0.00000000000000e+00, 0.00000000000000e+00, 2.00000000000000e+00, 3.00000000000000e+00, 0.00000000000000e+00, 0.00000000000000e+00],
            [5.00000000000000e-01, 0.00000000000000e+00, 2.00000000000000e+00, 3.00000000000000e+00, 6.15863381970675e-26, 1.60381089054864e-28],
            [1.00000000000000e+00, 0.00000000000000e+00, 2.00000000000000e+00, 3.00000000000000e+00, 8.05110306966028e-03, 3.35462627902512e-04],
            [1.50000000000000e+00, 0.00000000000000e+00, 2.00000000000000e+00, 3.00000000000000e+00, 4.43003781677631e-01, 9.34461101976254e-02],
            [2.00000000000000e+00, 0.00000000000000e+00, 2.00000000000000e+00, 3.00000000000000e+00, 5.51819161757164e-01, 3.67879441171442e-01],
            [2.50000000000000e+00, 0.00000000000000e+00, 2.00000000000000e+00, 3.00000000000000e+00, 3.68207332052299e-01, 5.99295787845538e-01],
            [3.00000000000000e+00, 0.00000000000000e+00, 2.00000000000000e+00, 3.00000000000000e+00, 2.20316171616565e-01, 7.43567079205906e-01],
            [3.50000000000000e+00, 0.00000000000000e+00, 2.00000000000000e+00, 3.00000000000000e+00, 1.32710267758176e-01, 8.29784773144222e-01],
            [4.00000000000000e+00, 0.00000000000000e+00, 2.00000000000000e+00, 3.00000000000000e+00, 8.27340846173058e-02, 8.82496902584595e-01],
            [0.00000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 1.00000000000000e+00, 0.00000000000000e+00, 0.00000000000000e+00],
            [5.00000000000000e-01, 5.00000000000000e-01, 1.00000000000000e+00, 1.00000000000000e+00, 0.00000000000000e+00, 0.00000000000000e+00],
            [1.00000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 1.00000000000000e+00, 5.41341132946451e-01, 1.35335283236613e-01],
            [1.50000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 1.00000000000000e+00, 3.67879441171442e-01, 3.67879441171442e-01],
            [2.00000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 1.00000000000000e+00, 2.28185386236708e-01, 5.13417119032592e-01],
            [2.50000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 1.00000000000000e+00, 1.51632664928158e-01, 6.06530659712633e-01],
            [3.00000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 1.00000000000000e+00, 1.07251207365702e-01, 6.70320046035639e-01],
            [3.50000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 1.00000000000000e+00, 7.96145900637543e-02, 7.16531310573789e-01],
            [4.00000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 1.00000000000000e+00, 6.13450851490029e-02, 7.51477293075286e-01],
            [0.00000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 2.00000000000000e+00, 0.00000000000000e+00, 0.00000000000000e+00],
            [5.00000000000000e-01, 5.00000000000000e-01, 1.00000000000000e+00, 2.00000000000000e+00, 0.00000000000000e+00, 0.00000000000000e+00],
            [1.00000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 2.00000000000000e+00, 2.93050222219747e-01, 1.83156388887342e-02],
            [1.50000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 2.00000000000000e+00, 7.35758882342885e-01, 3.67879441171442e-01],
            [2.00000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 2.00000000000000e+00, 3.79958748699232e-01, 6.41180388429955e-01],
            [2.50000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 2.00000000000000e+00, 1.94700195767851e-01, 7.78800783071405e-01],
            [3.00000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 2.00000000000000e+00, 1.09074404987675e-01, 8.52143788966211e-01],
            [3.50000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 2.00000000000000e+00, 6.62843938381015e-02, 8.94839316814370e-01],
            [4.00000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 2.00000000000000e+00, 4.29905748010600e-02, 9.21610447297725e-01],
            [0.00000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 3.00000000000000e+00, 0.00000000000000e+00, 0.00000000000000e+00],
            [5.00000000000000e-01, 5.00000000000000e-01, 1.00000000000000e+00, 3.00000000000000e+00, 0.00000000000000e+00, 0.00000000000000e+00],
            [1.00000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 3.00000000000000e+00, 1.61022061393206e-02, 3.35462627902512e-04],
            [1.50000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 3.00000000000000e+00, 1.10363832351433e+00, 3.67879441171442e-01],
            [2.00000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 3.00000000000000e+00, 4.40632343233130e-01, 7.43567079205906e-01],
            [2.50000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 3.00000000000000e+00, 1.65468169234612e-01, 8.82496902584595e-01],
            [3.00000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 3.00000000000000e+00, 7.20387839639600e-02, 9.38004999530729e-01],
            [3.50000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 3.00000000000000e+00, 3.56903868259736e-02, 9.63640444301286e-01],
            [4.00000000000000e+00, 5.00000000000000e-01, 1.00000000000000e+00, 3.00000000000000e+00, 1.95307877314814e-02, 9.76946277985140e-01],
            [0.00000000000000e+00, 5.00000000000000e-01, 2.00000000000000e+00, 1.00000000000000e+00, 0.00000000000000e+00, 0.00000000000000e+00],
            [5.00000000000000e-01, 5.00000000000000e-01, 2.00000000000000e+00, 1.00000000000000e+00, 0.00000000000000e+00, 0.00000000000000e+00],
            [1.00000000000000e+00, 5.00000000000000e-01, 2.00000000000000e+00, 1.00000000000000e+00, 1.46525111109873e-01, 1.83156388887342e-02],
            [1.50000000000000e+00, 5.00000000000000e-01, 2.00000000000000e+00, 1.00000000000000e+00, 2.70670566473225e-01, 1.35335283236613e-01],
            [2.00000000000000e+00, 5.00000000000000e-01, 2.00000000000000e+00, 1.00000000000000e+00, 2.34308567213979e-01, 2.63597138115727e-01],
            [2.50000000000000e+00, 5.00000000000000e-01, 2.00000000000000e+00, 1.00000000000000e+00, 1.83939720585721e-01, 3.67879441171442e-01],
            [3.00000000000000e+00, 5.00000000000000e-01, 2.00000000000000e+00, 1.00000000000000e+00, 1.43785268517511e-01, 4.49328964117222e-01],
            [3.50000000000000e+00, 5.00000000000000e-01, 2.00000000000000e+00, 1.00000000000000e+00, 1.14092693118354e-01, 5.13417119032592e-01],
            [4.00000000000000e+00, 5.00000000000000e-01, 2.00000000000000e+00, 1.00000000000000e+00, 9.21988770624913e-02, 5.64718122007759e-01],
            [0.00000000000000e+00, 5.00000000000000e-01, 2.00000000000000e+00, 2.00000000000000e+00, 0.00000000000000e+00, 0.00000000000000e+00],
            [5.00000000000000e-01, 5.00000000000000e-01, 2.00000000000000e+00, 2.00000000000000e+00, 0.00000000000000e+00, 0.00000000000000e+00],
            [1.00000000000000e+00, 5.00000000000000e-01, 2.00000000000000e+00, 2.00000000000000e+00, 7.20225118203259e-06, 1.12535174719259e-07],
            [1.50000000000000e+00, 5.00000000000000e-01, 2.00000000000000e+00, 2.00000000000000e+00, 1.46525111109873e-01, 1.83156388887342e-02],
            [2.00000000000000e+00, 5.00000000000000e-01, 2.00000000000000e+00, 2.00000000000000e+00, 4.00624155036601e-01, 1.69013315406066e-01],
            [2.50000000000000e+00, 5.00000000000000e-01, 2.00000000000000e+00, 2.00000000000000e+00, 3.67879441171442e-01, 3.67879441171442e-01],
            [3.00000000000000e+00, 5.00000000000000e-01, 2.00000000000000e+00, 2.00000000000000e+00, 2.69973721110041e-01, 5.27292424043049e-01],
            [3.50000000000000e+00, 5.00000000000000e-01, 2.00000000000000e+00, 2.00000000000000e+00, 1.89979374349616e-01, 6.41180388429955e-01],
            [4.00000000000000e+00, 5.00000000000000e-01, 2.00000000000000e+00, 2.00000000000000e+00, 1.34609406946660e-01, 7.21422290354756e-01],
            [0.00000000000000e+00, 5.00000000000000e-01, 2.00000000000000e+00, 3.00000000000000e+00, 0.00000000000000e+00, 0.00000000000000e+00],
            [5.00000000000000e-01, 5.00000000000000e-01, 2.00000000000000e+00, 3.00000000000000e+00, 0.00000000000000e+00, 0.00000000000000e+00],
            [1.00000000000000e+00, 5.00000000000000e-01, 2.00000000000000e+00, 3.00000000000000e+00, 6.15863381970675e-26, 1.60381089054864e-28],
            [1.50000000000000e+00, 5.00000000000000e-01, 2.00000000000000e+00, 3.00000000000000e+00, 8.05110306966028e-03, 3.35462627902512e-04],
            [2.00000000000000e+00, 5.00000000000000e-01, 2.00000000000000e+00, 3.00000000000000e+00, 4.43003781677631e-01, 9.34461101976254e-02],
            [2.50000000000000e+00, 5.00000000000000e-01, 2.00000000000000e+00, 3.00000000000000e+00, 5.51819161757164e-01, 3.67879441171442e-01],
            [3.00000000000000e+00, 5.00000000000000e-01, 2.00000000000000e+00, 3.00000000000000e+00, 3.68207332052299e-01, 5.99295787845538e-01],
            [3.50000000000000e+00, 5.00000000000000e-01, 2.00000000000000e+00, 3.00000000000000e+00, 2.20316171616565e-01, 7.43567079205906e-01],
            [4.00000000000000e+00, 5.00000000000000e-01, 2.00000000000000e+00, 3.00000000000000e+00, 1.32710267758176e-01, 8.29784773144222e-01],
        ];
        for row in data {
            let [x, location, scale, shape, pdf, cdf] = row;
            let d = DistributionFrechet::new(location, scale, shape)?;
            assert_approx_eq!(d.pdf(x), pdf, 1e-14);
            assert_approx_eq!(d.cdf(x), cdf, 1e-14);
        }
        Ok(())
    }

    #[test]
    fn mean_and_variance_work() -> Result<(), StrError> {
        let location = 8.782275;
        let scale = 1.0;
        let shape = 4.095645;
        let d = DistributionFrechet::new(location, scale, shape)?;
        assert_approx_eq!(d.mean(), 10.0, 1e-6);
        assert_approx_eq!(d.variance(), 0.25, 1e-6);

        let d = DistributionFrechet::new(location, scale, 0.0)?;
        assert_eq!(d.mean(), f64::INFINITY);
        assert_eq!(d.variance(), f64::INFINITY);
        Ok(())
    }
}
