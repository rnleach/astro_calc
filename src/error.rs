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
#[derive(Debug)]
pub enum AstroAlgorithmsError {
    
    /// Indicate the range of allowable dates was exceeded by an algorithm or
    /// type.
    RangeError ( DateRangeError ),
    
    /// Invalid values supplied to a method or function for a Gregorian calendar
    /// date. For example, 29 Feb 2017 and 31 Apr 1981 are dates that do not
    /// exist.
    InvalidGregorianDate,
    
    /// Same as `InvalidGregorianDate`, but uses the Julian calendar.
    InvalidJulianDate,

    UnspecifiedError,
}

/// An error indicating that the date was either too early or too late for the 
/// algorithm or type that was using it.
#[derive(Debug)]
pub enum DateRangeError {
    
    /// Most algorithms and types in this library are only valid for with 
    /// Julian date greater than `0.0` (which corresponds to proleptic 12 noon 
    /// on 1 Jan, -4712 in the Julian calendar). This error indicates that was
    /// violated somewhere. The first value contained in this type indicates the 
    /// date that triggered the error, and the second is the threshold that was
    /// not met, which is usually 0.0, but may be different for some algorithms.
    DateUnderflow(f64, f64),

    /// Many algorithms have limits on the date range they are applicable. This
    /// error indicates that limit has been surpassed. The first value contained
    /// in this type indicates the date value that triggered the error, and the
    /// second is the threshold that was exceeded.
    DateOverflow(f64, f64),
}

#[allow(missing_docs)]
pub type AstroResult<T> = result::Result<T, AstroAlgorithmsError>;
