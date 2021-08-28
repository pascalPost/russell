use crate::matrix::*;
use crate::vector::*;
use russell_openblas::*;

/// Solves a general linear system (real numbers)
///
/// For a general matrix `a` (square, symmetric, non-symmetric, dense,
/// sparse), find `x` such that:
///
/// ```text
///   a   ⋅  x  =  b
/// (m,m)   (m)   (m)
/// ```
///
/// However, the right-hand-side will hold the solution:
///
/// ```text
/// b := a⁻¹⋅b == x
/// ```
///
/// The solution is obtained via LU decomposition using Lapack dgesv routine.
///
/// # Note
///
/// 1. The matrix `a` will be modified
/// 2. The right-hand-side `b` will contain the solution `x`
///
/// ```
/// # fn main() -> Result<(), &'static str> {
/// // import
/// use russell_lab::*;
///
/// // set matrix and right-hand side
/// let mut a = Matrix::from(&[
///     &[1.0,  3.0, -2.0],
///     &[3.0,  5.0,  6.0],
///     &[2.0,  4.0,  3.0],
/// ])?;
/// let mut b = Vector::from(&[5.0, 7.0, 8.0]);
///
/// // solve linear system b := a⁻¹⋅b
/// solve_lin_sys(&mut b, &mut a)?;
///
/// // check
/// let x_correct = "┌         ┐\n\
///                  │ -15.000 │\n\
///                  │   8.000 │\n\
///                  │   2.000 │\n\
///                  └         ┘";
/// assert_eq!(format!("{:.3}", b), x_correct);
/// # Ok(())
/// # }
/// ```
pub fn solve_lin_sys(b: &mut Vector, a: &mut Matrix) -> Result<(), &'static str> {
    let (m, n) = (a.nrow, a.ncol);
    if m != n {
        return Err("matrix must be square");
    }
    let mut ipiv = vec![0; m];
    let m_i32 = to_i32(m);
    dgesv(m_i32, 1, &mut a.data, m_i32, &mut ipiv, &mut b.data, m_i32)?;
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use russell_chk::*;

    #[test]
    fn solve_lin_sys_fails_on_non_square() {
        let mut a = Matrix::new(2, 3);
        let mut b = Vector::new(3);
        assert_eq!(solve_lin_sys(&mut b, &mut a), Err("matrix must be square"));
    }

    #[test]
    fn solve_lin_sys_works() -> Result<(), &'static str> {
        #[rustfmt::skip]
        let mut a = Matrix::from(&[
            &[2.0, 1.0, 1.0, 3.0, 2.0],
            &[1.0, 2.0, 2.0, 1.0, 1.0],
            &[1.0, 2.0, 9.0, 1.0, 5.0],
            &[3.0, 1.0, 1.0, 7.0, 1.0],
            &[2.0, 1.0, 5.0, 1.0, 8.0],
        ])?;
        #[rustfmt::skip]
        let mut b = Vector::from(&[
            -2.0,
             4.0,
             3.0,
            -5.0,
             1.0,
        ]);
        solve_lin_sys(&mut b, &mut a)?;
        #[rustfmt::skip]
        let x_correct = Vector::from(&[
            -629.0 / 98.0,
             237.0 / 49.0,
             -53.0 / 49.0,
              62.0 / 49.0,
              23.0 / 14.0,
        ]);
        assert_vec_approx_eq!(b.data, x_correct.data, 1e-13);
        Ok(())
    }
}
