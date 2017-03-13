//!
//! Ecliptic coordinate system.
//!
//! Authors: Ryan Leach
//!
//! Copyright: Ryan Leach, 2017
//!
//! License: [BSD 3-clause](https://opensource.org/licenses/BSD-3-Clause)
//!
use super::*;

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
    valid_time: AstroTime,
}

impl EclipticCoords {
    /// Build a new set of coordinates.
    pub fn new<T, U>(latitude: T,
                     longitude: U,
                     epoch: AstroTime,
                     valid_time: AstroTime)
                     -> EclipticCoords
        where RadianAngle: From<T> + From<U>
    {
        EclipticCoords {
            latitude: RadianAngle::from(latitude),
            longitude: RadianAngle::from(longitude),
            epoch: epoch,
            valid_time: valid_time,
        }
    }

    /// Get the latitude.
    pub fn latitude(&self) -> RadianAngle {
        self.latitude
    }

    /// Get the longitude.
    pub fn longitude(&self) -> RadianAngle {
        self.longitude
    }
}

impl AstroCoordinate for EclipticCoords {}

impl HasEpoch for EclipticCoords {
    fn epoch(&self) -> AstroTime {
        self.epoch
    }
}

impl HasValidTime for EclipticCoords {
    fn valid_time(&self) -> AstroTime {
        self.valid_time
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
