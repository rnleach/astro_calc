#![cfg(test)]

// test approximate equality, only used in unit tests.
pub fn approx_eq(left: f64, right: f64, tol: f64) -> bool {
    (left - right).abs() < tol
}
