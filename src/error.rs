//!
//! Module contains library wide error values.
//!
//! Authors: Ryan Leach
//!
//! Copyright: Ryan Leach, 2017
//!
//! License: [BSD 3-clause](https://opensource.org/licenses/BSD-3-Clause)
//!
use std::result;

#[allow(missing_docs)]
#[derive(Debug, PartialEq)]
pub enum AstroAlgorithmsError {
    /// Indicate the range of allowable dates was exceeded by an algorithm or type. Most algorithms
    /// do not accept dates with a Julian Date before 0.0.
    DateRange,

    /// Invalid values supplied to a method or function for a Gregorian calendar
    /// date. For example, 29 Feb 2017 and 31 Apr 1981 are dates that do not
    /// exist. Values are year, month, day.
    InvalidGregorianDate,

    /// Same as `InvalidGregorianDate`, but uses the Julian calendar.
    InvalidJulianDate,

    /// Invalid arguments used for a time. Values are hours, minutes, seconds
    InvalidTime,

    /// Invalid angle. Some algorithms and types put restrictions on the allowed
    /// ranges for angles, the string should provide more context.
    InvalidAngle,

    /// Aborted due to encountering a NaN (Not a Number) with floating point
    /// numbers.
    EncounteredNaN,

    /// Aborted due to encountering infinite value in floating point numbers.
    EncounteredInf,

    /// A number in an inappropriate range for the a type or algorithm was used.
    Range,

    /// No error type created for this yet.
    Unspecified,
}

#[allow(missing_docs)]
pub type AstroResult<T> = result::Result<T, AstroAlgorithmsError>;
