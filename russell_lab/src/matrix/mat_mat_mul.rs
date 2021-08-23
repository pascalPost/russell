use super::*;
use russell_openblas::*;
use std::convert::TryInto;

/// Performs the matrix-matrix multiplication resulting in a matrix
///
/// ```text
///   c  := alpha *  a   multiply   b
/// (m,n)          (m,k)          (k,n)
/// ```
///
/// # Panics
///
/// This function panics if the matrix dimensions are incorrect
///
/// # Examples
///
/// ```
/// use russell_lab::*;
/// let a = Matrix::from(&[
///     &[1.0, 2.0],
///     &[3.0, 4.0],
///     &[5.0, 6.0],
/// ]);
/// let b = Matrix::from(&[
///     &[-1.0, -2.0, -3.0],
///     &[-4.0, -5.0, -6.0],
/// ]);
/// let mut c = Matrix::new(3, 3);
/// mat_mat_mul(&mut c, 1.0, &a, &b);
/// let correct = "┌             ┐\n\
///                │  -9 -12 -15 │\n\
///                │ -19 -26 -33 │\n\
///                │ -29 -40 -51 │\n\
///                └             ┘";
/// assert_eq!(format!("{}", c), correct);
/// ```
///
pub fn mat_mat_mul(c: &mut Matrix, alpha: f64, a: &Matrix, b: &Matrix) {
    if a.nrow != c.nrow {
        panic!("the number of rows of matrix [a] (={}) must be equal to the number of rows of matrix [c] (={})", a.nrow, c.nrow);
    }
    if b.nrow != a.ncol {
        panic!("the number of rows of matrix [b] (={}) must be equal to the number of columns of matrix [a] (={})", b.nrow, a.ncol);
    }
    if b.ncol != c.ncol {
        panic!("the number of columns of matrix [b] (={}) must be equal to the number of columns of matrix [c] (={})", b.ncol, c.ncol);
    }
    let m_i32: i32 = c.nrow.try_into().unwrap();
    let n_i32: i32 = c.ncol.try_into().unwrap();
    let k_i32: i32 = a.ncol.try_into().unwrap();
    let lda_i32: i32 = a.nrow.try_into().unwrap();
    let ldb_i32: i32 = b.nrow.try_into().unwrap();
    dgemm(
        false,
        false,
        m_i32,
        n_i32,
        k_i32,
        alpha,
        &a.data,
        lda_i32,
        &b.data,
        ldb_i32,
        0.0,
        &mut c.data,
        m_i32,
    );
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use russell_chk::*;

    #[test]
    fn mat_mat_mul_works() {
        let a = Matrix::from(&[
            // 2 x 3
            &[1.0, 2.00, 3.0],
            &[0.5, 0.75, 1.5],
        ]);
        let b = Matrix::from(&[
            // 3 x 4
            &[0.1, 0.5, 0.5, 0.75],
            &[0.2, 2.0, 2.0, 2.00],
            &[0.3, 0.5, 0.5, 0.50],
        ]);
        let mut c = Matrix::new(2, 4);
        // c := 2⋅a⋅b
        mat_mat_mul(&mut c, 2.0, &a, &b);
        #[rustfmt::skip]
        let correct =slice_to_colmajor(&[
            &[2.80, 12.0, 12.0, 12.50],
            &[1.30,  5.0,  5.0, 5.25],
        ]);
        assert_vec_approx_eq!(c.data, correct, 1e-15);
    }
}