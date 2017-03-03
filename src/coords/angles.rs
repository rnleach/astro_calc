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
//!  - [ ] Unittests for both positive and negative angles.
use std::convert::From;
use std::fmt;

use super::super::error::*;

/// Represent an angle in radians.
#[derive(Debug, Clone, Copy)]
pub struct RadianAngle {
    radians: f64,
}

/// Represent an angle in decimal degrees.
#[derive(Debug, Clone, Copy)]
pub struct DegreeAngle {
    degrees: f64,
}

/// Represent an angle as degrees, minutes, and decimal seconds.
#[derive(Debug, Clone, Copy)]
pub struct DMSAngle {
    degrees: i32,
    minutes: i32,
    seconds: f64,
}

/// Represent an angle in hours, minutes, and seconds.
///
/// This type is forced to be between 0h and 24 hours.
#[derive(Debug, Clone, Copy)]
pub struct HMSAngle {
    hours: i32,
    minutes: i32,
    seconds: f64,
}

/// Common interface for all angle types.
pub trait AngleTrait
    : From<RadianAngle> + From<DegreeAngle> + From<DMSAngle> 
    + From<HMSAngle> + fmt::Display {}

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
    pub fn new(hours: i32, minutes: i32, seconds: f64)-> AstroResult<HMSAngle>{
        if seconds.is_nan() {
            Err(AstroAlgorithmsError::EncounteredNaN)
        } else if seconds.is_infinite() {
            Err(AstroAlgorithmsError::EncounteredInf)
        } else if hours.is_negative() || minutes.is_negative() || seconds.is_sign_negative() {
            Err(AstroAlgorithmsError::EncounteredInappropriateNegativeValue)
        } else if hours > 23 {
            Err(AstroAlgorithmsError::InvalidAngle("Hour limited to range [0,24)".to_owned()))
        } else {
            Ok(HMSAngle{hours: hours, minutes: minutes, seconds: seconds })
        }
    }
}
#[cfg(test)]
mod angle_constructor_tests {
    use super::*;
    
    #[test]
    fn test_radian_angle_new(){
        use std::f64;

        assert_eq!(RadianAngle::new(2.0).unwrap().radians, 2.0);
        assert_eq!(RadianAngle::new(-52872.0).unwrap().radians, -52872.0);
        assert_eq!(RadianAngle::new(f64::NAN).unwrap_err(), 
            AstroAlgorithmsError::EncounteredNaN);
        assert_eq!(RadianAngle::new(-f64::INFINITY).unwrap_err(), 
            AstroAlgorithmsError::EncounteredInf);
    }

    #[test]
    fn test_degree_angle_new(){
        use std::f64;

        assert_eq!(DegreeAngle::new(200.0).unwrap().degrees, 200.0);
        assert_eq!(DegreeAngle::new(-2000.0).unwrap().degrees, -2000.0);
        assert_eq!(DegreeAngle::new(f64::NAN).unwrap_err(), 
            AstroAlgorithmsError::EncounteredNaN);
        assert_eq!(DegreeAngle::new(-f64::INFINITY).unwrap_err(), 
            AstroAlgorithmsError::EncounteredInf);
    }

    #[test]
    fn test_dms_angle_new(){
        use std::f64;

        let test_subject = DMSAngle::new(222, 22, 22.22).unwrap();
        assert_eq!( test_subject.degrees, 222);
        assert_eq!(test_subject.minutes, 22);
        assert_eq!(test_subject.seconds, 22.22);

        let test_subject = DMSAngle::new(-222, 22, 22.22).unwrap();
        assert_eq!( test_subject.degrees, -222);
        assert_eq!(test_subject.minutes, -22);
        assert_eq!(test_subject.seconds, -22.22);

        let test_subject = DMSAngle::new(-222, -22, 22.22).unwrap();
        assert_eq!( test_subject.degrees, -222);
        assert_eq!(test_subject.minutes, -22);
        assert_eq!(test_subject.seconds, -22.22);

        let test_subject = DMSAngle::new(222, -22, 22.22).unwrap();
        assert_eq!( test_subject.degrees, 222);
        assert_eq!(test_subject.minutes, 22);
        assert_eq!(test_subject.seconds, 22.22);

        assert_eq!(DMSAngle::new(222, 22, f64::NAN).unwrap_err(),
            AstroAlgorithmsError::EncounteredNaN);

        assert_eq!(DMSAngle::new(222, 22, f64::INFINITY).unwrap_err(), 
            AstroAlgorithmsError::EncounteredInf);
    }

    #[test]
    fn test_hms_angle_new(){
        use std::f64;

        let test_subject = HMSAngle::new(22, 22, 22.22).unwrap();
        assert_eq!( test_subject.hours, 22);
        assert_eq!(test_subject.minutes, 22);
        assert_eq!(test_subject.seconds, 22.22);
        assert_eq!(HMSAngle::new(22, 22, f64::NAN).unwrap_err(),
            AstroAlgorithmsError::EncounteredNaN);
        assert_eq!(HMSAngle::new(22, 22, f64::INFINITY).unwrap_err(), 
            AstroAlgorithmsError::EncounteredInf);
        assert_eq!(HMSAngle::new(-22, 22, 22.22).unwrap_err(), 
            AstroAlgorithmsError::EncounteredInappropriateNegativeValue);
        assert_eq!(HMSAngle::new(24, 22, 22.22).unwrap_err(), 
            AstroAlgorithmsError::InvalidAngle("Hour limited to range [0,24)".to_owned()));
    }
}

impl fmt::Display for RadianAngle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} radians", self.radians)
    }
}

impl fmt::Display for DegreeAngle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\u{00B0}", self.degrees)
    }
}

impl fmt::Display for DMSAngle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\u{00B0} {}\' {}\"", self.degrees, self.minutes, self.seconds)
    }
}

impl fmt::Display for HMSAngle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}h {}m {}s", self.hours, self.minutes, self.seconds)
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
        let mut decimal_degrees = radians.radians.to_degrees();

        // Force it to be between 0 and 360 degrees
        decimal_degrees = map_to_branch(decimal_degrees, 0.0, 360.0);
        
        let hours = (decimal_degrees / 15.0).trunc();
        let mut remainder = decimal_degrees - (hours * 15.0);
        let minutes = (remainder * 60.0).trunc();
        remainder = remainder - 60.0 * minutes;
        let seconds = remainder * 3600.0;

        HMSAngle {
            hours: hours as i32,
            minutes: minutes as i32,
            seconds: seconds,
        }
    }
}
impl From<DegreeAngle> for HMSAngle {
    fn from(degrees: DegreeAngle) -> Self {
        let decimal_degrees = map_to_branch(degrees.degrees, 0.0, 360.0);
        let hours = (decimal_degrees / 15.0).trunc();
        let mut remainder = decimal_degrees- (hours * 15.0);
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
        let mut decimal_degrees = dms.degrees as f64 + (dms.minutes * 60) as f64 + 3600.0 * dms.seconds;
        decimal_degrees = map_to_branch(decimal_degrees, 0.0, 360.0);
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
mod angle_from_tests {
    use super::*;
    use std::f64::consts::{PI, FRAC_PI_2, FRAC_PI_3, FRAC_PI_4};

    #[test]
    fn test_from_for_radian_angle() {
        assert!(approx_eq(RadianAngle::from(DegreeAngle::new(180.0).unwrap()).radians, PI, 1.0e-15));
        assert!(approx_eq(RadianAngle::from(DegreeAngle::new(-180.0).unwrap()).radians, -PI, 1.0e-15));
        assert!(approx_eq(RadianAngle::from(DegreeAngle::new(90.0).unwrap()).radians, FRAC_PI_2, 1.0e-15));
        assert!(approx_eq(RadianAngle::from(DegreeAngle::new(-90.0).unwrap()).radians, -FRAC_PI_2, 1.0e-15));
        assert!(approx_eq(RadianAngle::from(DegreeAngle::new(60.0).unwrap()).radians, FRAC_PI_3, 1.0e-15));
        assert!(approx_eq(RadianAngle::from(DegreeAngle::new(-60.0).unwrap()).radians, -FRAC_PI_3, 1.0e-15));
        assert!(approx_eq(RadianAngle::from(DegreeAngle::new(45.0).unwrap()).radians, FRAC_PI_4, 1.0e-15));
        assert!(approx_eq(RadianAngle::from(DegreeAngle::new(-45.0).unwrap()).radians, -FRAC_PI_4, 1.0e-15));

        assert!(approx_eq(RadianAngle::from(DMSAngle::new(180,0,0.0).unwrap()).radians, PI, 1.0e-15));
        assert!(approx_eq(RadianAngle::from(DMSAngle::new(-180,0,0.0).unwrap()).radians, -PI, 1.0e-15));
        assert!(approx_eq(RadianAngle::from(DMSAngle::new(90,0,0.0).unwrap()).radians, FRAC_PI_2, 1.0e-15));
        assert!(approx_eq(RadianAngle::from(DMSAngle::new(-90,0,0.0).unwrap()).radians, -FRAC_PI_2, 1.0e-15));
        assert!(approx_eq(RadianAngle::from(DMSAngle::new(60,0,0.0).unwrap()).radians, FRAC_PI_3, 1.0e-15));
        assert!(approx_eq(RadianAngle::from(DMSAngle::new(-60,0,0.0).unwrap()).radians, -FRAC_PI_3, 1.0e-15));
        assert!(approx_eq(RadianAngle::from(DMSAngle::new(45,0,0.0).unwrap()).radians, FRAC_PI_4, 1.0e-15));
        assert!(approx_eq(RadianAngle::from(DMSAngle::new(-45,0,0.0).unwrap()).radians, -FRAC_PI_4, 1.0e-15));

        assert!(approx_eq(RadianAngle::from(HMSAngle::new(12,0,0.0).unwrap()).radians, PI, 1.0e-15));
        assert!(approx_eq(RadianAngle::from(HMSAngle::new(6,0,0.0).unwrap()).radians, FRAC_PI_2, 1.0e-15));
        assert!(approx_eq(RadianAngle::from(HMSAngle::new(4,0,0.0).unwrap()).radians, FRAC_PI_3, 1.0e-15));
        assert!(approx_eq(RadianAngle::from(HMSAngle::new(3,0,0.0).unwrap()).radians, FRAC_PI_4, 1.0e-15));
    }
}

fn map_to_branch(val: f64, min: f64, max: f64) -> f64 {
    let range = max - min;

    if val < min {
        let factor = ((val - min) / range).floor();
        val - factor * range
    } else if val > max {
        let factor = ((val - max) / range).ceil();
        val - factor * range
    } else {
        val
    }
}

// test approximate equality, only used in unit tests.
#[cfg(test)]
fn approx_eq(left: f64, right: f64, tol: f64) -> bool {
    (left - right).abs() < tol
}

#[cfg(test)]
mod angles_tests {
    use super::*;

    #[test]
    fn test_map_to_branch()
    {
        assert!(approx_eq(map_to_branch(-200.0, -180.0, 180.0), 160.0, 1.0e-12));
        assert!(approx_eq(map_to_branch(-200.0, 0.0, 360.0), 160.0, 1.0e-12));
        assert!(approx_eq(map_to_branch(200.0, -180.0, 180.0), -160.0, 1.0e-12));
        assert!(approx_eq(map_to_branch(200.0, 0.0, 360.0), 200.0, 1.0e-12));

        assert!(approx_eq(map_to_branch(-500.0, -180.0, 180.0), -140.0, 1.0e-12));
        assert!(approx_eq(map_to_branch(-500.0, 0.0, 360.0), 220.0, 1.0e-12));
        assert!(approx_eq(map_to_branch(500.0, -180.0, 180.0), 140.0, 1.0e-12));
        assert!(approx_eq(map_to_branch(500.0, 0.0, 360.0), 140.0, 1.0e-12));
    }
}
