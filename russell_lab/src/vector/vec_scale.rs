use super::Vector;
use russell_openblas::{dscal, to_i32};

/// Scales vector
///
/// ```text
/// u := alpha * u
/// ```
///
/// # Example
///
/// ```
/// use russell_lab::{vec_scale, Vector};
///
/// fn main() {
///     let mut u = Vector::from(&[1.0, 2.0, 3.0]);
///     vec_scale(&mut u, 0.5);
///     let correct = "┌     ┐\n\
///                    │ 0.5 │\n\
///                    │   1 │\n\
///                    │ 1.5 │\n\
///                    └     ┘";
///     assert_eq!(format!("{}", u), correct);
/// }
/// ```
pub fn vec_scale(v: &mut Vector, alpha: f64) {
    let n_i32: i32 = to_i32(v.dim());
    dscal(n_i32, alpha, v.as_mut_data(), 1);
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{vec_scale, Vector};
    use russell_chk::vec_approx_eq;

    #[test]
    fn vec_scale_works() {
        let mut u = Vector::from(&[6.0, 9.0, 12.0]);
        vec_scale(&mut u, 1.0 / 3.0);
        let correct = &[2.0, 3.0, 4.0];
        vec_approx_eq(u.as_data(), correct, 1e-15);
    }
}
