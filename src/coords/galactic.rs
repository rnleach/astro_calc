//!
//! Galactic coordinate system.
//!
//! Authors: Ryan Leach
//!
//! Copyright: Ryan Leach, 2017
//!
//! License: [BSD 3-clause](https://opensource.org/licenses/BSD-3-Clause)
//!
use super::*;

/// Galactic Coordinates with the galactic equator in the galactic plane, and the galactic north
/// pole is in the same hemisphere as the terrestrial north pole.
#[derive(Debug, Clone, Copy)]
pub struct GalacticCoords {
    latitude: RadianAngle,
    longitude: RadianAngle,
}

impl GalacticCoords {
    /// Create a new location
    pub fn new<T, U>(lat: T, lon: U) -> GalacticCoords
        where RadianAngle: From<T> + From<U>
    {
        GalacticCoords {
            latitude: RadianAngle::from(lat),
            longitude: RadianAngle::from(lon),
        }
    }

    /// Get the longitude.
    pub fn longitude(&self) -> RadianAngle {
        self.longitude
    }

    /// Get the latitude
    pub fn latitude(&self) -> RadianAngle {
        self.latitude
    }
}

impl fmt::Display for GalacticCoords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lat = DegreeAngle::from(self.latitude);
        let lon = DegreeAngle::from(self.longitude);
        write!(f,
               "Galactic Coordinates\n  latitude(l): {}\n  longitude(b): {}\n",
               lat,
               lon)
    }
}
