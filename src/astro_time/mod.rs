//!
//! Module contains library wide utilities and constants.
//!
//! Authors: Ryan Leach
//! Copyright: Ryan Leach, 2017
//! License: BSD 3-clause, https://opensource.org/licenses/BSD-3-Clause
//!

/// Represent different types of time. 
///
/// Dynamic Time is measured by atomic clocks and represents the kind of time
/// you do physical calculations with. Universal time is determined by the 
/// position of Earth with respect to the Sun and varies by leap seconds to
/// account for minor changes in Earth's orbit.
pub enum TimeType {
    /// Universal Time, also known as UTC, Zulu, or GMT
    UT,
    /// Dynamic Time
    DT,
}

/// Represent a time.
///
/// The internal representation is as a Julian Day number, but it is only valid
/// for dates with Julian Day number >= 0.0. Many methods check for this and
/// will panic if anything else is found.
pub struct AstroTime {
    julian_day: f64,
    time_type : TimeType, 
}

impl AstroTime {

    /// Create an AstroTime from a Julian Day number.
    ///
    /// It defaults to `TimeType::UT`.
    pub fn from_raw( raw: f64 ) -> AstroTime {
        // Only valid for values > 0
        assert!( raw >= 0.0 );
        AstroTime { julian_day: raw, time_type: TimeType::UT }
    }

    /// Create from a date and time in the Gregorian calendar assuming it is in
    /// the UTC time zone.
    //
    // It defaults to `TimeType::UT`.
    pub fn from_gregorian_utc( mut year: i32,  mut month: i32, day: i32, 
        hour: i32, minute: i32, second: i32 ) -> AstroTime {
        // From chapter 7, pages 60-61 of Astronomical Algorithms, 2nd Edition 
        // by Jean Meeus.
        use std::f64;

        let decimal_day = day as f64 + day_fraction( hour, minute, second );

        if month < 3 {
            year -= 1;
            month += 12;
        }

        #[allow(non_snake_case)]
        let A = f64::floor( year as f64 / 100.0 );
        #[allow(non_snake_case)]
        let B = 2.0 - A + f64::floor( A / 4.0 );

        let jd = f64::floor( 365.25 * ( year + 4716) as f64 ) +
            f64::floor( 30.6001 * ( month + 1 ) as f64) + 
            decimal_day + B - 1524.5;

        AstroTime{ julian_day: jd, time_type: TimeType::UT }
    }

    /// Set the Time type to `TimeType::DT` to mark this as a dynamical time.
    ///
    /// For a reference of dynamical time vs. UTC, see chapter 10 of 
    /// Astronomical Algorithms 2nd ed, by Jean Meeus.
    pub fn dynamical_time( mut self ) -> AstroTime {
        self.time_type = TimeType::DT;
        self
    }

    /// Get the Julian Day number as a floating point value.
    pub fn julian_day_number( &self ) -> f64
    {
        self.julian_day
    }
}

// calculate the fraction of the day
fn day_fraction( hour: i32, minute: i32, second: i32 ) -> f64 {
    assert!( hour >= 0 && hour < 24 );
    assert!( minute >= 0 && minute < 60 );
    assert!( second >= 0 && second < 60 );

    (hour as f64 + ( minute as f64 + second as f64 / 60.0 ) / 60.0 ) / 24.0
}

#[cfg(test)]
mod tests {
    use astro_time::*;

    #[test]
    fn test_from_raw() {
        let test_time = AstroTime::from_raw( 110.0 );
    }
}
