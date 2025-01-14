use crate::matrix::Matrix;
use crate::vector::Vector;
use crate::StrError;
use russell_openblas::{dgesvd, to_i32};

/// Computes the singular value decomposition (SVD) of a matrix
///
/// Finds `u`, `s`, and `v`, such that:
///
/// ```text
///   a  :=  u   ⋅   s   ⋅   vᵀ
/// (m,n)  (m,m)   (m,n)   (n,n)
/// ```
///
/// # Output
///
/// * `s` -- min(m,n) vector with the diagonal elements
/// * `u` -- (m,m) orthogonal matrix
/// * `vt` -- (n,n) orthogonal matrix with the transpose of v
///
/// # Input
///
/// * `a` -- (m,n) matrix, symmetric or not [will be modified]
///
/// # Note
///
/// 1. The matrix `a` will be modified
///
/// # Examples
///
/// ## First - 2 x 3 rectangular matrix
///
/// ```
/// use russell_lab::{mat_svd, Matrix, Vector, StrError};
///
/// fn main() -> Result<(), StrError> {
///     // set matrix
///     let mut a = Matrix::from(&[
///         [3.0, 2.0,  2.0],
///         [2.0, 3.0, -2.0],
///     ]);
///
///     // allocate output structures
///     let (m, n) = a.dims();
///     let min_mn = if m < n { m } else { n };
///     let mut s = Vector::new(min_mn);
///     let mut u = Matrix::new(m, m);
///     let mut vt = Matrix::new(n, n);
///
///     // perform SVD
///     mat_svd(&mut s, &mut u, &mut vt, &mut a)?;
///
///     // define correct data
///     let s_correct = "┌       ┐\n\
///                      │ 5.000 │\n\
///                      │ 3.000 │\n\
///                      └       ┘";
///
///     // check solution
///     assert_eq!(format!("{:.3}", s), s_correct);
///
///     // check SVD: a == u * s * vt
///     let mut usv = Matrix::new(m, n);
///     for i in 0..m {
///         for j in 0..n {
///             for k in 0..min_mn {
///                 usv.add(i, j, u.get(i, k) * s[k] * vt.get(k, j));
///             }
///         }
///     }
///     let usv_correct = "┌                               ┐\n\
///                        │  3.000000  2.000000  2.000000 │\n\
///                        │  2.000000  3.000000 -2.000000 │\n\
///                        └                               ┘";
///     assert_eq!(format!("{:.6}", usv), usv_correct);
///     Ok(())
/// }
/// ```
///
/// ## Second - 4 x 2 rectangular matrix
///
/// ```
/// use russell_lab::{mat_svd, Matrix, Vector, StrError};
///
/// fn main() -> Result<(), StrError> {
///     // set matrix
///     let mut a = Matrix::from(&[
///         [2.0, 4.0],
///         [1.0, 3.0],
///         [0.0, 0.0],
///         [0.0, 0.0],
///     ]);
///
///     // allocate output structures
///     let (m, n) = a.dims();
///     let min_mn = if m < n { m } else { n };
///     let mut s = Vector::new(min_mn);
///     let mut u = Matrix::new(m, m);
///     let mut vt = Matrix::new(n, n);
///
///     // perform SVD
///     mat_svd(&mut s, &mut u, &mut vt, &mut a)?;
///
///     // define correct data
///     let s_correct = "┌      ┐\n\
///                      │ 5.46 │\n\
///                      │ 0.37 │\n\
///                      └      ┘";
///     let u_correct = "┌                         ┐\n\
///                      │ -0.82 -0.58  0.00  0.00 │\n\
///                      │ -0.58  0.82  0.00  0.00 │\n\
///                      │  0.00  0.00  1.00  0.00 │\n\
///                      │  0.00  0.00  0.00  1.00 │\n\
///                      └                         ┘";
///     let vt_correct = "┌             ┐\n\
///                       │ -0.40 -0.91 │\n\
///                       │ -0.91  0.40 │\n\
///                       └             ┘";
///
///     // check solution
///     assert_eq!(format!("{:.2}", s), s_correct);
///     assert_eq!(format!("{:.2}", u), u_correct);
///     assert_eq!(format!("{:.2}", vt), vt_correct);
///
///     // check SVD: a == u * s * vt
///     let mut usv = Matrix::new(m, n);
///     for i in 0..m {
///         for j in 0..n {
///             for k in 0..min_mn {
///                 usv.add(i, j, u.get(i, k) * s[k] * vt.get(k, j));
///             }
///         }
///     }
///     let usv_correct = "┌     ┐\n\
///                        │ 2 4 │\n\
///                        │ 1 3 │\n\
///                        │ 0 0 │\n\
///                        │ 0 0 │\n\
///                        └     ┘";
///     assert_eq!(format!("{}", usv), usv_correct);
///     Ok(())
/// }
/// ```
pub fn mat_svd(s: &mut Vector, u: &mut Matrix, vt: &mut Matrix, a: &mut Matrix) -> Result<(), StrError> {
    let (m, n) = a.dims();
    let min_mn = if m < n { m } else { n };
    if s.dim() != min_mn {
        return Err("[s] must be an min(m,n) vector");
    }
    if u.nrow() != m || u.ncol() != m {
        return Err("[u] must be an m-by-m square matrix");
    }
    if vt.nrow() != n || vt.ncol() != n {
        return Err("[vt] must be an n-by-n square matrix");
    }
    let m_i32 = to_i32(m);
    let n_i32 = to_i32(n);
    let mut superb = vec![0.0; min_mn];
    dgesvd(
        b'A',
        b'A',
        m_i32,
        n_i32,
        a.as_mut_data(),
        s.as_mut_data(),
        u.as_mut_data(),
        vt.as_mut_data(),
        &mut superb,
    )
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use russell_chk::vec_approx_eq;

    use super::{mat_svd, Matrix, Vector};
    use crate::mat_approx_eq;

    #[test]
    fn mat_svd_fails_on_wrong_dims() {
        let mut a = Matrix::new(3, 2);
        let mut s = Vector::new(2);
        let mut u = Matrix::new(3, 3);
        let mut vt = Matrix::new(2, 2);
        let mut s_3 = Vector::new(3);
        let mut u_2x2 = Matrix::new(2, 2);
        let mut u_3x2 = Matrix::new(3, 2);
        let mut vt_3x3 = Matrix::new(3, 3);
        let mut vt_2x3 = Matrix::new(2, 3);
        assert_eq!(
            mat_svd(&mut s_3, &mut u, &mut vt, &mut a),
            Err("[s] must be an min(m,n) vector")
        );
        assert_eq!(
            mat_svd(&mut s, &mut u_2x2, &mut vt, &mut a),
            Err("[u] must be an m-by-m square matrix")
        );
        assert_eq!(
            mat_svd(&mut s, &mut u_3x2, &mut vt, &mut a),
            Err("[u] must be an m-by-m square matrix")
        );
        assert_eq!(
            mat_svd(&mut s, &mut u, &mut vt_3x3, &mut a),
            Err("[vt] must be an n-by-n square matrix")
        );
        assert_eq!(
            mat_svd(&mut s, &mut u, &mut vt_2x3, &mut a),
            Err("[vt] must be an n-by-n square matrix")
        );
    }

    #[test]
    fn mat_svd_works() {
        // matrix
        let s33 = f64::sqrt(3.0) / 3.0;
        #[rustfmt::skip]
        let data = [
            [-s33, -s33, 1.0],
            [ s33, -s33, 1.0],
            [-s33,  s33, 1.0],
            [ s33,  s33, 1.0],
        ];
        let mut a = Matrix::from(&data);
        let a_copy = Matrix::from(&data);

        // allocate output data
        let (m, n) = a.dims();
        let min_mn = if m < n { m } else { n };
        let mut s = Vector::new(min_mn);
        let mut u = Matrix::new(m, m);
        let mut vt = Matrix::new(n, n);

        // calculate SVD
        mat_svd(&mut s, &mut u, &mut vt, &mut a).unwrap();

        // check
        #[rustfmt::skip]
        let s_correct = &[
            2.0,
            2.0 / f64::sqrt(3.0),
            2.0 / f64::sqrt(3.0),
        ];
        #[rustfmt::skip]
        let u_correct = &[
            [-0.5, -0.5, -0.5,  0.5],
            [-0.5, -0.5,  0.5, -0.5],
            [-0.5,  0.5, -0.5, -0.5],
            [-0.5,  0.5,  0.5,  0.5],
        ];
        #[rustfmt::skip]
        let vt_correct =&[
            [0.0,  0.0, -1.0],
            [0.0,  1.0,  0.0],
            [1.0,  0.0,  0.0],
        ];
        mat_approx_eq(&u, u_correct, 1e-15);
        vec_approx_eq(s.as_data(), s_correct, 1e-15);
        mat_approx_eq(&vt, vt_correct, 1e-15);

        // check SVD
        let mut usv = Matrix::new(m, n);
        for i in 0..m {
            for j in 0..n {
                for k in 0..min_mn {
                    usv.add(i, j, u.get(i, k) * s[k] * vt.get(k, j));
                }
            }
        }
        mat_approx_eq(&usv, &a_copy, 1e-15);
    }

    #[test]
    fn mat_svd_1_works() {
        // matrix
        #[rustfmt::skip]
        let data = [
            [1.0, 0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0, 1.0],
        ];
        let mut a = Matrix::from(&data);
        let a_copy = Matrix::from(&data);

        // allocate output data
        let (m, n) = a.dims();
        let min_mn = if m < n { m } else { n };
        let mut s = Vector::new(min_mn);
        let mut u = Matrix::new(m, m);
        let mut vt = Matrix::new(n, n);

        // calculate SVD
        mat_svd(&mut s, &mut u, &mut vt, &mut a).unwrap();

        // check
        let sqrt2 = std::f64::consts::SQRT_2;
        #[rustfmt::skip]
        let s_correct = &[
            sqrt2,
            sqrt2,
        ];
        #[rustfmt::skip]
        let u_correct = &[
            [1.0, 0.0],
            [0.0, 1.0],
        ];
        #[rustfmt::skip]
        let vt_correct = &[
            [ 1.0/sqrt2,        0.0, 1.0/sqrt2,       0.0],
            [       0.0,  1.0/sqrt2,       0.0, 1.0/sqrt2],
            [-1.0/sqrt2,        0.0, 1.0/sqrt2,       0.0],
            [       0.0, -1.0/sqrt2,       0.0, 1.0/sqrt2],
        ];
        mat_approx_eq(&u, u_correct, 1e-15);
        vec_approx_eq(s.as_data(), s_correct, 1e-15);
        mat_approx_eq(&vt, vt_correct, 1e-15);

        // check SVD
        let mut usv = Matrix::new(m, n);
        for i in 0..m {
            for j in 0..n {
                for k in 0..min_mn {
                    usv.add(i, j, u.get(i, k) * s[k] * vt.get(k, j));
                }
            }
        }
        mat_approx_eq(&usv, &a_copy, 1e-15);
    }
}
