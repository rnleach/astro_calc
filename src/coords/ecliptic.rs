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
}

impl EclipticCoords {
    /// Build a new set of coordinates.
    pub fn new(latitude: RadianAngle,
               longitude: RadianAngle,
               epoch: AstroTime)
               -> EclipticCoords {
        EclipticCoords {
            latitude: latitude,
            longitude: longitude,
            epoch: epoch,
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

impl AstroCoordinate for EclipticCoords {
    fn epoch(&self) -> AstroTime {
        self.epoch
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