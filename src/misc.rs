/// Compare floats with a hardcoded precision of
/// 5 significant digits.
pub fn approx_equal(a: f64, b: f64) -> bool {
    let dp = 5;
    let p = 10f64.powi(-(dp as i32));
    (a - b).abs() < p
}
