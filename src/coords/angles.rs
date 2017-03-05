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
use std::convert::From;
use std::fmt;
use std::ops;

// TODO tests with minutes and seconds for the conversions!
// TODO map to branch trait, use 0 to 360 and -180 to 180 branches

/// Represent an angle in radians.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RadianAngle {
    radians: f64,
}

/// Represent an angle in decimal degrees.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DegreeAngle {
    degrees: f64,
}

/// Represent an angle as degrees, minutes, and decimal seconds.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DMSAngle {
    degrees: i32,
    minutes: i32,
    seconds: f64,
}

/// Represent an angle in hours, minutes, and seconds.
///
/// This type is forced to be between 0h and 24 hours.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HMSAngle {
    hours: i32,
    minutes: i32,
    seconds: f64,
}

/// Common interface for all angle types.
pub trait Angle
    : From<RadianAngle> + From<DegreeAngle> + From<DMSAngle> + From<HMSAngle> + fmt::Display +
    ops::Add<RadianAngle> + ops::Add<DegreeAngle> +ops::Add<DMSAngle> +ops::Add<HMSAngle> +
    ops::Sub<RadianAngle> + ops::Sub<DegreeAngle> +ops::Sub<DMSAngle> +ops::Sub<HMSAngle>
    {
}

impl Angle for RadianAngle {}
impl Angle for DegreeAngle {}
impl Angle for DMSAngle {}
impl Angle for HMSAngle {}

impl RadianAngle {
    /// Create a new angle using radians.
    pub fn new(radians: f64) -> RadianAngle {
        RadianAngle { radians: radians }
    }

    /// Get the value in radians as an f64
    pub fn radians(&self) -> f64 {
        self.radians
    }
}

impl DegreeAngle {
    /// Create a new angle using degrees.
    pub fn new(degrees: f64) -> DegreeAngle {
        DegreeAngle { degrees: degrees }
    }

    /// Get the value in degrees as an f64
    pub fn degrees(&self) -> f64 {
        self.degrees
    }
}

impl DMSAngle {
    /// Create a new angle using degrees, minutes, seconds.
    pub fn new(degrees: i32, mut minutes: i32, mut seconds: f64) -> DMSAngle {
        if degrees < 0 {
            minutes = -minutes.abs();
            seconds = -seconds.abs();
        } else if degrees > 0 {
            minutes = minutes.abs();
            seconds = seconds.abs();
        } else if minutes < 0 {
            seconds = -seconds.abs();
        } else if minutes > 0 {
            seconds = seconds.abs();
        }

        DMSAngle {
            degrees: degrees,
            minutes: minutes,
            seconds: seconds,
        }
    }
}

impl HMSAngle {
    /// Create a new angle using hours, minutes, seconds.
    pub fn new(hours: i32, mut minutes: i32, mut seconds: f64) -> HMSAngle {
        if hours < 0 {
            minutes = -minutes.abs();
            seconds = -seconds.abs();
        } else if hours > 0 {
            minutes = minutes.abs();
            seconds = seconds.abs();
        } else if minutes < 0 {
            seconds = -seconds.abs();
        } else if minutes > 0 {
            seconds = seconds.abs();
        }
        HMSAngle {
            hours: hours,
            minutes: minutes,
            seconds: seconds,
        }
    }
}

#[cfg(test)]
mod angle_constructor_tests {
    use super::*;

    #[test]
    fn test_radian_angle_new() {

        assert_eq!(RadianAngle::new(2.0).radians, 2.0);
        assert_eq!(RadianAngle::new(-52872.0).radians, -52872.0);
    }

    #[test]
    fn test_degree_angle_new() {

        assert_eq!(DegreeAngle::new(200.0).degrees, 200.0);
        assert_eq!(DegreeAngle::new(-2000.0).degrees, -2000.0);
    }

    #[test]
    fn test_dms_angle_new() {

        let test_subject = DMSAngle::new(222, 22, 22.22);
        assert_eq!(test_subject.degrees, 222);
        assert_eq!(test_subject.minutes, 22);
        assert_eq!(test_subject.seconds, 22.22);

        let test_subject = DMSAngle::new(-222, 22, 22.22);
        assert_eq!(test_subject.degrees, -222);
        assert_eq!(test_subject.minutes, -22);
        assert_eq!(test_subject.seconds, -22.22);

        let test_subject = DMSAngle::new(-222, -22, 22.22);
        assert_eq!(test_subject.degrees, -222);
        assert_eq!(test_subject.minutes, -22);
        assert_eq!(test_subject.seconds, -22.22);

        let test_subject = DMSAngle::new(222, -22, 22.22);
        assert_eq!(test_subject.degrees, 222);
        assert_eq!(test_subject.minutes, 22);
        assert_eq!(test_subject.seconds, 22.22);
    }

    #[test]
    fn test_hms_angle_new() {

        let test_subject = HMSAngle::new(22, 22, 22.22);
        assert_eq!(test_subject.hours, 22);
        assert_eq!(test_subject.minutes, 22);
        assert_eq!(test_subject.seconds, 22.22);
    }
}

/// Create addition, subtraction operators for angles.
macro_rules! make_add_sub_operators_for {
    (RadianAngle, $rhs:ty) => (
        impl ops::Add<$rhs> for RadianAngle {
            type Output = RadianAngle;

            fn add(self, other: $rhs)->Self {
                Self::from(RadianAngle { radians: self.radians + RadianAngle::from(other).radians })
            }
        }
        impl ops::Sub<$rhs> for RadianAngle {
            type Output = RadianAngle;

            fn sub(self, other: $rhs)->Self {
                Self::from(RadianAngle { radians: self.radians - RadianAngle::from(other).radians })
            }
        }
    );
    ($lhs:ty, $rhs:ty) => (
        impl ops::Add<$rhs> for $lhs {
            type Output = $lhs;

            fn add(self, other: $rhs)->Self {
                Self::from(
                    DegreeAngle {
                        degrees: DegreeAngle::from(self).degrees + DegreeAngle::from(other).degrees
                    }
                )
            }
        }
        impl ops::Sub<$rhs> for $lhs {
            type Output = $lhs;

            fn sub(self, other: $rhs)->Self {
                Self::from(
                    DegreeAngle {
                        degrees: DegreeAngle::from(self).degrees - DegreeAngle::from(other).degrees
                    }
                )
            }
        }
    )
}
/// Make all the operators for the angles
macro_rules! make_all_operators_for {
    ($lhs:ty) => (
        make_add_sub_operators_for!($lhs, RadianAngle);
        make_add_sub_operators_for!($lhs, DegreeAngle);
        make_add_sub_operators_for!($lhs, DMSAngle);
        make_add_sub_operators_for!($lhs, HMSAngle);
    );
}

make_all_operators_for!(RadianAngle);
make_all_operators_for!(DegreeAngle);
make_all_operators_for!(DMSAngle);
make_all_operators_for!(HMSAngle);

impl ops::Neg for RadianAngle {
    type Output = RadianAngle;

    fn neg(self) -> Self::Output {
        RadianAngle { radians: -self.radians }
    }
}
impl ops::Neg for DegreeAngle {
    type Output = DegreeAngle;

    fn neg(self) -> Self::Output {
        DegreeAngle { degrees: -self.degrees }
    }
}
impl ops::Neg for DMSAngle {
    type Output = DMSAngle;

    fn neg(self) -> Self::Output {
        DMSAngle { degrees: -self.degrees, ..self }
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
        write!(f,
               "{}\u{00B0} {}\' {}\"",
               self.degrees,
               self.minutes,
               self.seconds)
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
        let degrees = dms.degrees as f64 + dms.minutes as f64 / 60.0 + dms.seconds / 3600.0;
        DegreeAngle { degrees: degrees }
    }
}
impl From<HMSAngle> for DegreeAngle {
    fn from(hms: HMSAngle) -> Self {
        let degrees = (hms.hours as f64 + hms.minutes as f64 / 60.0 + hms.seconds / 3600.0) * 15.0;
        DegreeAngle { degrees: degrees }
    }
}

impl From<RadianAngle> for DMSAngle {
    fn from(radians: RadianAngle) -> Self {
        let decimal_degrees = radians.radians.to_degrees();
        let degrees = decimal_degrees.trunc();
        let mut remainder = decimal_degrees - degrees;
        let minutes = (remainder * 60.0).trunc();
        remainder = remainder - minutes / 60.0;
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
        remainder = remainder - minutes / 60.0;
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
        let mut remainder = decimal_degrees / 15.0 - hours;
        let minutes = (remainder * 60.0).trunc();
        remainder = remainder - minutes / 60.0;
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
        let mut remainder = decimal_degrees / 15.0 - hours;
        let minutes = (remainder * 60.0).trunc();
        remainder = remainder - minutes / 60.0;
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
        let mut decimal_degrees = dms.degrees as f64 + (dms.minutes * 60) as f64 +
                                  3600.0 * dms.seconds;
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
        assert!(approx_eq(RadianAngle::from(DegreeAngle::new(180.0)).radians,
                          PI,
                          1.0e-15));
        assert!(approx_eq(RadianAngle::from(DegreeAngle::new(-180.0)).radians,
                          -PI,
                          1.0e-15));
        assert!(approx_eq(RadianAngle::from(DegreeAngle::new(90.0)).radians,
                          FRAC_PI_2,
                          1.0e-15));
        assert!(approx_eq(RadianAngle::from(DegreeAngle::new(-90.0)).radians,
                          -FRAC_PI_2,
                          1.0e-15));
        assert!(approx_eq(RadianAngle::from(DegreeAngle::new(60.0)).radians,
                          FRAC_PI_3,
                          1.0e-15));
        assert!(approx_eq(RadianAngle::from(DegreeAngle::new(-60.0)).radians,
                          -FRAC_PI_3,
                          1.0e-15));
        assert!(approx_eq(RadianAngle::from(DegreeAngle::new(45.0)).radians,
                          FRAC_PI_4,
                          1.0e-15));
        assert!(approx_eq(RadianAngle::from(DegreeAngle::new(-45.0)).radians,
                          -FRAC_PI_4,
                          1.0e-15));

        assert!(approx_eq(RadianAngle::from(DMSAngle::new(180, 0, 0.0)).radians,
                          PI,
                          1.0e-15));
        assert!(approx_eq(RadianAngle::from(DMSAngle::new(-180, 0, 0.0)).radians,
                          -PI,
                          1.0e-15));
        assert!(approx_eq(RadianAngle::from(DMSAngle::new(90, 0, 0.0)).radians,
                          FRAC_PI_2,
                          1.0e-15));
        assert!(approx_eq(RadianAngle::from(DMSAngle::new(-90, 0, 0.0)).radians,
                          -FRAC_PI_2,
                          1.0e-15));
        assert!(approx_eq(RadianAngle::from(DMSAngle::new(60, 0, 0.0)).radians,
                          FRAC_PI_3,
                          1.0e-15));
        assert!(approx_eq(RadianAngle::from(DMSAngle::new(-60, 0, 0.0)).radians,
                          -FRAC_PI_3,
                          1.0e-15));
        assert!(approx_eq(RadianAngle::from(DMSAngle::new(45, 0, 0.0)).radians,
                          FRAC_PI_4,
                          1.0e-15));
        assert!(approx_eq(RadianAngle::from(DMSAngle::new(-45, 0, 0.0)).radians,
                          -FRAC_PI_4,
                          1.0e-15));

        assert!(approx_eq(RadianAngle::from(HMSAngle::new(12, 0, 0.0)).radians,
                          PI,
                          1.0e-15));
        assert!(approx_eq(RadianAngle::from(HMSAngle::new(6, 0, 0.0)).radians,
                          FRAC_PI_2,
                          1.0e-15));
        assert!(approx_eq(RadianAngle::from(HMSAngle::new(4, 0, 0.0)).radians,
                          FRAC_PI_3,
                          1.0e-15));
        assert!(approx_eq(RadianAngle::from(HMSAngle::new(3, 0, 0.0)).radians,
                          FRAC_PI_4,
                          1.0e-15));
    }

    #[test]
    fn test_from_for_degree_angle() {
        assert!(approx_eq(DegreeAngle::from(RadianAngle::new(PI)).degrees,
                          180.0,
                          1.0e-15));
        assert!(approx_eq(DegreeAngle::from(RadianAngle::new(-PI)).degrees,
                          -180.0,
                          1.0e-15));
        assert!(approx_eq(DegreeAngle::from(RadianAngle::new(FRAC_PI_2)).degrees,
                          90.0,
                          1.0e-15));
        assert!(approx_eq(DegreeAngle::from(RadianAngle::new(-FRAC_PI_2)).degrees,
                          -90.0,
                          1.0e-15));
        assert!(approx_eq(DegreeAngle::from(RadianAngle::new(FRAC_PI_3)).degrees,
                          60.0,
                          1.0e-14));
        assert!(approx_eq(DegreeAngle::from(RadianAngle::new(-FRAC_PI_3)).degrees,
                          -60.0,
                          1.0e-14));
        assert!(approx_eq(DegreeAngle::from(RadianAngle::new(FRAC_PI_4)).degrees,
                          45.0,
                          1.0e-15));
        assert!(approx_eq(DegreeAngle::from(RadianAngle::new(-FRAC_PI_4)).degrees,
                          -45.0,
                          1.0e-15));

        assert!(approx_eq(DegreeAngle::from(DMSAngle::new(180, 0, 0.0)).degrees,
                          180.0,
                          1.0e-15));
        assert!(approx_eq(DegreeAngle::from(DMSAngle::new(-180, 0, 0.0)).degrees,
                          -180.0,
                          1.0e-15));
        assert!(approx_eq(DegreeAngle::from(DMSAngle::new(90, 0, 0.0)).degrees,
                          90.0,
                          1.0e-15));
        assert!(approx_eq(DegreeAngle::from(DMSAngle::new(-90, 0, 0.0)).degrees,
                          -90.0,
                          1.0e-15));
        assert!(approx_eq(DegreeAngle::from(DMSAngle::new(60, 0, 0.0)).degrees,
                          60.0,
                          1.0e-14));
        assert!(approx_eq(DegreeAngle::from(DMSAngle::new(-60, 0, 0.0)).degrees,
                          -60.0,
                          1.0e-14));
        assert!(approx_eq(DegreeAngle::from(DMSAngle::new(45, 0, 0.0)).degrees,
                          45.0,
                          1.0e-15));
        assert!(approx_eq(DegreeAngle::from(DMSAngle::new(-45, 0, 0.0)).degrees,
                          -45.0,
                          1.0e-15));

        assert!(approx_eq(DegreeAngle::from(HMSAngle::new(12, 0, 0.0)).degrees,
                          180.0,
                          1.0e-15));
        assert!(approx_eq(DegreeAngle::from(HMSAngle::new(6, 0, 0.0)).degrees,
                          90.0,
                          1.0e-15));
        assert!(approx_eq(DegreeAngle::from(HMSAngle::new(4, 0, 0.0)).degrees,
                          60.0,
                          1.0e-14));
        assert!(approx_eq(DegreeAngle::from(HMSAngle::new(3, 0, 0.0)).degrees,
                          45.0,
                          1.0e-15));
    }

    #[test]
    fn test_from_for_dms_angle() {
        //
        // from radians
        //
        let test_val = DMSAngle::from(RadianAngle::new(PI));
        assert_eq!(test_val.degrees, 180);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-15));

        let test_val = DMSAngle::from(RadianAngle::new(-PI));
        assert_eq!(test_val.degrees, -180);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-15));

        let test_val = DMSAngle::from(RadianAngle::new(FRAC_PI_2));
        assert_eq!(test_val.degrees, 90);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-15));

        let test_val = DMSAngle::from(RadianAngle::new(-FRAC_PI_2));
        assert_eq!(test_val.degrees, -90);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-15));

        let test_val = DMSAngle::from(RadianAngle::new(FRAC_PI_3));
        assert_eq!(test_val.degrees, 60);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-10));

        let test_val = DMSAngle::from(RadianAngle::new(-FRAC_PI_3));
        assert_eq!(test_val.degrees, -60);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-10));

        let test_val = DMSAngle::from(RadianAngle::new(FRAC_PI_4));
        assert_eq!(test_val.degrees, 45);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-10));

        let test_val = DMSAngle::from(RadianAngle::new(-FRAC_PI_4));
        assert_eq!(test_val.degrees, -45);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-10));

        //
        // From degrees
        //
        let test_val = DMSAngle::from(DegreeAngle::new(180.0));
        assert_eq!(test_val.degrees, 180);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-15));

        let test_val = DMSAngle::from(DegreeAngle::new(-180.0));
        assert_eq!(test_val.degrees, -180);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-15));

        let test_val = DMSAngle::from(DegreeAngle::new(90.0));
        assert_eq!(test_val.degrees, 90);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-15));

        let test_val = DMSAngle::from(DegreeAngle::new(-90.0));
        assert_eq!(test_val.degrees, -90);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-15));

        let test_val = DMSAngle::from(DegreeAngle::new(60.0));
        assert_eq!(test_val.degrees, 60);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-10));

        let test_val = DMSAngle::from(DegreeAngle::new(-60.0));
        assert_eq!(test_val.degrees, -60);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-10));

        let test_val = DMSAngle::from(DegreeAngle::new(45.0));
        assert_eq!(test_val.degrees, 45);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-10));

        let test_val = DMSAngle::from(DegreeAngle::new(-45.0));
        assert_eq!(test_val.degrees, -45);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-10));

        let test_val = DMSAngle::from(DegreeAngle::new(-45.55));
        assert_eq!(test_val.degrees, -45);
        assert_eq!(test_val.minutes, 32);
        assert!(approx_eq(test_val.seconds, 60.0, 1.0e-10));

        //
        // From HMS
        //
        let test_val = DMSAngle::from(HMSAngle::new(12, 0, 0.0));
        assert_eq!(test_val.degrees, 180);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-15));

        let test_val = DMSAngle::from(HMSAngle::new(6, 0, 0.0));
        assert_eq!(test_val.degrees, 90);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-15));

        let test_val = DMSAngle::from(HMSAngle::new(4, 0, 0.0));
        assert_eq!(test_val.degrees, 60);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-10));

        let test_val = DMSAngle::from(HMSAngle::new(3, 0, 0.0));
        assert_eq!(test_val.degrees, 45);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-10));
    }

    #[test]
    fn test_from_for_hms_angle() {
        //
        // from radians
        //
        let test_val = HMSAngle::from(RadianAngle::new(PI));
        assert_eq!(test_val.hours, 12);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-15));

        let test_val = HMSAngle::from(RadianAngle::new(-PI));
        assert_eq!(test_val.hours, 12);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-15));

        let test_val = HMSAngle::from(RadianAngle::new(FRAC_PI_2));
        assert_eq!(test_val.hours, 6);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-15));

        let test_val = HMSAngle::from(RadianAngle::new(-FRAC_PI_2));
        assert_eq!(test_val.hours, 18);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-15));

        let test_val = HMSAngle::from(RadianAngle::new(FRAC_PI_3));
        assert_eq!(test_val.hours, 4);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-10));

        let test_val = HMSAngle::from(RadianAngle::new(-FRAC_PI_3));
        assert_eq!(test_val.hours, 20);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-10));

        let test_val = HMSAngle::from(RadianAngle::new(FRAC_PI_4));
        assert_eq!(test_val.hours, 3);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-10));

        let test_val = HMSAngle::from(RadianAngle::new(-FRAC_PI_4));
        assert_eq!(test_val.hours, 21);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-10));

        //
        // From degrees
        //
        let test_val = HMSAngle::from(DegreeAngle::new(180.0));
        assert_eq!(test_val.hours, 12);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-15));

        let test_val = HMSAngle::from(DegreeAngle::new(-180.0));
        assert_eq!(test_val.hours, 12);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-15));

        let test_val = HMSAngle::from(DegreeAngle::new(90.0));
        assert_eq!(test_val.hours, 6);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-15));

        let test_val = HMSAngle::from(DegreeAngle::new(-90.0));
        assert_eq!(test_val.hours, 18);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-15));

        let test_val = HMSAngle::from(DegreeAngle::new(60.0));
        assert_eq!(test_val.hours, 4);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-10));

        let test_val = HMSAngle::from(DegreeAngle::new(-60.0));
        assert_eq!(test_val.hours, 20);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-10));

        let test_val = HMSAngle::from(DegreeAngle::new(45.0));
        assert_eq!(test_val.hours, 3);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-10));

        let test_val = HMSAngle::from(DegreeAngle::new(-45.0));
        assert_eq!(test_val.hours, 21);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-10));

        let test_val = HMSAngle::from(DegreeAngle::new(-45.55));
        assert_eq!(test_val.hours, 20);
        assert_eq!(test_val.minutes, 57);
        assert!(approx_eq(test_val.seconds, 48.0, 1.0e-10));

        //
        // From DMS
        //
        let test_val = HMSAngle::from(DMSAngle::new(180, 0, 0.0));
        assert_eq!(test_val.hours, 12);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-15));

        let test_val = HMSAngle::from(DMSAngle::new(90, 0, 0.0));
        assert_eq!(test_val.hours, 6);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-15));

        let test_val = HMSAngle::from(DMSAngle::new(60, 0, 0.0));
        assert_eq!(test_val.hours, 4);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-10));

        let test_val = HMSAngle::from(DMSAngle::new(45, 0, 0.0));
        assert_eq!(test_val.hours, 3);
        assert_eq!(test_val.minutes, 0);
        assert!(approx_eq(test_val.seconds, 0.0, 1.0e-10));
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
    fn test_map_to_branch() {
        assert!(approx_eq(map_to_branch(-200.0, -180.0, 180.0), 160.0, 1.0e-12));
        assert!(approx_eq(map_to_branch(-200.0, 0.0, 360.0), 160.0, 1.0e-12));
        assert!(approx_eq(map_to_branch(200.0, -180.0, 180.0), -160.0, 1.0e-12));
        assert!(approx_eq(map_to_branch(200.0, 0.0, 360.0), 200.0, 1.0e-12));

        assert!(approx_eq(map_to_branch(-500.0, -180.0, 180.0), -140.0, 1.0e-12));
        assert!(approx_eq(map_to_branch(-500.0, 0.0, 360.0), 220.0, 1.0e-12));
        assert!(approx_eq(map_to_branch(500.0, -180.0, 180.0), 140.0, 1.0e-12));
        assert!(approx_eq(map_to_branch(500.0, 0.0, 360.0), 140.0, 1.0e-12));

        assert!(approx_eq(map_to_branch(-45.55, 0.0, 360.0), 314.45, 1.0e-12));
    }
}
