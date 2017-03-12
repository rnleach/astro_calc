#![warn(missing_docs)]
//!
//! # Astronomical Algorithms and Calculations.
//!
//! Authors: Ryan Leach
//!
//! Copyright: Ryan Leach, 2017
//!
//! License: [BSD 3-clause](https://opensource.org/licenses/BSD-3-Clause)
//!
//! # Library for astronomical calculations.
//! I'm implementing many of the algorithms from
//! ["Astronomical Algorithms, 2nd Ed." by Jean Meeus]
//! (https://www.amazon.com/Astronomical-Algorithms-Jean-Meeus/dp/0943396611/ref=sr_1_1?ie=UTF8&qid=1486964675&sr=8-1&keywords=astronomical+algorithms).
//!

#[macro_use]
extern crate lazy_static;

// Public export modules
pub mod error;
pub mod angles;
pub mod astro_time;
pub mod coords;

// Private modules
mod test_util;
