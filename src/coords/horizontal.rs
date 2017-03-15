//!
//! Horizontal coordinate system.
//!
//! Authors: Ryan Leach
//!
//! Copyright: Ryan Leach, 2017
//!
//! License: [BSD 3-clause](https://opensource.org/licenses/BSD-3-Clause)
//!
use super::*;

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

impl HorizontalCoords {
    /// Build a new set of coordinates.
    pub fn new<T, U>(altitude: T,
                     azimuth: U,
                     observer_loc: GeoCoords,
                     valid_time: AstroTime)
                     -> HorizontalCoords
        where RadianAngle: From<T> + From<U>
    {
        HorizontalCoords {
            altitude: RadianAngle::from(altitude),
            azimuth: RadianAngle::from(azimuth),
            observer_loc: observer_loc,
            valid_time: valid_time,
        }
    }

    /// Get the altitude.
    pub fn altitude(&self) -> RadianAngle {
        self.altitude
    }

    /// Get the azimuth.
    pub fn azimuth(&self) -> RadianAngle {
        self.azimuth
    }

    /// Get the observers location.
    pub fn observer_location(&self) -> GeoCoords {
        self.observer_loc
    }
}

impl HasValidTime for HorizontalCoords {
    /// Get the valid time
    fn valid_time(&self) -> AstroTime {
        self.valid_time
    }
}

impl fmt::Display for HorizontalCoords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let alt = DegreeAngle::from(self.altitude);
        let az = DegreeAngle::from(self.azimuth);
        write!(f,
               "Horizontal Coordinates\n  Alt(h): {}\n  Az(A): {}\n  valid: {}\n  for {}\n",
               alt,
               az,
               self.valid_time,
               self.observer_loc)
    }
}
