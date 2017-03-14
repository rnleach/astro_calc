//!
//! Module for dealing with spatial coordinates in astronomical calculations.
//!
//! Authors: Ryan Leach
//!
//! Copyright: Ryan Leach, 2017
//!
//! License: [BSD 3-clause](https://opensource.org/licenses/BSD-3-Clause)
//!
mod ecliptic;
mod equatorial;
mod galactic;
mod geo;
mod horizontal;
mod precession;
mod proper_motion;

use std::fmt;
use super::angles::{RadianAngle, DegreeAngle, DMSAngle, HMSAngle};
use super::astro_time::AstroTime;

pub use self::ecliptic::EclipticCoords;
pub use self::equatorial::EquatorialCoords;
pub use self::galactic::GalacticCoords;
pub use self::geo::GeoCoords;
pub use self::horizontal::HorizontalCoords;
pub use self::precession::{EPSILON_2000, EPSILON_1950, J2050, J2000, B1950, B1900};
pub use self::proper_motion::{ProperMotionEc, ProperMotionEq};

// TODO (**In Progress**) implement with low level, primitive type only, private functions closely
// tied to algorithms in the book.
//
//  SUB TODO - implement chpt 22 (nutation) so I can use apparent coords and times
//  SUB TODO - implement conversion from equatorial to ecliptic for proper motion pg 138.
//
// TODO unit test everything
// TODO add trait constraint From for ecliptic and equatorial coords. HorizontalCoords cannot be
//      derived from the others without valid time and earth location. But, all the other types
//      should be derivable from HorizontalCoords. Galactic coords should be derivable from all
//      others, but all others need an epoch to be transformed into galactic coords.
// TODO add enum to tag coordinates as mean or apparent, because it can make a difference when
//      you need to calculate sidereal time.

/// Coordinate systems used in positional astronomy.
pub trait AstroCoordinate: fmt::Display + HasEpoch + HasValidTime {
    // TODO add From<Horizontal>, From<GalacticCoords>, From<EquatorialCoords>, From<EclipticCoords>
}

/// Coordinate systems with an epoch
pub trait HasEpoch {
    /// Get the epoch of the equinox.
    fn epoch(&self) -> AstroTime;
}

/// Coordinate systems with a valid time
pub trait HasValidTime {
    /// Get the valid time
    fn valid_time(&self) -> AstroTime;
}

// Calculate the local sidereal time
fn local_mean_sidereal_time(gmt: AstroTime, geo_location: GeoCoords) -> RadianAngle {
    let gst = gmt.mean_sidereal_greenwich();
    let long = geo_location.meeus_long();
    gst - long
}

// TODO local_apparent_sidereal_time

// Calculate the local hour angle
fn local_mean_hour_angle(gmt: AstroTime,
                         geo_location: GeoCoords,
                         equatorial_location: EquatorialCoords)
                         -> RadianAngle {
    let lst = local_mean_sidereal_time(gmt, geo_location);
    let alpha = equatorial_location.right_acension();
    lst - alpha
}

// TODO local_apparent_hour_angle

// Calculate a right-ascension given an hour angle, time, and geographic location.
fn right_acension_from_mean_hour_angle(ha: RadianAngle,
                                       geo_location: GeoCoords,
                                       gmt: AstroTime)
                                       -> RadianAngle {
    let lst = local_mean_sidereal_time(gmt, geo_location);
    lst - ha
}

// TODO right_acension_from_apparent_hour_angle

// TODO mean_obliquity_of_ecliptic

// TODO apparent_obliquity_of_ecliptic

/******************************

// Transform from equatorial to ecliptical coordinates.
fn trans_equatorial_to_ecliptical(eq: EquatorialCoords,
                                  obliquity_of_ecliptic: RadianAngle)
                                  -> EclipticCoords {
    let lon = RadianAngle::atan2(eq.right_acension().sin() * obliquity_of_ecliptic.cos() +
                                 eq.declination().tan() * obliquity_of_ecliptic.sin(),
                                 eq.right_acension().cos());
    let lat = RadianAngle::asin(eq.declination().sin() * obliquity_of_ecliptic.cos() -
                                eq.declination().cos() * obliquity_of_ecliptic.sin() *
                                eq.right_acension().sin());
    EclipticCoords::new(lat, lon, eq.epoch())
}

// Transform from ecliptical to equatorial coordinates.
fn trans_ecliptical_to_equatorial(ec: EclipticCoords,
                                  obliquity_of_ecliptic: RadianAngle)
                                  -> EquatorialCoords {
    let ra = RadianAngle::atan2(ec.longitude().sin() * obliquity_of_ecliptic.cos() -
                                ec.latitude().tan() * obliquity_of_ecliptic.sin(),
                                ec.longitude().cos());
    let dec = RadianAngle::asin(ec.latitude().sin() * obliquity_of_ecliptic.cos() +
                                ec.latitude().cos() * obliquity_of_ecliptic.sin() *
                                ec.longitude().sin());
    EquatorialCoords::new(ra, dec, ec.epoch())
}

// Transform from equatorial to horizontal coordinates. This assumes azimuth reckoned from the
// south and increasing to the west.
fn trans_equatorial_to_horizontal(eq: EquatorialCoords,
                                  geo: GeoCoords,
                                  gmt: AstroTime,
                                  use_apparent: bool)
                                  -> HorizontalCoords {
    // TODO transform equatorial coordinates to gmt epoch!
    let eqa = eq; // eqa = equatorial coords adjusted to current time epoch
    let h = if use_apparent {
        // TODO local_apparent_hour_angle(gmt, geo, eqa)
        local_mean_hour_angle(gmt, geo, eqa)
    } else {
        local_mean_hour_angle(gmt, geo, eqa)
    };
    let phi = geo.radian_lat();
    let delta = eqa.declination();
    let az = RadianAngle::atan2(h.sin(), h.cos() * phi.sin() - delta.tan() * phi.cos());
    let alt = RadianAngle::asin(phi.sin() * delta.sin() + phi.cos() * delta.cos() * h.cos());

    HorizontalCoords::new(alt, az, geo, gmt)
}

// Transform from horizontal to equatorial coordinates.
fn trans_horizontal_to_equatorial(hzc: HorizontalCoords, get_apparent: bool) -> EquatorialCoords {
    let az = hzc.azimuth();
    let phi = hzc.observer_location().radian_lat();
    let alt = hzc.altitude();
    let h = RadianAngle::atan2(az.sin(), az.cos() * phi.sin() + alt.tan() * phi.cos());

    let ra = if get_apparent {
        // TODO right_acension_from_apparent_hour_angle(h, hzc.observer_loc, hzc.valid_time)
        //.map_to_time_range()
        right_acension_from_mean_hour_angle(h, hzc.observer_location(), hzc.valid_time())
            .map_to_time_range()
    } else {
        right_acension_from_mean_hour_angle(h, hzc.observer_location(), hzc.valid_time())
            .map_to_time_range()
    };
    let dec = RadianAngle::asin(phi.sin() * alt.sin() - phi.cos() * alt.cos() * az.cos());

    EquatorialCoords::new(ra, dec, hzc.valid_time())
}

#[cfg(test)]
mod private_test {
    use super::*;
    use super::super::test_util::*;
    use super::super::astro_time::Builder;

    #[test]
    fn test_local_mean_hour_angle() {
        // This example is from page 95 of Meeus. I had to make a correction since I am not
        // adjusting for the apparent sidereal time in my calculations. That will come later.
        // The adjust term is the subtraction of 0.0009858333333 degrees from my answer.
        // Even still, the book example is only accurate to 1 decimal point in seconds, which
        // translates about 3.5 decimal places in degrees.
        let gmt = Builder::from_gregorian_utc(1987, 4, 10, 19, 21, 0).build().unwrap();
        let geo_loc = GeoCoords::new(DMSAngle::new(38, 55, 17.0), DMSAngle::new(-77, 3, 56.0));
        let astro_loc = EquatorialCoords::new(HMSAngle::new(23, 9, 16.641),
                                              DMSAngle::new(-6, 43, 11.61),
                                              gmt);
        println!();
        println!("Error = {}",
                 HMSAngle::from(local_mean_hour_angle(gmt, geo_loc, astro_loc).map_to_time_range() -
                                DegreeAngle::new(0.0009858333333) -
                                DegreeAngle::new(64.352133)));
        println!("Error = {}",
                 DMSAngle::from(local_mean_hour_angle(gmt, geo_loc, astro_loc).map_to_time_range() -
                                DegreeAngle::new(0.0009858333333) -
                                DegreeAngle::new(64.352133)));
        println!("Error = {}",
                 DegreeAngle::from(local_mean_hour_angle(gmt, geo_loc, astro_loc)
                     .map_to_time_range() -
                                   DegreeAngle::new(0.0009858333333) -
                                   DegreeAngle::new(64.352133)));
        println!("Error = {}",
                 RadianAngle::from(local_mean_hour_angle(gmt, geo_loc, astro_loc)
                     .map_to_time_range() -
                                   DegreeAngle::new(0.0009858333333) -
                                   DegreeAngle::new(64.352133)));
        println!();
        assert!(approx_eq(DegreeAngle::from(local_mean_hour_angle(gmt, geo_loc, astro_loc)
                                  .map_to_time_range())
                              .degrees() - 0.0009858333333,
                          64.352133,
                          1.4e-4));
    }

    #[test]
    fn test_trans_equatorial_to_ecliptical_and_back() {
        let eq_coords = EquatorialCoords::new(HMSAngle::new(7, 45, 18.946),
                                              DMSAngle::new(28, 1, 34.26),
                                              *J2000);
        let obliquity = RadianAngle::from(DegreeAngle::new(23.4392911));

        let ec_coords = trans_equatorial_to_ecliptical(eq_coords, obliquity);

        println!("\nPosition in EclipticCoords:\n{}\n", ec_coords);

        assert!(approx_eq(DegreeAngle::from(ec_coords.latitude()).degrees(),
                          6.684170,
                          1.0e-6));
        assert!(approx_eq(DegreeAngle::from(ec_coords.longitude()).degrees(),
                          113.215630,
                          1.0e-6));

        let eq_back = trans_ecliptical_to_equatorial(ec_coords, obliquity);
        println!("Position in EquatorialCoords: \n{}", eq_back);

        assert!(approx_eq(eq_back.right_acension().radians(),
                          eq_coords.right_acension().radians(),
                          1.0e-15));
        assert!(approx_eq(eq_back.declination().radians(),
                          eq_coords.declination().radians(),
                          1.0e-15));
    }

    #[test]
    fn test_trans_equatorial_to_horizontal_and_back() {
        // example from page 95 of Meuus
        let vtime = Builder::from_gregorian_utc(1987, 4, 10, 19, 21, 0).build().unwrap();

        // TODO adjusted RA manually to get apparent local hour angle. I need a function
        // to make adjustments in chpt 22 for apparent sidereal time since these are apparent coords
        let eq_coords = EquatorialCoords::new(HMSAngle::new(23, 9, 16.8746),
                                              DMSAngle::new(-6, 43, 11.61),
                                              vtime);

        println!("Position in original EquatorialCoords: \n{}", eq_coords);

        let geo_coords = GeoCoords::new(DMSAngle::new(38, 55, 17.0), DMSAngle::new(-77, 3, 56.0));

        let h_coords = trans_equatorial_to_horizontal(eq_coords, geo_coords, vtime, false);

        println!("\nPosition in horizontal coordinates:\n{}\n", h_coords);

        assert!(approx_eq(DegreeAngle::from(h_coords.altitude()).degrees(),
                          15.1249,
                          1.0e-3));
        assert!(approx_eq(DegreeAngle::from(h_coords.azimuth()).degrees(),
                          68.0337,
                          1.0e-3));

        let h_back = trans_horizontal_to_equatorial(h_coords, false);
        println!("Position in back EquatorialCoords: \n{}", h_back);

        assert!(approx_eq(h_back.right_acension().radians(),
                          eq_coords.right_acension().radians(),
                          1.0e-15));
        assert!(approx_eq(h_back.declination().radians(),
                          eq_coords.declination().radians(),
                          1.0e-15));
    }
}

**********************/
