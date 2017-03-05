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
mod angles;

use super::astro_time::*;

pub use self::angles::*;

// TODO implement with low level, primitive type only, private functions closely tied to algorithms
// in the book.
//
// TODO Add factory functions to build all types and force invariants (e.g. lat-lon).
// TODO unit test everything
// TODO add trait constraint for From, all types should be able to be converted from all others,
//      except HorizontalCoords cannot be derived from the others (and is not an AstroCoordinate)
//      but, all the other types should be derivable from HorizontalCoords.
// TODO add Display trait to AstroCoordinate and HorizontalCoords, and terrestrial coordinates

// This is a heavy handed solution for such small constants, but I cannot do compile time function
// evaluation yet (maybe later). If I calculated the value directly in radians, I would still have
// to define the constant in the angles sub-module, but it belongs here.
lazy_static! {
    /// Obliquity of the ecliptic for the standard 2000 epoch.
    pub static ref EPSILON_2000: RadianAngle =
        RadianAngle::from(DegreeAngle::new( 23.439_291_1 ));

    /// Obliquity of the ecliptic for the standard 1950 epoch.
    pub static ref EPSILON_1950: RadianAngle =
        RadianAngle::from(DegreeAngle::new( 23.445_788_9 ));
}

/// Coordinate systems used in positional astronomy.
pub trait AstroCoordinate {}

/// Galactic Coordinates with the galactic equator in the galactic plane, and the galactic north
/// pole is in the same hemisphere as the terrestrial north pole.
#[derive(Debug, Clone, Copy)]
pub struct GalacticCoords {
    latitude: RadianAngle,
    longitude: RadianAngle,
    epoch: AstroTime,
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

impl AstroCoordinate for GalacticCoords {}
impl AstroCoordinate for EclipticCoords {}
impl AstroCoordinate for EquatorialCoords {}

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

// Calculate the local sidereal time
fn local_sidereal_time(gmt: AstroTime, geo_location: GeoCoords) -> DegreeAngle {
    let gst = gmt.sidereal_greenwich();
    let long = geo_location.longitude;
    gst - long
}

// Calculate the local hour angle
fn local_hour_angle(gmt: AstroTime,
                    geo_location: GeoCoords,
                    equatorial_location: EquatorialCoords)
                    -> DegreeAngle {
    let lst = local_sidereal_time(gmt, geo_location);
    let alpha = equatorial_location.right_acension;
    lst - alpha
}

// Transform from equatorial to ecliptical coordinates.
fn trans_equatorial_to_ecliptical(eq: EquatorialCoords,
                                  obliquity_of_ecliptic: RadianAngle)
                                  -> EclipticCoords {
    EclipticCoords {
        latitude: RadianAngle::new(0.0),
        longitude: RadianAngle::new(0.0),
        epoch: eq.epoch,
    }
}

// test approximate equality, only used in unit tests.
#[cfg(test)]
fn approx_eq(left: f64, right: f64, tol: f64) -> bool {
    (left - right).abs() < tol
}

#[cfg(test)]
mod private_test {
    use super::*;

    #[test]
    fn test_local_hour_angle() {
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
                 HMSAngle::from(local_hour_angle(gmt, geo_loc, astro_loc).map_to_time_range() -
                                DegreeAngle::new(0.0009858333333) -
                                DegreeAngle::new(64.352133)));
        println!("Error = {}",
                 DMSAngle::from(local_hour_angle(gmt, geo_loc, astro_loc).map_to_time_range() -
                                DegreeAngle::new(0.0009858333333) -
                                DegreeAngle::new(64.352133)));
        println!("Error = {}",
                 DegreeAngle::from(local_hour_angle(gmt, geo_loc, astro_loc).map_to_time_range() -
                                   DegreeAngle::new(0.0009858333333) -
                                   DegreeAngle::new(64.352133)));
        println!("Error = {}",
                 RadianAngle::from(local_hour_angle(gmt, geo_loc, astro_loc).map_to_time_range() -
                                   DegreeAngle::new(0.0009858333333) -
                                   DegreeAngle::new(64.352133)));
        println!();
        assert!(approx_eq(local_hour_angle(gmt, geo_loc, astro_loc)
                              .map_to_time_range()
                              .degrees() - 0.0009858333333,
                          64.352133,
                          1.4e-4));
    }
}
