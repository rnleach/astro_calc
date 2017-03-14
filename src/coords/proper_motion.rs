//!
//! Adjustments to right acension and declination due to proper motion.
//!
//! Authors: Ryan Leach
//!
//! Copyright: Ryan Leach, 2017
//!
//! License: [BSD 3-clause](https://opensource.org/licenses/BSD-3-Clause)
//!

use std::fmt;

use super::*;
use super::super::angles::{RadianAngle, DMSAngle, HMSAngle};
use super::super::astro_time::AstroTime;
use super::super::error::AstroResult;

pub trait ProperMotion: fmt::Display {}

/// Account for proper motion as provided by an ephemeris in equatorial coordinates.
#[derive(Debug, Clone, Copy)]
pub struct ProperMotionEq {
    /// Annual change in right acension.
    right_acension: RadianAngle,
    /// Annual change in declination.
    declination: RadianAngle,
    /// Epoch this proper motion is valid for.
    epoch: AstroTime,
}

impl ProperMotion for ProperMotionEq {}

impl ProperMotionEq {
    /// Create a new proper motion in equatorial coordinates.
    pub fn new<T, U>(right_acension: T, declination: U, epoch: AstroTime) -> ProperMotionEq
        where RadianAngle: From<T> + From<U>
    {
        ProperMotionEq {
            right_acension: RadianAngle::from(right_acension),
            declination: RadianAngle::from(declination),
            epoch: epoch,
        }
    }
}

impl fmt::Display for ProperMotionEq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ProperMotionEq { right_acension: ra, declination: dec, epoch: e } = *self;

        write!(f,
               "Proper motion in equatorial coordinates/per year\n  right acension: {}\n  \
                declination: {}\n  for epoch: {}\n",
               HMSAngle::from(ra),
               DMSAngle::from(dec),
               e)
    }
}

/// Proper motion in ecliptical coordinates. This is rare.
#[derive(Debug, Clone, Copy)]
pub struct ProperMotionEc {
    /// Annual change in latitude
    latitude: RadianAngle,
    /// Anual change in longitude
    longitude: RadianAngle,
    /// Epoch this proper motion is valid for.
    epoch: AstroTime,
}

impl ProperMotion for ProperMotionEc {}

impl ProperMotionEc {
    /// Create a new proper motion in ecliptic coordinates.
    pub fn new<T, U>(latitude: T, longitude: U, epoch: AstroTime) -> ProperMotionEc
        where RadianAngle: From<T> + From<U>
    {
        ProperMotionEc {
            latitude: RadianAngle::from(latitude),
            longitude: RadianAngle::from(longitude),
            epoch: epoch,
        }
    }
}

impl fmt::Display for ProperMotionEc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let ProperMotionEc { latitude: lat, longitude: lon, epoch: e } = *self;

        write!(f,
               "Proper motion in ecliptical coordinates/per year\n  latitude: {}\n  longitude: \
                {}\n  for epoch: {}\n",
               DMSAngle::from(lat),
               HMSAngle::from(lon),
               e)
    }
}

// Apply the affects of proper motion to convert coordinates from one valid time to another, in
// equatorial coordinates.
//
// Note that this should be done __BEFORE__ applying precession. There is no check to make sure
// that the epoch of the proper motion matches that of the coordinates.
pub fn apply_proper_motion_eq(coords: EquatorialCoords,
                              to_valid_time: AstroTime,
                              motion: ProperMotionEq)
                              -> AstroResult<EquatorialCoords> {
    let to_valid_time = try!(to_valid_time.as_dt());
    let coords_valid_time = try!(coords.valid_time().as_dt());
    let dt = (to_valid_time.julian_day_number() - coords_valid_time.julian_day_number()) / 365.25;

    let new_ra = coords.right_acension().radians() + motion.right_acension.radians() * dt;
    let new_dec = coords.declination().radians() + motion.declination.radians() * dt;

    let new_ra = RadianAngle::new(new_ra);
    let new_dec = RadianAngle::new(new_dec);

    Ok(EquatorialCoords::new(new_ra, new_dec, coords.epoch(), to_valid_time))
}

// Apply the affects of proper motion to convert coordinates from one valid time to another, in
// ecliptic coordinates.
//
// Note that this should be done __BEFORE__ applying precession. There is no check to make sure
// that the epoch of the proper motion matches that of the coordinates.
pub fn apply_proper_motion_ec(coords: EclipticCoords,
                              to_valid_time: AstroTime,
                              motion: ProperMotionEc)
                              -> AstroResult<EclipticCoords> {

    let to_valid_time = try!(to_valid_time.as_dt());
    let coords_valid_time = try!(coords.valid_time().as_dt());
    let dt = (to_valid_time.julian_day_number() - coords_valid_time.julian_day_number()) / 365.25;


    let new_lat = coords.latitude().radians() + motion.latitude.radians() * dt;
    let new_lon = coords.longitude().radians() + motion.longitude.radians() * dt;

    let new_lat = RadianAngle::new(new_lat);
    let new_lon = RadianAngle::new(new_lon);

    Ok(EclipticCoords::new(new_lat, new_lon, coords.epoch(), to_valid_time))
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::test_util::*;
    use super::super::super::astro_time::*;

    #[test]
    fn test_apply_proper_motion_eq() {

        // Extracted from exampe on pg 135 of Meeus.
        let coords = EquatorialCoords::new(HMSAngle::new(2, 44, 11.986),
                                           DMSAngle::new(49, 13, 42.48),
                                           *J2000,
                                           *J2000);

        //let to_epoch = Builder::from_julian_date(2_462_088.69).build().unwrap();
        let to_valid_time = Builder::from_gregorian_utc(2028, 11, 13, 4, 33, 36).build().unwrap();
        let motion = ProperMotionEq::new(HMSAngle::new(0, 0, 0.03425),
                                         DMSAngle::new(0, 0, -0.0895),
                                         *J2000);

        let new_coords = apply_proper_motion_eq(coords, to_valid_time, motion).unwrap();

        println!("\nCoords: {}\nMotion: {}\nNew Coords: {}",
                 coords,
                 motion,
                 new_coords);

        assert!(approx_eq(new_coords.right_acension().radians(),
                          RadianAngle::from(HMSAngle::new(2, 44, 12.975)).radians(),
                          1.0e-7));
        assert!(approx_eq(new_coords.declination().radians(),
                          RadianAngle::from(DMSAngle::new(49, 13, 39.9)).radians(),
                          1.0e-7));

        assert!(new_coords.valid_time() != coords.valid_time());
        assert!(new_coords.epoch() == coords.epoch());

        println!("---------------------------------------------------");

        let old_coords = apply_proper_motion_eq(new_coords, *J2000, motion).unwrap();

        println!("\nNew Coords: {}\n Motion: {}\nOld Coords: {}\nCoords: {}",
                 new_coords,
                 motion,
                 old_coords,
                 coords);

        assert!(old_coords.right_acension().radians() == coords.right_acension().radians());
        assert!(old_coords.declination().radians() == coords.declination().radians());
        assert!(old_coords.valid_time() == coords.valid_time());
        // Nothing in this test changed the epoch.
        assert!(old_coords.epoch() == coords.epoch());
    }

}
