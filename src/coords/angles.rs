//!
//! Module for dealing with angles in astronomical calculations.
//!
//! Authors: Ryan Leach
//!
//! Copyright: Ryan Leach, 2017
//!
//! License: [BSD 3-clause](https://opensource.org/licenses/BSD-3-Clause)
//!
//! Most internal calculations will be done using the RadianAngle type, since
//! most computer functions work with radians. The other types are mainly there
//! for ease of use and formatting with input and output.
//!
//! TODO
//!  - [ ] Factor out common functions, there is a lot of repeated code in From traits
//!  - [ ] Use constructors in from and force wrapping?
//!  - [ ] Do not allow negative HMS
//!  - [ ] Unittests for both positive and negative angles.
//!  - [ ] Implement `Display` for each of the structs.
//!  - [ ] Documentation comments
use std::convert::From;

use super::super::error::*;

#[derive(Debug, Clone, Copy)]
pub struct RadianAngle {
    radians: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct DegreeAngle {
    degrees: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct DMSAngle {
    degrees: i32,
    minutes: i32,
    seconds: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct HMSAngle {
    hours: i32,
    minutes: i32,
    seconds: f64,
}

/// Common interface for all angle types.
pub trait AngleTrait
    : From<RadianAngle> + From<DegreeAngle> + From<DMSAngle> + From<HMSAngle> {
    // TODO
    // Getters for each format.
}

impl AngleTrait for RadianAngle {}
impl AngleTrait for DegreeAngle {}
impl AngleTrait for DMSAngle {}
impl AngleTrait for HMSAngle {}

impl RadianAngle {
    /// Create a new angle using radians.
    pub fn new(radians: f64)-> AstroResult<RadianAngle>{
        if radians.is_nan() {
            Err(AstroAlgorithmsError::EncounteredNaN)
        } else if radians.is_infinite() {
            Err(AstroAlgorithmsError::EncounteredInf)
        } else {
            Ok(RadianAngle{radians: radians })
        }
    }
}

impl DegreeAngle {
    /// Create a new angle using degrees.
    pub fn new(degrees: f64)-> AstroResult<DegreeAngle>{
        if degrees.is_nan() {
            Err(AstroAlgorithmsError::EncounteredNaN)
        } else if degrees.is_infinite() {
            Err(AstroAlgorithmsError::EncounteredInf)
        } else {
            Ok(DegreeAngle{degrees: degrees })
        }
    }
}

impl DMSAngle {
    /// Create a new angle using degrees, minutes, seconds.
    pub fn new(degrees: i32, mut minutes: i32, mut seconds: f64)-> AstroResult<DMSAngle>{
        if degrees.is_negative() {
            if minutes.is_positive() {
                minutes *= -1;
            }
            if seconds.is_sign_positive() {
                seconds *= -1.0;
            }
        } else {
            if minutes.is_negative() {
                minutes *= -1;
            }
            if seconds.is_sign_negative() {
                seconds *= -1.0;
            }
        }
        if seconds.is_nan() {
            Err(AstroAlgorithmsError::EncounteredNaN)
        } else if seconds.is_infinite() {
            Err(AstroAlgorithmsError::EncounteredInf)
        } else {
            Ok(DMSAngle{degrees: degrees, minutes: minutes, seconds: seconds })
        }
    }
}

impl HMSAngle {
    /// Create a new angle using hours, minutes, seconds.
    pub fn new(hours: i32, mut minutes: i32, mut seconds: f64)-> AstroResult<HMSAngle>{
        if seconds.is_nan() {
            Err(AstroAlgorithmsError::EncounteredNaN)
        } else if seconds.is_infinite() {
            Err(AstroAlgorithmsError::EncounteredInf)
        } else if hours.is_negative() || minutes.is_negative() || seconds.is_sign_negative() {
            Err(AstroAlgorithmsError::EncounteredInappropriateNegativeValue)
        } else {
            Ok(HMSAngle{hours: hours, minutes: minutes, seconds: seconds })
        }
    }
}

impl From<DegreeAngle> for RadianAngle {
    fn from(degrees: DegreeAngle) -> Self {
        RadianAngle { radians: degrees.degrees.to_radians() }
    }
}
impl From<DMSAngle> for RadianAngle {
    fn from(dms: DMSAngle) -> Self {
        let degrees = dms.degrees as f64 + (60 * dms.minutes) as f64 + 3600.0 * dms.seconds;
        RadianAngle { radians: degrees.to_radians() }
    }
}
impl From<HMSAngle> for RadianAngle {
    fn from(hms: HMSAngle) -> Self {
        let degrees = (hms.hours * 15) as f64 + (60 * hms.minutes) as f64 + 3600.0 * hms.seconds;
        RadianAngle { radians: degrees.to_radians() }
    }
}

impl From<RadianAngle> for DegreeAngle {
    fn from(radians: RadianAngle) -> Self {
        DegreeAngle { degrees: radians.radians.to_degrees() }
    }
}
impl From<DMSAngle> for DegreeAngle {
    fn from(dms: DMSAngle) -> Self {
        let degrees = dms.degrees as f64 + (60 * dms.minutes) as f64 + 3600.0 * dms.seconds;
        DegreeAngle { degrees: degrees }
    }
}
impl From<HMSAngle> for DegreeAngle {
    fn from(hms: HMSAngle) -> Self {
        let degrees = (hms.hours * 15) as f64 + (60 * hms.minutes) as f64 + 3600.0 * hms.seconds;
        DegreeAngle { degrees: degrees }
    }
}

impl From<RadianAngle> for DMSAngle {
    fn from(radians: RadianAngle) -> Self {
        let decimal_degrees = radians.radians.to_degrees();
        let degrees = decimal_degrees.trunc();
        let mut remainder = decimal_degrees - degrees;
        let minutes = (remainder * 60.0).trunc();
        remainder = remainder - 60.0 * minutes;
        let seconds = remainder * 3600.0;

        DMSAngle {
            degrees: degrees as i32,
            minutes: minutes.abs() as i32,
            seconds: seconds.abs(),
        }
    }
}
impl From<DegreeAngle> for DMSAngle {
    fn from(decimal_degrees: DegreeAngle) -> Self {
        let degrees = decimal_degrees.degrees.trunc();
        let mut remainder = decimal_degrees.degrees - degrees;
        let minutes = (remainder * 60.0).trunc();
        remainder = remainder - 60.0 * minutes;
        let seconds = remainder * 3600.0;

        DMSAngle {
            degrees: degrees as i32,
            minutes: minutes.abs() as i32,
            seconds: seconds.abs(),
        }
    }
}
impl From<HMSAngle> for DMSAngle {
    fn from(hms: HMSAngle) -> Self {
        let decimal_degrees = (hms.hours * 15) as f64 + (60 * hms.minutes) as f64 +
                              3600.0 * hms.seconds;
        let degrees = decimal_degrees.trunc();
        let mut remainder = decimal_degrees - degrees;
        let minutes = (remainder * 60.0).trunc();
        remainder = remainder - 60.0 * minutes;
        let seconds = remainder * 3600.0;

        DMSAngle {
            degrees: degrees as i32,
            minutes: minutes.abs() as i32,
            seconds: seconds.abs(),
        }
    }
}

impl From<RadianAngle> for HMSAngle {
    fn from(radians: RadianAngle) -> Self {
        let decimal_degrees = radians.radians.to_degrees();
        let hours = (decimal_degrees / 15.0).trunc();
        let mut remainder = decimal_degrees - (hours * 15.0);
        let minutes = (remainder * 60.0).trunc();
        remainder = remainder - 60.0 * minutes;
        let seconds = remainder * 3600.0;

        HMSAngle {
            hours: hours as i32,
            minutes: minutes.abs() as i32,
            seconds: seconds.abs(),
        }
    }
}
impl From<DegreeAngle> for HMSAngle {
    fn from(decimal_degrees: DegreeAngle) -> Self {
        let hours = (decimal_degrees.degrees / 15.0).trunc();
        let mut remainder = decimal_degrees.degrees - (hours * 15.0);
        let minutes = (remainder * 60.0).trunc();
        remainder = remainder - 60.0 * minutes;
        let seconds = remainder * 3600.0;

        HMSAngle {
            hours: hours as i32,
            minutes: minutes.abs() as i32,
            seconds: seconds.abs(),
        }
    }
}
impl From<DMSAngle> for HMSAngle {
    fn from(dms: DMSAngle) -> Self {
        let decimal_degrees = dms.degrees as f64 + (dms.minutes * 60) as f64 + 3600.0 * dms.seconds;
        let hours = (decimal_degrees / 15.0).trunc();
        let mut remainder = decimal_degrees - (hours * 15.0);
        let minutes = (remainder * 60.0).trunc();
        remainder = remainder - 60.0 * minutes;
        let seconds = remainder * 3600.0;

        HMSAngle {
            hours: hours as i32,
            minutes: minutes.abs() as i32,
            seconds: seconds.abs(),
        }
    }
}

#[cfg(test)]
mod angles_tests {

}
