//!
//! Module contains library wide utilities and constants.
//!
//! Authors: Ryan Leach
//! Copyright: Ryan Leach, 2017
//! License: BSD 3-clause, https://opensource.org/licenses/BSD-3-Clause
//!
use std::result;

#[allow(missing_docs)]
#[derive(Debug)]
pub enum AstroAlgorithmsError {

}

#[allow(missing_docs)]
pub type Result<T> = result::Result<T, AstroAlgorithmsError>;
