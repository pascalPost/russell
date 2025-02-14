use super::ComplexMatrix;
use crate::StrError;
use num_complex::Complex64;
use russell_openblas::{to_i32, zgemm};

/// Performs the matrix-matrix multiplication resulting in a matrix (complex version)
///
/// ```text
///   c  :=  α ⋅  a   ⋅   b
/// (m,n)       (m,k)   (k,n)
/// ```
///
/// # Example
///
/// ```
/// use russell_lab::{complex_mat_mat_mul, ComplexMatrix, StrError};
/// use num_complex::Complex64;
///
/// fn main() -> Result<(), StrError> {
///     let a = ComplexMatrix::from(&[
///         [1.0, 2.0],
///         [3.0, 4.0],
///         [5.0, 6.0],
///     ]);
///     let b = ComplexMatrix::from(&[
///         [-1.0, -2.0, -3.0],
///         [-4.0, -5.0, -6.0],
///     ]);
///     let alpha = Complex64::new(1.0, 0.0);
///     let mut c = ComplexMatrix::new(3, 3);
///     complex_mat_mat_mul(&mut c, alpha, &a, &b);
///     let correct = "┌                      ┐\n\
///                    │  -9+0i -12+0i -15+0i │\n\
///                    │ -19+0i -26+0i -33+0i │\n\
///                    │ -29+0i -40+0i -51+0i │\n\
///                    └                      ┘";
///     assert_eq!(format!("{}", c), correct);
///     Ok(())
/// }
/// ```
pub fn complex_mat_mat_mul(
    c: &mut ComplexMatrix,
    alpha: Complex64,
    a: &ComplexMatrix,
    b: &ComplexMatrix,
) -> Result<(), StrError> {
    let (m, n) = c.dims();
    let k = a.ncol();
    if a.nrow() != m || b.nrow() != k || b.ncol() != n {
        return Err("matrices are incompatible");
    }
    let m_i32: i32 = to_i32(m);
    let n_i32: i32 = to_i32(n);
    let k_i32: i32 = to_i32(k);
    let zero = Complex64::new(0.0, 0.0);
    zgemm(
        false,
        false,
        m_i32,
        n_i32,
        k_i32,
        alpha,
        a.as_data(),
        b.as_data(),
        zero,
        c.as_mut_data(),
    );
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{complex_mat_mat_mul, ComplexMatrix};
    use crate::complex_mat_approx_eq;
    use num_complex::Complex64;

    #[test]
    fn mat_mat_mul_fails_on_wrong_dims() {
        let a_2x1 = ComplexMatrix::new(2, 1);
        let a_1x2 = ComplexMatrix::new(1, 2);
        let b_2x1 = ComplexMatrix::new(2, 1);
        let b_1x3 = ComplexMatrix::new(1, 3);
        let mut c_2x2 = ComplexMatrix::new(2, 2);
        let alpha = Complex64::new(1.0, 0.0);
        assert_eq!(
            complex_mat_mat_mul(&mut c_2x2, alpha, &a_2x1, &b_2x1),
            Err("matrices are incompatible")
        );
        assert_eq!(
            complex_mat_mat_mul(&mut c_2x2, alpha, &a_1x2, &b_2x1),
            Err("matrices are incompatible")
        );
        assert_eq!(
            complex_mat_mat_mul(&mut c_2x2, alpha, &a_2x1, &b_1x3),
            Err("matrices are incompatible")
        );
    }

    #[test]
    fn mat_mat_mul_works() {
        let a = ComplexMatrix::from(&[
            // 2 x 3
            [1.0, 2.00, 3.0],
            [0.5, 0.75, 1.5],
        ]);
        let b = ComplexMatrix::from(&[
            // 3 x 4
            [0.1, 0.5, 0.5, 0.75],
            [0.2, 2.0, 2.0, 2.00],
            [0.3, 0.5, 0.5, 0.50],
        ]);
        let mut c = ComplexMatrix::new(2, 4);
        // c := 2⋅a⋅b
        let alpha = Complex64::new(2.0, 0.0);
        complex_mat_mat_mul(&mut c, alpha, &a, &b).unwrap();
        #[rustfmt::skip]
        let correct = &[
            [Complex64::new(2.80,0.0), Complex64::new(12.0,0.0), Complex64::new(12.0,0.0), Complex64::new(12.50,0.0)],
            [Complex64::new(1.30,0.0), Complex64::new( 5.0,0.0), Complex64::new( 5.0,0.0), Complex64::new( 5.25,0.0)],
        ];
        complex_mat_approx_eq(&c, correct, 1e-15);
    }
}
