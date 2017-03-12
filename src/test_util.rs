#![cfg(test)]
//!
//! Utility functions for working with tests.
//!
//! Authors: Ryan Leach
//!
//! Copyright: Ryan Leach, 2017
//!
//! License: [BSD 3-clause](https://opensource.org/licenses/BSD-3-Clause)
//!
//! All of the coordinates carry a valid time with them. This is the epoch. The epoch may be the
//! standard epochs of 1950 or 2000, or it could be any other date.

// test approximate equality, only used in unit tests.
pub fn approx_eq(left: f64, right: f64, tol: f64) -> bool {
    (left - right).abs() < tol
}
