use crate::matrix::Matrix;
use crate::vector::Vector;
use crate::StrError;
use russell_openblas::{dgemv, to_i32};

/// Performs the vector-matrix multiplication resulting in a vector
///
/// ```text
///  v  :=  α ⋅  u  ⋅  a  
/// (n)         (m)  (m,n)
/// ```
///
/// or
///
/// ```text
///  v  :=  α ⋅   aᵀ  ⋅  u
/// (n)         (n,m)   (m)  
/// ```
///
/// # Note
///
/// The length of vector `u` must equal the number of rows of matrix `a` and
/// the length of vector `v` must equal the number of columns of matrix `a`.
///
/// # Example
///
/// ```
/// use russell_lab::{vec_mat_mul, Matrix, Vector, StrError};
///
/// fn main() -> Result<(), StrError> {
///     let a = Matrix::from(&[
///         [ 5.0, -2.0, 1.0],
///         [-4.0,  0.0, 2.0],
///         [15.0, -6.0, 0.0],
///         [ 3.0,  5.0, 1.0],
///     ]);
///     let u = Vector::from(&[1.0, 2.0, 3.0, 4.0]);
///     let mut v = Vector::new(a.ncol());
///     vec_mat_mul(&mut v, 0.5, &u, &a)?;
///     let correct = "┌     ┐\n\
///                    │  27 │\n\
///                    │   0 │\n\
///                    │ 4.5 │\n\
///                    └     ┘";
///     assert_eq!(format!("{}", v), correct);
///     Ok(())
/// }
/// ```
pub fn vec_mat_mul(v: &mut Vector, alpha: f64, u: &Vector, a: &Matrix) -> Result<(), StrError> {
    let n = v.dim();
    let m = u.dim();
    if m != a.nrow() || n != a.ncol() {
        return Err("matrix and vectors are incompatible");
    }
    if m == 0 || n == 0 {
        return Ok(());
    }
    let m_i32: i32 = to_i32(m);
    let n_i32: i32 = to_i32(n);
    dgemv(
        true,
        m_i32,
        n_i32,
        alpha,
        a.as_data(),
        u.as_data(),
        1,
        0.0,
        v.as_mut_data(),
        1,
    );
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{vec_mat_mul, Matrix, Vector};
    use russell_chk::vec_approx_eq;

    #[test]
    fn vec_mat_mul_fails_on_wrong_dims() {
        let u = Vector::new(2);
        let a_1x2 = Matrix::new(1, 2);
        let a_3x1 = Matrix::new(3, 1);
        let mut v = Vector::new(3);
        assert_eq!(
            vec_mat_mul(&mut v, 1.0, &u, &a_1x2),
            Err("matrix and vectors are incompatible")
        );
        assert_eq!(
            vec_mat_mul(&mut v, 1.0, &u, &a_3x1),
            Err("matrix and vectors are incompatible")
        );
    }

    #[test]
    fn vec_mat_mul_works() {
        #[rustfmt::skip]
        let a = Matrix::from(&[
            [ 5.0, -2.0, 0.0, 1.0],
            [10.0, -4.0, 0.0, 2.0],
            [15.0, -6.0, 0.0, 3.0],
        ]);
        let u = Vector::from(&[1.0, 3.0, 8.0]);
        let mut v = Vector::new(a.ncol());
        vec_mat_mul(&mut v, 1.0, &u, &a).unwrap();
        let correct = &[155.0, -62.0, 0.0, 31.0];
        vec_approx_eq(v.as_data(), correct, 1e-15);
    }

    #[test]
    fn vec_mat_mul_zero_works() {
        let a_0x0 = Matrix::new(0, 0);
        let a_0x1 = Matrix::new(0, 1);
        let a_1x0 = Matrix::new(1, 0);
        let u0 = Vector::new(0);
        let u1 = Vector::new(1);
        let mut v0 = Vector::new(0);
        let mut v1 = Vector::new(1);
        vec_mat_mul(&mut v0, 1.0, &u0, &a_0x0).unwrap();
        assert_eq!(v0.as_data(), &[] as &[f64]);
        vec_mat_mul(&mut v1, 1.0, &u0, &a_0x1).unwrap();
        assert_eq!(v1.as_data(), &[0.0]);
        vec_mat_mul(&mut v0, 1.0, &u1, &a_1x0).unwrap();
        assert_eq!(v0.as_data(), &[] as &[f64]);
    }
}
