//!
//! Geographic coordinate system for describing a position on eart.
//!
//! Authors: Ryan Leach
//!
//! Copyright: Ryan Leach, 2017
//!
//! License: [BSD 3-clause](https://opensource.org/licenses/BSD-3-Clause)
//!
use super::*;

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
    pub fn new<T, U>(lat: T, lon: U) -> GeoCoords
        where RadianAngle: From<T> + From<U>
    {
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
