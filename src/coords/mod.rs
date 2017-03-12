//!
//! Module for dealing with spatial coordinates in astronomical calculations.
//!
//! Authors: Ryan Leach
//!
//! Copyright: Ryan Leach, 2017
//!
//! License: [BSD 3-clause](https://opensource.org/licenses/BSD-3-Clause)
//!
//! All of the coordinates carry a valid time with them. This is the epoch. The epoch may be the
//! standard epochs of 1950 or 2000, or it could be any other date.
mod precession;

use self::precession::*;
use std::fmt;
use super::angles::{RadianAngle, DegreeAngle, DMSAngle, HMSAngle, Angle};
use super::astro_time::AstroTime;

pub use self::precession::{EPSILON_2000, EPSILON_1950, J2050, J2000, B1950, B1900};

// TODO (**In Progress**) implement with low level, primitive type only, private functions closely
// tied to algorithms in the book.
//
//  SUB TODO - implement chpt 22 so I can use apparent coords and times
//  SUB TODO - account for proper motion - need a type.
//
// TODO Add factory functions to build all types and force invariants (e.g. lat-lon).
// TODO unit test everything
// TODO add trait constraint From for ecliptic and equatorial coords. HorizontalCoords cannot be
//      derived from the others without valid time and earth location. But, all the other types
//      should be derivable from HorizontalCoords. Galactic coords should be derivable from all
//      others, but all others need an epoch to be transformed into galactic coords.
// TODO add enum to tag coordinates as mean or apparent, because it can make a difference when
//      you need to calculate sidereal time.

/// Coordinate systems used in positional astronomy.
pub trait AstroCoordinate: fmt::Display {
    /// Get the epoch associated with these coordinates.
    fn epoch(&self) -> AstroTime;
}

/// Ecliptic coordinates are closely aligned with the mean plane of the planetary orbits in
/// our solar system, and also with the Sun's path through the sky.
///
/// Celestial longitude is measured from the vernal equinox along the ecliptic with positive values
/// westward. Celestial latitude is positive north of the ecliptic.
#[derive(Debug, Clone, Copy)]
pub struct EclipticCoords {
    latitude: RadianAngle,
    longitude: RadianAngle,
    epoch: AstroTime,
}

/// Equatorial coordinates are aligned with the Earth's equator and poles.
///
/// This is the most frequently used system, and is the system of the "fixed stars". Right
/// ascension is usually measured in hours, minutes, and seconds of time. Declination is measured
/// positive in the northern celestial hemisphere.
#[derive(Debug, Clone, Copy)]
pub struct EquatorialCoords {
    declination: RadianAngle,
    right_acension: RadianAngle,
    epoch: AstroTime,
}
impl EquatorialCoords {
    /// Build a new set of coordinates.
    pub fn new(right_acension: RadianAngle,
               declination: RadianAngle,
               epoch: AstroTime)
               -> EquatorialCoords {
        EquatorialCoords {
            right_acension: right_acension,
            declination: declination,
            epoch: epoch,
        }
    }

    /// Get the right acension.
    pub fn right_acension(&self) -> RadianAngle {
        self.right_acension
    }

    /// Get the declination.
    pub fn declination(&self) -> RadianAngle {
        self.declination
    }
}

impl AstroCoordinate for EclipticCoords {
    fn epoch(&self) -> AstroTime {
        self.epoch
    }
}
impl AstroCoordinate for EquatorialCoords {
    fn epoch(&self) -> AstroTime {
        self.epoch
    }
}

/// Galactic Coordinates with the galactic equator in the galactic plane, and the galactic north
/// pole is in the same hemisphere as the terrestrial north pole.
#[derive(Debug, Clone, Copy)]
pub struct GalacticCoords {
    latitude: RadianAngle,
    longitude: RadianAngle,
}

/// Coordinates in the sky from the point of view of an observer on Earth.
///
/// There are many conventions when measuring azimuth, for this library the azimuth is measured
/// westward of south. Altitude is measured positive above the horizon.
///
/// These do not implement the AstroCoordinate trait because it is not possible to calculate them
/// without a terrestrial location and time.
#[derive(Debug, Clone, Copy)]
pub struct HorizontalCoords {
    altitude: RadianAngle,
    azimuth: RadianAngle,
    observer_loc: GeoCoords,
    valid_time: AstroTime,
}

/// Geographic coordinates, latitude and longitude.
///
/// The Constructor and method `longitude()` will use the usual convention of increasing values to
/// the east. The internal value and `meeus_long()` method will use the old astronomical convention
/// of increasing longitude to the west. The algorithms in this library depend on the astronomical
/// convention.
#[derive(Debug, Clone, Copy)]
pub struct GeoCoords {
    latitude: RadianAngle,
    longitude: RadianAngle,
}

impl GeoCoords {
    /// Create a new location
    pub fn new_degrees(lat: DegreeAngle, lon: DegreeAngle) -> GeoCoords {
        GeoCoords {
            latitude: RadianAngle::from(lat),
            longitude: -RadianAngle::from(lon),
        }
    }

    /// Get the longitude with the typical convention of increasing values to the east.
    pub fn longitude(&self) -> DegreeAngle {
        DegreeAngle::from(-self.longitude)
    }

    /// Get the latitude
    pub fn latitude(&self) -> DegreeAngle {
        DegreeAngle::from(self.latitude)
    }

    /// Get the longitude with the astronomical convention of increasing longitude to the west.
    pub fn meeus_long(&self) -> RadianAngle {
        self.longitude
    }

    /// Radian latitude - for use in calculations.
    pub fn radian_lat(&self) -> RadianAngle {
        self.latitude
    }
}

impl fmt::Display for GalacticCoords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lat = DegreeAngle::from(self.latitude);
        let lon = DegreeAngle::from(self.longitude);
        write!(f,
               "Galactic Coordinates\n  latitude: {}\n  longitude: {}\n",
               lat,
               lon)
    }
}

impl fmt::Display for EclipticCoords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lat = DegreeAngle::from(self.latitude);
        let lon = DegreeAngle::from(self.longitude);
        write!(f,
               "Ecliptic Coordinates\n  latitude: {}\n  longitude: {}\n  epoch: {}\n",
               lat,
               lon,
               self.epoch)
    }
}

impl fmt::Display for EquatorialCoords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dec = DMSAngle::from(self.declination);
        let ra = HMSAngle::from(self.right_acension).map_to_time_range();
        write!(f,
               "Equatorial Coordinates\n  RA: {}\n  dec: {}\n  epoch: {}\n",
               ra,
               dec,
               self.epoch)
    }
}

impl fmt::Display for GeoCoords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lat = DegreeAngle::from(self.latitude());
        let lon = DegreeAngle::from(self.longitude());
        write!(f,
               "Geographic Location - latitude: {},  longitude: {}",
               lat,
               lon)
    }
}

impl fmt::Display for HorizontalCoords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let alt = DegreeAngle::from(self.altitude);
        let az = DegreeAngle::from(self.azimuth);
        write!(f,
               "Horizontal Coordinates\n  Alt: {}\n  Az: {}\n  valid: {}\n  for {}\n",
               alt,
               az,
               self.valid_time,
               self.observer_loc)
    }
}

// Calculate the local sidereal time
fn local_mean_sidereal_time(gmt: AstroTime, geo_location: GeoCoords) -> RadianAngle {
    let gst = gmt.mean_sidereal_greenwich();
    let long = geo_location.longitude;
    gst - long
}

// TODO local_apparent_sidereal_time

// Calculate the local hour angle
fn local_mean_hour_angle(gmt: AstroTime,
                         geo_location: GeoCoords,
                         equatorial_location: EquatorialCoords)
                         -> RadianAngle {
    let lst = local_mean_sidereal_time(gmt, geo_location);
    let alpha = equatorial_location.right_acension;
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

// Transform from equatorial to ecliptical coordinates.
fn trans_equatorial_to_ecliptical(eq: EquatorialCoords,
                                  obliquity_of_ecliptic: RadianAngle)
                                  -> EclipticCoords {
    let lon = RadianAngle::atan2(eq.right_acension.sin() * obliquity_of_ecliptic.cos() +
                                 eq.declination.tan() * obliquity_of_ecliptic.sin(),
                                 eq.right_acension.cos());
    let lat = RadianAngle::asin(eq.declination.sin() * obliquity_of_ecliptic.cos() -
                                eq.declination.cos() * obliquity_of_ecliptic.sin() *
                                eq.right_acension.sin());
    EclipticCoords {
        latitude: lat,
        longitude: lon,
        epoch: eq.epoch,
    }
}

// Transform from ecliptical to equatorial coordinates.
fn trans_ecliptical_to_equatorial(ec: EclipticCoords,
                                  obliquity_of_ecliptic: RadianAngle)
                                  -> EquatorialCoords {
    let ra = RadianAngle::atan2(ec.longitude.sin() * obliquity_of_ecliptic.cos() -
                                ec.latitude.tan() * obliquity_of_ecliptic.sin(),
                                ec.longitude.cos());
    let dec = RadianAngle::asin(ec.latitude.sin() * obliquity_of_ecliptic.cos() +
                                ec.latitude.cos() * obliquity_of_ecliptic.sin() *
                                ec.longitude.sin());
    EquatorialCoords {
        right_acension: ra,
        declination: dec,
        epoch: ec.epoch,
    }
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
    let phi = geo.latitude;
    let delta = eqa.declination;
    let az = RadianAngle::atan2(h.sin(), h.cos() * phi.sin() - delta.tan() * phi.cos());
    let alt = RadianAngle::asin(phi.sin() * delta.sin() + phi.cos() * delta.cos() * h.cos());

    HorizontalCoords {
        altitude: alt,
        azimuth: az,
        observer_loc: geo,
        valid_time: gmt,
    }
}

// Transform from horizontal to equatorial coordinates.
fn trans_horizontal_to_equatorial(hzc: HorizontalCoords, get_apparent: bool) -> EquatorialCoords {
    let az = hzc.azimuth;
    let phi = hzc.observer_loc.latitude;
    let alt = hzc.altitude;
    let h = RadianAngle::atan2(az.sin(), az.cos() * phi.sin() + alt.tan() * phi.cos());

    let ra = if get_apparent {
        // TODO right_acension_from_apparent_hour_angle(h, hzc.observer_loc, hzc.valid_time)
        //.map_to_time_range()
        right_acension_from_mean_hour_angle(h, hzc.observer_loc, hzc.valid_time).map_to_time_range()
    } else {
        right_acension_from_mean_hour_angle(h, hzc.observer_loc, hzc.valid_time).map_to_time_range()
    };
    let delta = RadianAngle::asin(phi.sin() * alt.sin() - phi.cos() * alt.cos() * az.cos());

    EquatorialCoords {
        declination: delta,
        right_acension: ra,
        epoch: hzc.valid_time,
    }
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
        let geo_loc = GeoCoords::new_degrees(DegreeAngle::from(DMSAngle::new(38, 55, 17.0)),
                                             DegreeAngle::from(DMSAngle::new(-77, 3, 56.0)));
        let astro_loc = EquatorialCoords {
            right_acension: RadianAngle::from(HMSAngle::new(23, 9, 16.641)),
            declination: RadianAngle::from(DMSAngle::new(-6, 43, 11.61)),
            epoch: gmt,
        };
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
        let eq_coords = EquatorialCoords {
            right_acension: RadianAngle::from(HMSAngle::new(7, 45, 18.946)),
            declination: RadianAngle::from(DMSAngle::new(28, 1, 34.26)),
            epoch: J2000.clone(),
        };
        let obliquity = RadianAngle::from(DegreeAngle::new(23.4392911));

        let ec_coords = trans_equatorial_to_ecliptical(eq_coords, obliquity);

        println!("\nPosition in EclipticCoords:\n{}\n", ec_coords);

        assert!(approx_eq(DegreeAngle::from(ec_coords.latitude).degrees(),
                          6.684170,
                          1.0e-6));
        assert!(approx_eq(DegreeAngle::from(ec_coords.longitude).degrees(),
                          113.215630,
                          1.0e-6));

        let eq_back = trans_ecliptical_to_equatorial(ec_coords, obliquity);
        println!("Position in EquatorialCoords: \n{}", eq_back);

        assert!(approx_eq(eq_back.right_acension.radians(),
                          eq_coords.right_acension.radians(),
                          1.0e-15));
        assert!(approx_eq(eq_back.declination.radians(),
                          eq_coords.declination.radians(),
                          1.0e-15));
    }

    #[test]
    fn test_trans_equatorial_to_horizontal_and_back() {
        // example from page 95 of Meuus
        let vtime = Builder::from_gregorian_utc(1987, 4, 10, 19, 21, 0).build().unwrap();

        // TODO adjusted RA manually to get apparent local hour angle. I need a function
        // to make adjustments in chpt 22 for apparent sidereal time since these are apparent coords
        let eq_coords = EquatorialCoords {
            right_acension: RadianAngle::from(HMSAngle::new(23, 9, 16.8746)),
            declination: RadianAngle::from(DMSAngle::new(-6, 43, 11.61)),
            epoch: vtime,
        };

        println!("Position in original EquatorialCoords: \n{}", eq_coords);

        let geo_coords = GeoCoords::new_degrees(DegreeAngle::from(DMSAngle::new(38, 55, 17.0)),
                                                DegreeAngle::from(DMSAngle::new(-77, 3, 56.0)));

        let h_coords = trans_equatorial_to_horizontal(eq_coords, geo_coords, vtime, false);

        println!("\nPosition in horizontal coordinates:\n{}\n", h_coords);

        assert!(approx_eq(DegreeAngle::from(h_coords.altitude).degrees(),
                          15.1249,
                          1.0e-3));
        assert!(approx_eq(DegreeAngle::from(h_coords.azimuth).degrees(),
                          68.0337,
                          1.0e-3));

        let h_back = trans_horizontal_to_equatorial(h_coords, false);
        println!("Position in back EquatorialCoords: \n{}", h_back);

        assert!(approx_eq(h_back.right_acension.radians(),
                          eq_coords.right_acension.radians(),
                          1.0e-15));
        assert!(approx_eq(h_back.declination.radians(),
                          eq_coords.declination.radians(),
                          1.0e-15));
    }
}
