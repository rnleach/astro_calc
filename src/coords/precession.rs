//!
//! Adjustments to right ascension and declination due to precession.
//!
//! Authors: Ryan Leach
//!
//! Copyright: Ryan Leach, 2017
//!
//! License: [BSD 3-clause](https://opensource.org/licenses/BSD-3-Clause)
//!

use super::{EquatorialCoords, HasEpoch, HasValidTime};
use super::super::angles::{RadianAngle, DegreeAngle, DMSAngle};
use super::super::astro_time::{Builder, AstroTime};
use super::super::error::AstroResult;

// This is a heavy handed solution for such small constants, but I cannot do compile time function
// evaluation yet (maybe later). If I calculated the value directly in radians, I would still have
// to define the constant in the angles sub-module, but it belongs here.
lazy_static! {
    /// Mean Obliquity of the ecliptic for the standard 2000 epoch.
    pub static ref EPSILON_2000: RadianAngle =
        RadianAngle::from(DegreeAngle::new( 23.439_291_1 ));

    /// Mean Obliquity of the ecliptic for the standard 1950 epoch.
    pub static ref EPSILON_1950: RadianAngle =
        RadianAngle::from(DegreeAngle::new( 23.445_788_9 ));

    /// Time of the standard epoch J2050
    pub static ref J2050: AstroTime = Builder::from_julian_date(2_649_807.00)
                                               .dynamical_time().build().unwrap();

    /// Time of the standard epoch J2050
    pub static ref J2000: AstroTime = Builder::from_julian_date(2_451_545.00)
                                               .dynamical_time().build().unwrap();

    /// Time of the standard epoch B1950
    pub static ref B1950: AstroTime = Builder::from_julian_date(2_433_282.423_5)
                                               .dynamical_time().build().unwrap();

    /// Time of the standard epoch B1900
    pub static ref B1900: AstroTime = Builder::from_julian_date(2_415_020.313_5)
                                               .dynamical_time().build().unwrap();
}

/// Apply the effects of precession to convert coordinates from one equinox to another.
///
/// coords: The coordinates to convert.
/// to_epoch: The date/time of the epoch to convert to.
pub fn precess_coords(coords: EquatorialCoords,
                      to_epoch: AstroTime)
                      -> AstroResult<EquatorialCoords> {
    // Algorithm from page 134 of Meeus
    let jd0 = try!(coords.epoch().as_dt());
    let jd0 = jd0.julian_day_number();
    let to_epoch = try!(to_epoch.as_dt());

    #[allow(non_snake_case)]
    let T = (jd0 - 2_451_545.0) / 36_525.0;
    let t = (to_epoch.julian_day_number() - jd0) / 36_525.0;

    let first_term = (2_306.2181 + (1.396_56 - 0.000_139 * T) * T) * t; // arcseconds
    let zeta = first_term + ((0.301_88 - 0.000_344 * T) + 0.017_998 * t) * t * t;
    let z = first_term + ((1.094_68 + 0.000_066 * T) + 0.018_203 * t) * t * t;
    let theta = ((2_004.310_9 - (0.853_30 - 0.000_217 * T) * T) -
                 ((0.426_65 + 0.000_217 * T) - 0.041_833 * t) * t) * t;

    let zeta = RadianAngle::from(DMSAngle::new(0, 0, zeta));
    let z = RadianAngle::from(DMSAngle::new(0, 0, z));
    let theta = RadianAngle::from(DMSAngle::new(0, 0, theta));
    let dec0 = coords.declination();
    let ra0 = coords.right_acension();

    #[allow(non_snake_case)]
    let A = dec0.cos() * (ra0 + zeta).sin();
    #[allow(non_snake_case)]
    let B = theta.cos() * dec0.cos() * (ra0 + zeta).cos() - theta.sin() * dec0.sin();
    #[allow(non_snake_case)]
    let C = theta.sin() * dec0.cos() * (ra0 + zeta).cos() + theta.cos() * dec0.sin();

    let ra = RadianAngle::atan2(A, B) + z;
    let dec = RadianAngle::new(C.asin());

    Ok(EquatorialCoords::new(ra, dec, to_epoch, coords.valid_time()))
}

#[cfg(test)]
mod precession_tests {
    use super::*;
    use super::super::proper_motion::{ProperMotionEq, apply_proper_motion_eq};
    use super::super::super::test_util::*;
    use super::super::super::angles::*;
    use super::super::super::astro_time::*;

    #[test]
    fn test_precess_coords() {
        // From example on pg 135 Meeus
        let coords = EquatorialCoords::new(RadianAngle::from(HMSAngle::new(2, 44, 11.986)),
                                           RadianAngle::from(DMSAngle::new(49, 13, 42.48)),
                                           *J2000,
                                           *J2000);

        //let to_epoch = Builder::from_julian_date(2_462_088.69).build().unwrap();
        let to_epoch = Builder::from_gregorian_utc(2028, 11, 13, 4, 33, 36).build().unwrap();
        let motion = ProperMotionEq::new(HMSAngle::new(0, 0, 0.03425),
                                         DMSAngle::new(0, 0, -0.0895),
                                         *J2000);

        let mut new_coords = apply_proper_motion_eq(coords, to_epoch, motion).unwrap();
        new_coords = precess_coords(new_coords, to_epoch).unwrap();

        println!("\nNew Coords:\n  {}", new_coords);
        println!("RA = {}", DegreeAngle::from(new_coords.right_acension()));
        println!("dec = {}", DegreeAngle::from(new_coords.declination()));
        println!("Proper Motion = {}", motion);

        assert!(approx_eq(DegreeAngle::from(new_coords.right_acension()).degrees(),
                          41.547_214,
                          1.0e-6));
        assert!(approx_eq(DegreeAngle::from(new_coords.declination()).degrees(),
                          49.348_483,
                          1.0e-6));

        let mut old_coords = precess_coords(new_coords, *J2000).unwrap();
        old_coords = apply_proper_motion_eq(old_coords, *J2000, motion).unwrap();

        println!("\nOld Coords:\n  {}", old_coords);
        println!("RA = {}", DegreeAngle::from(old_coords.right_acension()));
        println!("dec = {}", DegreeAngle::from(old_coords.declination()));

        assert!(approx_eq(old_coords.right_acension().radians(),
                          coords.right_acension().radians(),
                          1.0e-10));
        assert!(approx_eq(old_coords.declination().radians(),
                          coords.declination().radians(),
                          1.0e-10));
        assert_eq!(old_coords.valid_time(), coords.valid_time());
        assert_eq!(old_coords.epoch(), coords.epoch());
    }
}
