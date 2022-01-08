use std::cmp::Ordering;

pub const EPSILON: f64 = 1e-8;

/// Compare floats with a hardcoded precision of
/// 5 significant digits.
pub fn approx_equal(a: f64, b: f64) -> bool {
    let dp = 5;
    let p = 10f64.powi(-(dp as i32));
    (a - b).abs() < p
}

/// Weird function only used for computing MatrixN::submatrix
pub fn cmp_to_offset(ordering_row: Ordering, ordering_col: Ordering) -> Option<(i32, i32)> {
    match (ordering_row, ordering_col) {
        (Ordering::Equal, _) => None,
        (_, Ordering::Equal) => None,
        (Ordering::Greater, Ordering::Greater) => Some((-1, -1)),
        (Ordering::Less, Ordering::Greater) => Some((0, -1)),
        (Ordering::Greater, Ordering::Less) => Some((-1, 0)),
        (Ordering::Less, Ordering::Less) => Some((0, 0)),
    }
}
