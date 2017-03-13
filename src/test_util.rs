#![cfg(test)]
//!
//! Utility functions for tests.
//!
//! Authors: Ryan Leach
//!
//! Copyright: Ryan Leach, 2017
//!
//! License: [BSD 3-clause](https://opensource.org/licenses/BSD-3-Clause)
//!

// test approximate equality, only used in unit tests.
pub fn approx_eq(left: f64, right: f64, tol: f64) -> bool {
    (left - right).abs() < tol
}
