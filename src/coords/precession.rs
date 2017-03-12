//!
//! Adjustments to right acension and declination due to precession.
//!
//! Authors: Ryan Leach
//!
//! Copyright: Ryan Leach, 2017
//!
//! License: [BSD 3-clause](https://opensource.org/licenses/BSD-3-Clause)
//!
//! All of the coordinates carry a valid time with them. This is the epoch. The epoch may be the
//! standard epochs of 1950 or 2000, or it could be any other date.

use super::super::angles::{RadianAngle, DegreeAngle, DMSAngle};
use super::super::astro_time::{Builder, AstroTime};
use super::{EquatorialCoords, AstroCoordinate};

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
/// to_epoch: The date/time of the epoch and equinox to convert to.
pub fn precess_coords(coords: EquatorialCoords, to_epoch: AstroTime) -> EquatorialCoords {
    // Algorithm from page 134 of Meeus
    let jd0 = coords.epoch().julian_day_number();

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

    EquatorialCoords::new(ra, dec, to_epoch)
}

// Apply the affects of proper motion to convert coordinates from one epoch to another.
//
// Note that this should be done __BEFORE__ applying precession. There is no check to make sure
// that the epoch and equinox of the proper motion matches that of the coordinates.
pub fn apply_proper_motion<T, U>(coords: EquatorialCoords,
                                 to_epoch: AstroTime,
                                 ra_motion: T,
                                 dec_motion: U)
                                 -> EquatorialCoords
    where RadianAngle: From<T> + From<U>
{

    let dt = (to_epoch.julian_day_number() - coords.epoch().julian_day_number()) / 365.25;

    let new_ra = coords.right_acension().radians() + RadianAngle::from(ra_motion).radians() * dt;
    let new_dec = coords.declination().radians() + RadianAngle::from(dec_motion).radians() * dt;

    let new_ra = RadianAngle::new(new_ra);
    let new_dec = RadianAngle::new(new_dec);

    EquatorialCoords::new(new_ra, new_dec, coords.epoch())
}

#[cfg(test)]
mod precession_tests {
    use super::*;
    use super::super::super::test_util::*;
    use super::super::super::angles::*;
    use super::super::super::astro_time::*;

    #[test]
    fn test_precess_coords() {
        // From example on pg 135 Meeus
        let coords = EquatorialCoords::new(RadianAngle::from(HMSAngle::new(2, 44, 11.986)),
                                           RadianAngle::from(DMSAngle::new(49, 13, 42.48)),
                                           *J2000);

        let to_epoch = Builder::from_julian_date(2_462_088.69).build().unwrap();
        let ra_pmotion = HMSAngle::new(0, 0, 0.03425);
        let dec_pmotion = DMSAngle::new(0, 0, -0.0895);

        let mut new_coords = apply_proper_motion(coords, to_epoch, ra_pmotion, dec_pmotion);
        new_coords = precess_coords(new_coords, to_epoch);

        println!("\nNew Coords:\n  {}", new_coords);
        println!("RA = {}", DegreeAngle::from(new_coords.right_acension()));
        println!("dec = {}", DegreeAngle::from(new_coords.declination()));
        println!("delta_RA = {}", ra_pmotion);
        println!("delta dec = {}", dec_pmotion);

        assert!(approx_eq(DegreeAngle::from(new_coords.right_acension()).degrees(),
                          41.547_214,
                          1.0e-6));
        assert!(approx_eq(DegreeAngle::from(new_coords.declination()).degrees(),
                          49.348_483,
                          1.0e-6));

        let mut old_coords = precess_coords(new_coords, J2000.clone());
        old_coords = apply_proper_motion(old_coords, to_epoch, -ra_pmotion, -dec_pmotion);

        println!("\nOld Coords:\n  {}", old_coords);
        println!("RA = {}", DegreeAngle::from(old_coords.right_acension()));
        println!("dec = {}", DegreeAngle::from(old_coords.declination()));

        assert!(approx_eq(new_coords.right_acension().radians(),
                          RadianAngle::from(DegreeAngle::new(41.547_214)).radians(),
                          1.0e-6));
        assert!(approx_eq(new_coords.declination().radians(),
                          RadianAngle::from(DegreeAngle::new(49.348_483)).radians(),
                          1.0e-6));
    }
}