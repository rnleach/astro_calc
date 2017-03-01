//!
//! Module for dealing with angles in astronomical calculations.
//!
//! Authors: Ryan Leach
//!
//! Copyright: Ryan Leach, 2017
//!
//! License: [BSD 3-clause](https://opensource.org/licenses/BSD-3-Clause)
//!
//! TODO
//!  - [ ] Factor out common functions, there is a lot of repeated code
//!  - [ ] Use is_sign_positive to force DMS to all have the same sign.
//!  - [ ] Do not allow negative HMS
//!  - [ ] Constructors
//!  - [ ] Unittests for both positive and negative angles.
//!  - [ ] Documentation comments
use std::convert::From;

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
trait Angle
    : From<RadianAngle> + From<DegreeAngle> + From<DMSAngle> + From<HMSAngle> {
    // TODO
    // Setters.
}

impl Angle for RadianAngle {}
impl Angle for DegreeAngle {}
impl Angle for DMSAngle {}
impl Angle for HMSAngle {}

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