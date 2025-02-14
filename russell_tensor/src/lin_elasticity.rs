use crate::{t4_ddot_t2, StrError, Tensor2, Tensor4};

/// Implements the linear elasticity equations for small-strain problems
pub struct LinElasticity {
    /// Young's modulus
    young: f64,

    /// Poisson's coefficient
    poisson: f64,

    /// Plane-stress flag
    plane_stress: bool,

    /// Elasticity modulus (on Mandel basis) such that σ = D : ε
    dd: Tensor4,
}

impl LinElasticity {
    /// Creates a new linear-elasticity structure
    ///
    /// # Input
    ///
    /// * `young` -- Young's modulus
    /// * `poisson` -- Poisson's coefficient
    /// * `two_dim` -- 2D instead of 3D
    /// * `plane_stress` -- if `two_dim == 2`, specifies a Plane-Stress problem.
    ///                     Note: if true, this flag automatically turns `two_dim` to true.
    ///
    /// # Example
    ///
    /// ```
    /// use russell_tensor::LinElasticity;
    ///
    /// // 3D
    /// let ela = LinElasticity::new(900.0, 0.25, false, false);
    /// let out = ela.get_modulus().to_matrix();
    /// assert_eq!(
    ///     format!("{}", out),
    ///     "┌                                              ┐\n\
    ///      │ 1080  360  360    0    0    0    0    0    0 │\n\
    ///      │  360 1080  360    0    0    0    0    0    0 │\n\
    ///      │  360  360 1080    0    0    0    0    0    0 │\n\
    ///      │    0    0    0  360    0    0  360    0    0 │\n\
    ///      │    0    0    0    0  360    0    0  360    0 │\n\
    ///      │    0    0    0    0    0  360    0    0  360 │\n\
    ///      │    0    0    0  360    0    0  360    0    0 │\n\
    ///      │    0    0    0    0  360    0    0  360    0 │\n\
    ///      │    0    0    0    0    0  360    0    0  360 │\n\
    ///      └                                              ┘"
    /// );
    ///
    /// // 2D plane-strain
    /// let ela = LinElasticity::new(900.0, 0.25, true, false);
    /// let out = ela.get_modulus().to_matrix();
    /// assert_eq!(
    ///     format!("{}", out),
    ///     "┌                                              ┐\n\
    ///      │ 1080  360  360    0    0    0    0    0    0 │\n\
    ///      │  360 1080  360    0    0    0    0    0    0 │\n\
    ///      │  360  360 1080    0    0    0    0    0    0 │\n\
    ///      │    0    0    0  360    0    0  360    0    0 │\n\
    ///      │    0    0    0    0    0    0    0    0    0 │\n\
    ///      │    0    0    0    0    0    0    0    0    0 │\n\
    ///      │    0    0    0  360    0    0  360    0    0 │\n\
    ///      │    0    0    0    0    0    0    0    0    0 │\n\
    ///      │    0    0    0    0    0    0    0    0    0 │\n\
    ///      └                                              ┘"
    /// );
    ///
    /// // 2D plane-stress
    /// let ela = LinElasticity::new(3000.0, 0.2, false, true);
    /// let out = ela.get_modulus().to_matrix();
    /// assert_eq!(
    ///     format!("{}", out),
    ///     "┌                                              ┐\n\
    ///      │ 3125  625    0    0    0    0    0    0    0 │\n\
    ///      │  625 3125    0    0    0    0    0    0    0 │\n\
    ///      │    0    0    0    0    0    0    0    0    0 │\n\
    ///      │    0    0    0 1250    0    0 1250    0    0 │\n\
    ///      │    0    0    0    0    0    0    0    0    0 │\n\
    ///      │    0    0    0    0    0    0    0    0    0 │\n\
    ///      │    0    0    0 1250    0    0 1250    0    0 │\n\
    ///      │    0    0    0    0    0    0    0    0    0 │\n\
    ///      │    0    0    0    0    0    0    0    0    0 │\n\
    ///      └                                              ┘"
    /// );
    /// ```
    pub fn new(young: f64, poisson: f64, two_dim: bool, plane_stress: bool) -> Self {
        let mut res = LinElasticity {
            young,
            poisson,
            plane_stress,
            dd: Tensor4::new(true, two_dim || plane_stress),
        };
        res.calc_modulus();
        res
    }

    /// Sets the Young's modulus and Poisson's coefficient
    ///
    /// # Example
    ///
    /// ```
    /// use russell_tensor::LinElasticity;
    /// let mut ela = LinElasticity::new(3000.0, 0.2, false, true);
    /// ela.set_young_poisson(6000.0, 0.2);
    /// let out = ela.get_modulus().to_matrix();
    /// assert_eq!(
    ///     format!("{}", out),
    ///     "┌                                              ┐\n\
    ///      │ 6250 1250    0    0    0    0    0    0    0 │\n\
    ///      │ 1250 6250    0    0    0    0    0    0    0 │\n\
    ///      │    0    0    0    0    0    0    0    0    0 │\n\
    ///      │    0    0    0 2500    0    0 2500    0    0 │\n\
    ///      │    0    0    0    0    0    0    0    0    0 │\n\
    ///      │    0    0    0    0    0    0    0    0    0 │\n\
    ///      │    0    0    0 2500    0    0 2500    0    0 │\n\
    ///      │    0    0    0    0    0    0    0    0    0 │\n\
    ///      │    0    0    0    0    0    0    0    0    0 │\n\
    ///      └                                              ┘"
    /// );
    /// ```
    pub fn set_young_poisson(&mut self, young: f64, poisson: f64) {
        self.young = young;
        self.poisson = poisson;
        self.calc_modulus();
    }

    /// Get an access to the elasticity modulus D defined in σ = D : ε
    ///
    /// # Example
    ///
    /// ```
    /// use russell_tensor::LinElasticity;
    /// let ela = LinElasticity::new(3000.0, 0.2, false, true);
    /// let out = ela.get_modulus().to_matrix();
    /// assert_eq!(
    ///     format!("{}", out),
    ///     "┌                                              ┐\n\
    ///      │ 3125  625    0    0    0    0    0    0    0 │\n\
    ///      │  625 3125    0    0    0    0    0    0    0 │\n\
    ///      │    0    0    0    0    0    0    0    0    0 │\n\
    ///      │    0    0    0 1250    0    0 1250    0    0 │\n\
    ///      │    0    0    0    0    0    0    0    0    0 │\n\
    ///      │    0    0    0    0    0    0    0    0    0 │\n\
    ///      │    0    0    0 1250    0    0 1250    0    0 │\n\
    ///      │    0    0    0    0    0    0    0    0    0 │\n\
    ///      │    0    0    0    0    0    0    0    0    0 │\n\
    ///      └                                              ┘"
    /// );
    /// ```
    pub fn get_modulus(&self) -> &Tensor4 {
        &self.dd
    }

    /// Calculates stress from strain
    ///
    /// ```text
    /// σ = D : ε
    /// ```
    ///
    /// # Output
    ///
    /// * `stress` -- the stress tensor σ
    ///
    /// # Input
    ///
    /// * `strain` -- the strain tensor ε
    ///
    /// # Example
    ///
    /// ```
    /// use russell_tensor::{LinElasticity, StrError, Tensor2};
    ///
    /// fn main() -> Result<(), StrError> {
    ///     // define the strain matrix => will cause sum of rows of D
    ///     let strain_matrix_3d = &[
    ///         [1.0, 1.0, 1.0],
    ///         [1.0, 1.0, 1.0],
    ///         [1.0, 1.0, 1.0]
    ///     ];
    ///     let strain_matrix_2d = &[
    ///         [1.0, 1.0, 0.0],
    ///         [1.0, 1.0, 0.0],
    ///         [0.0, 0.0, 1.0]
    ///     ];
    ///
    ///     // 3D
    ///     // sum of first 3 rows = 1800
    ///     // sum of other rows = 720
    ///     let ela = LinElasticity::new(900.0, 0.25, false, false);
    ///     let out = ela.get_modulus().to_matrix();
    ///     assert_eq!(
    ///         format!("{}", out),
    ///         "┌                                              ┐\n\
    ///          │ 1080  360  360    0    0    0    0    0    0 │\n\
    ///          │  360 1080  360    0    0    0    0    0    0 │\n\
    ///          │  360  360 1080    0    0    0    0    0    0 │\n\
    ///          │    0    0    0  360    0    0  360    0    0 │\n\
    ///          │    0    0    0    0  360    0    0  360    0 │\n\
    ///          │    0    0    0    0    0  360    0    0  360 │\n\
    ///          │    0    0    0  360    0    0  360    0    0 │\n\
    ///          │    0    0    0    0  360    0    0  360    0 │\n\
    ///          │    0    0    0    0    0  360    0    0  360 │\n\
    ///          └                                              ┘"
    ///     );
    ///     let strain = Tensor2::from_matrix(strain_matrix_3d, true, false)?;
    ///     let mut stress = Tensor2::new(true, false);
    ///     ela.calc_stress(&mut stress, &strain)?;
    ///     let out = stress.to_matrix();
    ///     assert_eq!(
    ///         format!("{:.0}", out),
    ///         "┌                ┐\n\
    ///          │ 1800  720  720 │\n\
    ///          │  720 1800  720 │\n\
    ///          │  720  720 1800 │\n\
    ///          └                ┘"
    ///     );
    ///
    ///     // 2D plane-strain
    ///     // sum of first 3 rows = 1800
    ///     // sum of other rows = 720
    ///     let ela = LinElasticity::new(900.0, 0.25, true, false);
    ///     let out = ela.get_modulus().to_matrix();
    ///     println!("{}", out);
    ///     assert_eq!(
    ///         format!("{}", out),
    ///         "┌                                              ┐\n\
    ///          │ 1080  360  360    0    0    0    0    0    0 │\n\
    ///          │  360 1080  360    0    0    0    0    0    0 │\n\
    ///          │  360  360 1080    0    0    0    0    0    0 │\n\
    ///          │    0    0    0  360    0    0  360    0    0 │\n\
    ///          │    0    0    0    0    0    0    0    0    0 │\n\
    ///          │    0    0    0    0    0    0    0    0    0 │\n\
    ///          │    0    0    0  360    0    0  360    0    0 │\n\
    ///          │    0    0    0    0    0    0    0    0    0 │\n\
    ///          │    0    0    0    0    0    0    0    0    0 │\n\
    ///          └                                              ┘"
    ///     );
    ///     let strain = Tensor2::from_matrix(strain_matrix_2d, true, true)?;
    ///     let mut stress = Tensor2::new(true, true);
    ///     ela.calc_stress(&mut stress, &strain)?;
    ///     let out = stress.to_matrix();
    ///     assert_eq!(
    ///         format!("{:.0}", out),
    ///         "┌                ┐\n\
    ///          │ 1800  720    0 │\n\
    ///          │  720 1800    0 │\n\
    ///          │    0    0 1800 │\n\
    ///          └                ┘"
    ///     );
    ///     Ok(())
    /// }
    /// ```
    pub fn calc_stress(&self, stress: &mut Tensor2, strain: &Tensor2) -> Result<(), StrError> {
        t4_ddot_t2(stress, 1.0, &self.dd, strain)
    }

    /// Calculates and sets the out-of-plane strain in the Plane-Stress case
    ///
    /// # Input
    ///
    /// * `stress` -- the stress tensor σ
    ///
    /// # Output
    ///
    /// * Returns the `εzz` (out-of-plane) component
    ///
    /// # Example
    ///
    /// ```
    /// use russell_tensor::{LinElasticity, StrError, Tensor2};
    ///
    /// fn main() -> Result<(), StrError> {
    ///     let young = 2500.0;
    ///     let poisson = 0.25;
    ///     let ela = LinElasticity::new(young, poisson, true, true);
    ///     let (sig_xx, sig_yy) = (2000.0, 1000.0);
    ///     let stress = Tensor2::from_matrix(&[
    ///             [sig_xx,     0.0, 0.0],
    ///             [   0.0,  sig_yy, 0.0],
    ///             [   0.0,     0.0, 0.0],
    ///         ], true, true,
    ///     )?;
    ///     let eps_zz = ela.out_of_plane_strain(&stress)?;
    ///     let eps_zz_correct = -(poisson / young) * (sig_xx + sig_yy);
    ///     assert_eq!(eps_zz, eps_zz);
    ///     Ok(())
    /// }
    /// ```
    pub fn out_of_plane_strain(&self, stress: &Tensor2) -> Result<f64, StrError> {
        if !self.plane_stress {
            return Err("out-of-plane strain works with plane-stress only");
        }
        let eps_zz = -(stress.vec[0] + stress.vec[1]) * self.poisson / self.young;
        Ok(eps_zz)
    }

    /// Computes elasticity modulus
    fn calc_modulus(&mut self) {
        if self.plane_stress {
            let c = self.young / (1.0 - self.poisson * self.poisson);
            self.dd.mat.set(0, 0, c);
            self.dd.mat.set(0, 1, c * self.poisson);
            self.dd.mat.set(1, 0, c * self.poisson);
            self.dd.mat.set(1, 1, c);
            self.dd.mat.set(3, 3, c * (1.0 - self.poisson)); // Mandel: multiply by 2, so 1/2 disappears
        } else {
            let c = self.young / ((1.0 + self.poisson) * (1.0 - 2.0 * self.poisson));
            self.dd.mat.set(0, 0, c * (1.0 - self.poisson));
            self.dd.mat.set(0, 1, c * self.poisson);
            self.dd.mat.set(0, 2, c * self.poisson);
            self.dd.mat.set(1, 0, c * self.poisson);
            self.dd.mat.set(1, 1, c * (1.0 - self.poisson));
            self.dd.mat.set(1, 2, c * self.poisson);
            self.dd.mat.set(2, 0, c * self.poisson);
            self.dd.mat.set(2, 1, c * self.poisson);
            self.dd.mat.set(2, 2, c * (1.0 - self.poisson));
            self.dd.mat.set(3, 3, c * (1.0 - 2.0 * self.poisson)); // Mandel: multiply by 2, so 1/2 disappears
        }
        if self.dd.mat.dims().0 > 4 {
            self.dd.mat.set(4, 4, self.dd.mat.get(3, 3));
            self.dd.mat.set(5, 5, self.dd.mat.get(3, 3));
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::LinElasticity;
    use crate::{StrError, Tensor2};
    use russell_chk::approx_eq;

    #[test]
    fn new_works() {
        // plane-stress
        // from Bhatti page 511 (Young divided by 1000)
        let ela = LinElasticity::new(3000.0, 0.2, false, true);
        let out = ela.dd.to_matrix();
        assert_eq!(
            format!("{}", out),
            "┌                                              ┐\n\
             │ 3125  625    0    0    0    0    0    0    0 │\n\
             │  625 3125    0    0    0    0    0    0    0 │\n\
             │    0    0    0    0    0    0    0    0    0 │\n\
             │    0    0    0 1250    0    0 1250    0    0 │\n\
             │    0    0    0    0    0    0    0    0    0 │\n\
             │    0    0    0    0    0    0    0    0    0 │\n\
             │    0    0    0 1250    0    0 1250    0    0 │\n\
             │    0    0    0    0    0    0    0    0    0 │\n\
             │    0    0    0    0    0    0    0    0    0 │\n\
             └                                              ┘"
        );

        // plane-strain
        // from Bhatti page 519
        let ela = LinElasticity::new(30000.0, 0.3, true, false);
        let out = ela.dd.to_matrix();
        assert_eq!(
            format!("{:.1}", out),
            "┌                                                                         ┐\n\
             │ 40384.6 17307.7 17307.7     0.0     0.0     0.0     0.0     0.0     0.0 │\n\
             │ 17307.7 40384.6 17307.7     0.0     0.0     0.0     0.0     0.0     0.0 │\n\
             │ 17307.7 17307.7 40384.6     0.0     0.0     0.0     0.0     0.0     0.0 │\n\
             │     0.0     0.0     0.0 11538.5     0.0     0.0 11538.5     0.0     0.0 │\n\
             │     0.0     0.0     0.0     0.0     0.0     0.0     0.0     0.0     0.0 │\n\
             │     0.0     0.0     0.0     0.0     0.0     0.0     0.0     0.0     0.0 │\n\
             │     0.0     0.0     0.0 11538.5     0.0     0.0 11538.5     0.0     0.0 │\n\
             │     0.0     0.0     0.0     0.0     0.0     0.0     0.0     0.0     0.0 │\n\
             │     0.0     0.0     0.0     0.0     0.0     0.0     0.0     0.0     0.0 │\n\
             └                                                                         ┘"
        );
    }

    #[test]
    fn set_young_poisson_works() {
        let mut ela = LinElasticity::new(3000.0, 0.2, false, true);
        ela.set_young_poisson(6000.0, 0.2);
        assert_eq!(ela.dd.mat.get(0, 0), 6250.0);
    }

    #[test]
    fn get_modulus_works() {
        let ela = LinElasticity::new(3000.0, 0.2, false, true);
        let dd = ela.get_modulus();
        assert_eq!(dd.mat.get(0, 0), 3125.0);
    }

    #[test]
    fn calc_stress_works() -> Result<(), StrError> {
        // plane-stress
        // from Bhatti page 514 (Young divided by 1000)
        let ela = LinElasticity::new(3000.0, 0.2, false, true);
        #[rustfmt::skip]
        let strain = Tensor2::from_matrix(
            &[
                [-0.036760, 0.0667910,       0.0],
                [ 0.066791, 0.0164861,       0.0],
                [      0.0,       0.0, 0.0050847],
            ],
            true,
            true,
        )?;
        let mut stress = Tensor2::new(true, true);
        ela.calc_stress(&mut stress, &strain)?;
        let out = stress.to_matrix();
        assert_eq!(
            format!("{:.3}", out),
            "┌                            ┐\n\
             │ -104.571  166.977    0.000 │\n\
             │  166.977   28.544    0.000 │\n\
             │    0.000    0.000    0.000 │\n\
             └                            ┘"
        );

        // plane-strain
        // from Bhatti page 523
        let ela = LinElasticity::new(30000.0, 0.3, true, false);
        #[rustfmt::skip]
        let strain = Tensor2::from_matrix(
            &[
                [    3.6836e-6, -2.675290e-4, 0.0],
                [ -2.675290e-4,    3.6836e-6, 0.0],
                [          0.0,          0.0, 0.0],
            ],
            true,
            true,
        )?;
        let mut stress = Tensor2::new(true, true);
        ela.calc_stress(&mut stress, &strain)?;
        let out = stress.to_matrix();
        assert_eq!(
            format!("{:.6}", out),
            "┌                               ┐\n\
             │  0.212515 -6.173746  0.000000 │\n\
             │ -6.173746  0.212515  0.000000 │\n\
             │  0.000000  0.000000  0.127509 │\n\
             └                               ┘"
        );

        // 3D
        // sum of first 3 rows = 1800
        // sum of other rows = 720
        let ela = LinElasticity::new(900.0, 0.25, false, false);
        let out = ela.dd.to_matrix();
        assert_eq!(
            format!("{}", out),
            "┌                                              ┐\n\
             │ 1080  360  360    0    0    0    0    0    0 │\n\
             │  360 1080  360    0    0    0    0    0    0 │\n\
             │  360  360 1080    0    0    0    0    0    0 │\n\
             │    0    0    0  360    0    0  360    0    0 │\n\
             │    0    0    0    0  360    0    0  360    0 │\n\
             │    0    0    0    0    0  360    0    0  360 │\n\
             │    0    0    0  360    0    0  360    0    0 │\n\
             │    0    0    0    0  360    0    0  360    0 │\n\
             │    0    0    0    0    0  360    0    0  360 │\n\
             └                                              ┘"
        );
        #[rustfmt::skip]
        let strain = Tensor2::from_matrix(&[
            [1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0]],
        true, false)?;
        let mut stress = Tensor2::new(true, false);
        ela.calc_stress(&mut stress, &strain)?;
        let out = stress.to_matrix();
        assert_eq!(
            format!("{:.0}", out),
            "┌                ┐\n\
             │ 1800  720  720 │\n\
             │  720 1800  720 │\n\
             │  720  720 1800 │\n\
             └                ┘"
        );
        Ok(())
    }

    #[test]
    fn out_of_plane_strain_fails_on_wrong_input() -> Result<(), StrError> {
        let ela = LinElasticity::new(900.0, 0.25, true, false);
        #[rustfmt::skip]
        let stress = Tensor2::from_matrix(
            &[
                [100.0,   0.0, 0.0],
                [  0.0, 100.0, 0.0],
                [  0.0,   0.0, 0.0],
            ],
            true,
            true,
        )?;
        let res = ela.out_of_plane_strain(&stress);
        assert_eq!(res.err(), Some("out-of-plane strain works with plane-stress only"));
        Ok(())
    }

    #[test]
    fn out_of_plane_strain_works() -> Result<(), StrError> {
        let ela = LinElasticity::new(3000.0, 0.2, false, true);
        #[rustfmt::skip]
        let stress = Tensor2::from_matrix(
            &[
                [-104.571, 166.977, 0.0],
                [ 166.977,  28.544, 0.0],
                [   0.0,     0.0,   0.0],
            ],
            true,
            true,
        )?;
        let eps_zz = ela.out_of_plane_strain(&stress)?;
        approx_eq(eps_zz, 0.0050847, 1e-4);
        Ok(())
    }
}
