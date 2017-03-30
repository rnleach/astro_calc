//!
//! Equatorial coordinate system.
//!
//! Authors: Ryan Leach
//!
//! Copyright: Ryan Leach, 2017
//!
//! License: [BSD 3-clause](https://opensource.org/licenses/BSD-3-Clause)
//!
use super::*;
use super::super::angles::Angle;

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
    valid_time: AstroTime,
}

impl EquatorialCoords {
    /// Build a new set of coordinates.
    pub fn new<T, U>(right_acension: T,
                     declination: U,
                     epoch: AstroTime,
                     valid_time: AstroTime)
                     -> EquatorialCoords
        where RadianAngle: From<T> + From<U>
    {
        EquatorialCoords {
            right_acension: RadianAngle::from(right_acension),
            declination: RadianAngle::from(declination),
            epoch: epoch,
            valid_time: valid_time,
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

impl AstroCoordinate for EquatorialCoords {}

impl HasEpoch for EquatorialCoords {
    fn epoch(&self) -> AstroTime {
        self.epoch
    }
}

impl HasValidTime for EquatorialCoords {
    fn valid_time(&self) -> AstroTime {
        self.valid_time
    }
}

impl fmt::Display for EquatorialCoords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dec = DMSAngle::from(self.declination);
        let ra = HMSAngle::from(self.right_acension).map_to_time_range();
        write!(f,
               "Equatorial Coordinates\n  RA(\u{03B1}): {}\n  dec(\u{03B4}): {}\n  epoch: {}\n  \
                    valid_time: {}\n",
               ra,
               dec,
               self.epoch,
               self.valid_time)
    }
}