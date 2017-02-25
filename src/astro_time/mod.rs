//!
//! Module for dealing with time in astronomical calculations.
//!
//! Authors: Ryan Leach
//!
//! Copyright: Ryan Leach, 2017
//!
//! License: [BSD 3-clause](https://opensource.org/licenses/BSD-3-Clause)
//!
use std::cmp::Ordering;
use std::option::Option;

use super::error::*;

mod time_data;

/// Represent different types of time. 
///
/// Dynamic Time is measured by atomic clocks and represents the kind of time you do physical 
/// calculations with. Universal time is determined by the position of Earth with respect to the 
/// Sun and varies by leap seconds to account for minor changes in Earth's orbit.
#[derive(PartialEq, Eq, Debug)]
pub enum TimeType {
    /// Universal Time, also known as UTC, Zulu, or GMT
    UT,
    /// Dynamic Time
    DT,
}

/// Builder for AstroTime
#[derive(Debug)]
pub struct Builder {
    target: AstroResult<AstroTime>,
}

impl Builder {
    /// Create an AstroTime from a Julian Day number.
    ///
    /// It defaults to `TimeType::UT`.
    pub fn from_julian_date( raw: f64 ) -> Builder {
        if raw >= 0.0 {
            Builder { target: Ok( AstroTime { julian_day: raw, time_type: TimeType::UT } ) }
        } else {
            Builder { target: Err( AstroAlgorithmsError::RangeError( 
                DateRangeError::DateUnderflow( raw, 0.0 )
            ))}
        }
    }

    /// Create from a date and time in the Gregorian calendar assuming it is in the UTC time zone.
    ///
    /// It defaults to `TimeType::UT`.
    pub fn from_gregorian_utc( mut year: i32,  mut month: i32, day: i32, hour: i32, minute: i32, 
        second: i32 ) -> Builder {
        // From chapter 7, pages 60-61 of Astronomical Algorithms, 2nd Edition by Jean Meeus.
        use std::f64;

        if is_valid_gregorian(year, month, day) && is_valid_time(hour, minute, second) {

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
                     f64::floor( 30.6001 * ( month + 1 ) as f64) + decimal_day + B - 1524.5;

            if jd >= 0.0 {
                Builder { target: Ok( AstroTime{ julian_day: jd, time_type: TimeType::UT } ) }
            }
            else {
                Builder { target: Err( AstroAlgorithmsError::RangeError( 
                    DateRangeError::DateUnderflow( jd, 0.0 )
                ))}
            }
        } else if !is_valid_gregorian(year, month, day) { 
            Builder { target: 
                Err(AstroAlgorithmsError::InvalidGregorianDate(year, month, day)) 
            }
        } else {
            Builder { target: Err(AstroAlgorithmsError::InvalidTime(hour, minute, second)) }
        }
    }

    /// Create from a date and time in the Gregorian calendar assuming it is in the UTC time zone.
    ///
    /// It defaults to `TimeType::UT`.
    pub fn from_julian_utc( mut year: i32,  mut month: i32, day: i32, hour: i32, minute: i32, 
        second: i32 ) -> Builder {
        // From chapter 7, pages 60-61 of Astronomical Algorithms, 2nd Edition by Jean Meeus.
        use std::f64;

        if is_valid_julian( year, month, day ) && is_valid_time(hour, minute, second) {

            let decimal_day = day as f64 + day_fraction( hour, minute, second );

            if month < 3 {
                year -= 1;
                month += 12;
            }

            let jd = f64::floor( 365.25 * ( year + 4716) as f64 ) +
                f64::floor( 30.6001 * ( month + 1 ) as f64) + decimal_day - 1524.5;

            if jd >= 0.0 {
                Builder { target: Ok( AstroTime{ julian_day: jd, time_type: TimeType::UT } ) }
            }
            else {
                Builder { target: Err( AstroAlgorithmsError::RangeError( 
                        DateRangeError::DateUnderflow( jd, 0.0 )
                ))}
            }
        } else if !is_valid_julian(year, month, day) { 
            Builder { target: Err( AstroAlgorithmsError::InvalidJulianDate(year, month, day)) }
        } else {
            Builder { target: Err(AstroAlgorithmsError::InvalidTime(hour, minute, second)) }
        }
    }

    /// Set the Time type to `TimeType::DT` to mark this as a dynamical time.
    ///
    /// For a reference of dynamical time vs. UTC, see chapter 10 of Astronomical Algorithms 
    /// 2nd ed, by Jean Meeus.
    ///
    /// Note that this DOES NOT DO ANY CONVERSION from UTC to dynamcial time using delta-t. It is
    /// only for specifying a dynamical time while building.
    pub fn dynamical_time( self ) -> Builder {
        match self.target {
            Ok(mut atime ) => {
                atime.time_type = TimeType::DT;
                Builder{ target: Ok( atime ) }
            }
            _ => self, // do nothing
        }
    }

    /// Finish building and get result.
    pub fn build( self ) ->  AstroResult<AstroTime> {
        self.target
    }
        
}
#[cfg(test)]
mod astro_tm_bldr_tests {
    use astro_time::*;

    #[test]
    fn test_from_julian_date() {
        let test_time = Builder::from_julian_date( 110.0 ).build().unwrap();
        let jd = test_time.julian_day_number();
        assert!( jd == 110.0 );

        let test_time = Builder::from_julian_date( -110.0 ).build();
        assert!( test_time.is_err());

        if let AstroAlgorithmsError::RangeError(DateRangeError::DateUnderflow(src, thresh)) =
        test_time.unwrap_err() {
            assert!(src == -110.0 );
            assert!(thresh == 0.0 );
        } else {
            panic!("Wrong error type returned.");
        }
    }

    #[test]
    fn test_from_gregorian_utc() {

        //
        // Test things that should work
        //
        assert!( approx_eq(
            Builder::from_gregorian_utc( -99, 3, 1, 0, 0, 0 )
                .build().unwrap().julian_day_number(), 
            1_684_959.5, 1.0e-15
        ));

        assert!( approx_eq(
            Builder::from_gregorian_utc( 1957, 10, 4, 19, 26, 24 )
                .build().unwrap().julian_day_number(), 
            2_436_116.31, 1.0e-15
        ));

        assert!( approx_eq(
            Builder::from_gregorian_utc( 2000, 1, 1, 12, 0, 0 )
                .build().unwrap().julian_day_number(), 
            2_451_545.0, 1.0e-15
        ));

        assert!( approx_eq(
            Builder::from_gregorian_utc( 1999, 1, 1, 0, 0, 0 )
                .build().unwrap().julian_day_number(), 
            2_451_179.5, 1.0e-15
        ));

        assert!( approx_eq(
            Builder::from_gregorian_utc( 1987, 1, 27, 0, 0, 0 )
                .build().unwrap().julian_day_number(), 
            2_446_822.5, 1.0e-15
        ));

        assert!( approx_eq(
            Builder::from_gregorian_utc( 1987, 6, 19, 12, 0, 0 )
                .build().unwrap().julian_day_number(), 
            2_446_966.0, 1.0e-15
        ));

        assert!( approx_eq(
            Builder::from_gregorian_utc( 1988, 1, 27, 0, 0, 0 )
                .build().unwrap().julian_day_number(), 
            2_447_187.5, 1.0e-15
        ));

        assert!( approx_eq(
            Builder::from_gregorian_utc( 1988, 6, 19, 12, 0, 0 )
                .build().unwrap().julian_day_number(), 
            2_447_332.0, 1.0e-15
        ));

        assert!( approx_eq(
            Builder::from_gregorian_utc( 1900, 1, 1, 0, 0, 0 )
                .build().unwrap().julian_day_number(), 
            2_415_020.5, 1.0e-15
        ));

        assert!( approx_eq(
            Builder::from_gregorian_utc( 1600, 1, 1, 0, 0, 0 )
                .build().unwrap().julian_day_number(), 
            2_305_447.5, 1.0e-15
        ));

        assert!( approx_eq(
            Builder::from_gregorian_utc( 1600, 12, 31, 0, 0, 0 )
                .build().unwrap().julian_day_number(), 
            2_305_812.5, 1.0e-15
        ));

        assert!( approx_eq(
            Builder::from_gregorian_utc( 837, 4, 14, 7, 12, 0 )
                .build().unwrap().julian_day_number(), 
            2_026_871.8, 1.0e-15
        ));

        assert!( approx_eq(
            Builder::from_gregorian_utc( -123, 12, 28, 0, 0, 0 )
                .build().unwrap().julian_day_number(), 
            1_676_496.5, 1.0e-15
        ));

        assert!( approx_eq(
            Builder::from_gregorian_utc( -123, 12, 29, 0, 0, 0 )
                .build().unwrap().julian_day_number(), 
            1_676_497.5, 1.0e-15
        ));

        assert!( approx_eq(
            Builder::from_gregorian_utc( -200, 7, 2, 12, 0, 0 )
                .build().unwrap().julian_day_number(), 
            1_648_194.0, 1.0e-15
        ));

        assert!( approx_eq(
            Builder::from_gregorian_utc( -1000, 7, 3, 12, 0, 0 )
                .build().unwrap().julian_day_number(), 
            1_356_001.0, 1.0e-15
        ));

        assert!( approx_eq(
            Builder::from_gregorian_utc( -4713, 11, 24, 12, 0, 0 )
                .build().unwrap().julian_day_number(), 
            0.0, 1.0e-15
        ));

        //
        // Test things that should fail
        //
        let test_time = Builder::from_gregorian_utc( -4713, 11, 24, 11, 59, 59 ).build();
        assert!( test_time.is_err());
        if let AstroAlgorithmsError::RangeError(DateRangeError::DateUnderflow(_, thresh)) =
        test_time.unwrap_err() {
            assert!(thresh == 0.0 );
        } else {
            panic!("Wrong error type returned.");
        }

        let test_time = Builder::from_gregorian_utc( 1999, 2, 29, 11, 59, 59 ).build();
        assert!( test_time.is_err());
        if let AstroAlgorithmsError::InvalidGregorianDate(year, month, day) =
        test_time.unwrap_err() {
            assert!(year == 1999 && month == 2 && day == 29);
        } else {
            panic!("Wrong error type returned.");
        }

        let test_time = Builder::from_gregorian_utc( 1999, 2, 28, 24, 59, 59 ).build();
        assert!( test_time.is_err());
        if let AstroAlgorithmsError::InvalidTime(hour, minute, second) =
        test_time.unwrap_err() {
            assert!(hour == 24 && minute == 59 && second == 59);
        } else {
            panic!("Wrong error type returned.");
        }
    }

    #[test]
    fn test_from_julian_utc() {

        //
        // Test things that should work
        //
        assert!( approx_eq(
            Builder::from_julian_utc( 1957, 9, 21, 19, 26, 24 )
                .build().unwrap().julian_day_number(),
            2_436_116.31, 1.0e-15
        ));

        assert!( approx_eq(
            Builder::from_julian_utc( 1999, 12, 19, 12, 0, 0 )
                .build().unwrap().julian_day_number(),
            2_451_545.0, 1.0e-15
        ));

        assert!( approx_eq(
            Builder::from_julian_utc( 1998, 12, 19, 0, 0, 0 )
                .build().unwrap().julian_day_number(),
            2_451_179.5, 1.0e-15
        ));

        assert!( approx_eq(
            Builder::from_julian_utc( 837, 4, 10, 7, 12, 0 )
                .build().unwrap().julian_day_number(),
            2_026_871.8, 1.0e-15
        ));

        assert!( approx_eq(
            Builder::from_julian_utc( -123, 12, 31, 0, 0, 0 )
                .build().unwrap().julian_day_number(),
            1_676_496.5, 1.0e-15
        ));

        assert!( approx_eq(
            Builder::from_julian_utc( -122, 1, 1, 0, 0, 0 )
                .build().unwrap().julian_day_number(),
            1_676_497.5, 1.0e-15
        ));

        assert!( approx_eq(
            Builder::from_julian_utc( -1000, 7, 12, 12, 0, 0 )
                .build().unwrap().julian_day_number(),
            1_356_001.0, 1.0e-15
        ));

        assert!( approx_eq(
            Builder::from_julian_utc( -1000, 2, 29, 0, 0, 0 )
                .build().unwrap().julian_day_number(),
            1_355_866.5, 1.0e-15
        ));

        assert!( approx_eq(
            Builder::from_julian_utc( -1001, 8, 17, 21, 36, 0 )
                .build().unwrap().julian_day_number(),
            1_355_671.4, 1.0e-15
        ));

        assert!( approx_eq(
            Builder::from_julian_utc( -4712, 1, 1, 12, 0, 0 )
                .build().unwrap().julian_day_number(),
            0.0, 1.0e-15
        ));

        //
        // Test things that should fail
        //
        let test_time = Builder::from_julian_utc( -4712, 1, 1, 11, 59, 59 ).build();
        assert!( test_time.is_err());
        if let AstroAlgorithmsError::RangeError(DateRangeError::DateUnderflow(_, thresh)) =
        test_time.unwrap_err() {
            assert!(thresh == 0.0 );
        } else {
            panic!("Wrong error type returned.");
        }

        let test_time = Builder::from_julian_utc( 1999, 2, 29, 11, 59, 59 ).build();
        assert!( test_time.is_err());
        if let AstroAlgorithmsError::InvalidJulianDate(year, month, day) =
        test_time.unwrap_err() {
            assert!(year == 1999 && month == 2 && day == 29);
        } else {
            panic!("Wrong error type returned.");
        }

        let test_time = Builder::from_julian_utc( 1999, 2, 28, 24, 59, 59 ).build();
        assert!( test_time.is_err());
        if let AstroAlgorithmsError::InvalidTime(hour, minute, second) =
        test_time.unwrap_err() {
            assert!(hour == 24 && minute == 59 && second == 59);
        } else {
            panic!("Wrong error type returned.");
        }
    }
}

/// Represent a time.
///
/// The internal representation is as a Julian Day number, but it is only valid for dates with 
/// Julian Day number >= 0.0. Many methods check for this and will return an error if found.
#[derive(Debug)]
pub struct AstroTime {
    julian_day: f64,
    time_type : TimeType, 
}

impl PartialEq for AstroTime {
    
    fn eq(&self, other: &AstroTime) -> bool {
        self.time_type == other.time_type && self.julian_day == other.julian_day
    }
}

impl PartialOrd for AstroTime {

    fn partial_cmp(&self, other: &AstroTime) -> Option<Ordering> {
        if self.time_type == other.time_type {
            self.julian_day.partial_cmp( &other.julian_day )
        } else {
            None
        }
    }
}

impl AstroTime {

    /// Get the Julian Day number as a floating point value.
    pub fn julian_day_number( &self ) -> f64 {
        self.julian_day
    }

    /// Get the Modified Julian Day as defined by the number of days since midnight
    /// 17 November, 1858.
    pub fn modified_julian_day_number( &self ) -> f64 {
        self.julian_day - 2_400_000.5
    }

    /// Get the year, month, day, hour, minute, and second in the UTC time zone.
    ///
    /// # Examples
    ///
    /// ```
    /// # use astro_calc::astro_time::Builder;
    /// let a_date = Builder::from_gregorian_utc( 2017, 2, 11, 19, 58, 5).build().unwrap();
    /// let (year, month, day, hour, minute, second) = a_date.to_gregorian_utc();
    /// assert!( year == 2017 );
    /// assert!( month == 2 );
    /// assert!( day == 11 );
    /// assert!( hour == 19 );
    /// assert!( minute == 58 );
    /// assert!( second == 5 );
    /// ```
    pub fn to_gregorian_utc( &self ) -> ( i32, i32, i32, i32, i32, i32 ) {
        // Adapted from chapter 7, pages 60-61 of Astronomical Algorithms, 2nd Edition 
        // by Jean Meeus.
        use std::f64;

        let z = f64::floor( self.julian_day + 0.5 );
        let f = self.julian_day + 0.5 - z;

        let alpha = f64::floor( ( z - 1_867_216.25 ) / 36_524.25 );
        let a = z + 1.0 + alpha - f64::floor( alpha / 4.0 );

        let b = a + 1524.0;
        let c = f64::floor( ( b - 122.1 ) / 365.25 );
        let d = f64::floor( 365.25 * c );
        let e = f64::floor( ( b - d ) / 30.6001 );

        // Extra call to floor to ensure rounding mode
        let day = f64::floor( b - d - f64::floor( 30.6001 * e)) as i32;
        let mut month = f64::floor( e - 1.0 ) as i32;
        if e > 13.0 { month = f64::floor( e - 13.0 ) as i32; }

        let year = f64::floor( if month > 2 { c - 4716.0 } else { c - 4715.0 } ) as i32;

        let ( hour, minute, second ) = to_hms( f );

        ( year, month, day, hour, minute, second )
    }

    /// Whatever time type (dynamical or UTC) create a copy in UTC by applying an *_approximate_* 
    /// conversion. This can be wildly inaccurate for years before 1620 and years after 2017. 
    /// Future dates use a forecasted correction value, which is very hard to predict.
    ///
    /// Offsets are linearly interpolated from data take from "Astronomical Algorithms, 2nd ed." 
    /// by Jean Meeus, pg 79 and some data downloaded from the US Navy's website. It is hard coded
    /// into the library.
    pub fn as_utc( &self ) -> AstroResult<AstroTime> {
        if self.time_type == TimeType::UT {
            Builder::from_julian_date( self.julian_day ).build()
        }
        else {
            let dt = self.get_delta_t();
            Builder::from_julian_date( self.julian_day - dt ).build()
        }
    }

    // Whatever time type (dynamical or UTC) create a copy in dynamical time by applying an
    /// *_approximate_* conversion. This can be wildly inaccurate for years before 1620 and years
    /// after 2017. Future dates use a forecasted correction value, which is very hard to predict.
    ///
    /// Offsets are linearly interpolated from data take from "Astronomical Algorithms, 2nd ed."
    /// by Jean Meeus, pg 79 and some data downloaded from the US Navy's website. It is hard coded
    /// into the library.
    pub fn as_dt( &self ) -> AstroResult<AstroTime> {
        if self.time_type == TimeType::DT {
            Builder::from_julian_date( self.julian_day ).dynamical_time().build()
        }
        else {
            let dt = self.get_delta_t();
            Builder::from_julian_date( self.julian_day + dt ).dynamical_time().build()
        }
    }

    // Calculate the delta-t value for applying a conversion between unversal 
    // and dynamical time.
    fn get_delta_t( &self ) -> f64 {
        use self::time_data::TIME_DELTA;
        use std::usize::MAX;

        // Use linear interpolation on the table if possible
        if self.julian_day  >= TIME_DELTA[0].0 && 
            self.julian_day < TIME_DELTA[ TIME_DELTA.len() - 1 ].0 
        {
            let mut i: usize = MAX;
            for ii in  ( 0..(TIME_DELTA.len() - 1) ).rev()
            {
                let ( jd, _ ) = TIME_DELTA[ii];
                if jd < self.julian_day { 
                    i = ii;
                    break; 
                }
            }

            debug_assert!( i < TIME_DELTA.len() - 1 );
            let ( left, _ )   = TIME_DELTA[i];
            let ( right, _ )  = TIME_DELTA[i + 1];
            let ( _, bottom ) = TIME_DELTA[i];
            let ( _, top )    = TIME_DELTA[i + 1];

            (( top - bottom ) / ( right - left ) * 
                    ( self.julian_day - left ) + bottom ) / 86_400.0
        }
        else {

            // Algorithm adapted from chapter 10, pages 78-80 of Astronomical 
            // Algorithms,  2nd Edition by Jean Meeus.
            let t: f64 = ( self.julian_day - 
                Builder::from_gregorian_utc( 2000, 1, 1, 0, 0, 0 ).build()
                .unwrap().julian_day ) / 36524.25;

            if self.julian_day < 
            Builder::from_gregorian_utc( 948, 1, 1, 0, 0, 0 ).build().unwrap().julian_day {
                ( 2177.0 + 497.0 * t + 44.1 * t * t ) / 86_400.0
            } else {
                // Year must not be in table or before 948
                ( 102.0 + 102.0 * t + 25.3 * t * t ) / 86_400.0
            }
        }
    }
}
#[cfg(test)]
mod astro_time_tests {
    use astro_time::*;

    #[test]
    fn test_modified_julian_day_number() {
        assert!( approx_eq(
            Builder::from_gregorian_utc( 1858, 11, 17, 0, 0, 0 )
                .build().unwrap().modified_julian_day_number(),
            0.0, 1.0e-15
        ));
    }

    #[test]
    fn test_to_gregorian_utc(){

        assert!( Builder::from_julian_date( 2_436_116.31 ).build().unwrap() 
            .to_gregorian_utc() == (1957, 10, 4, 19, 26, 24));
        
        assert!( Builder::from_julian_date( 2_451_545.0 ).build().unwrap() 
            .to_gregorian_utc() == (2000, 1, 1, 12, 0, 0));


        assert!( Builder::from_julian_date( 1_676_497.5 ).build().unwrap() 
            .to_gregorian_utc() == ( -123, 12, 29, 0, 0, 0 ));

        assert!( Builder::from_gregorian_utc( -123, 12, 29, 0, 0, 0 ).build().unwrap()
            .to_gregorian_utc() == ( -123, 12, 29, 0, 0, 0 ));

        assert!( Builder::from_gregorian_utc( -2300, 6, 12, 19, 23, 14 ).build().unwrap()
            .to_gregorian_utc() == ( -2300, 6, 12, 19, 23, 14 ));

        assert!( Builder::from_julian_date( 1_356_001.25 ).build().unwrap() 
            .to_gregorian_utc() == ( -1000, 7, 3, 18, 0, 0 ));

        assert!( Builder::from_julian_date( 1_356_001.0 ).build().unwrap() 
            .to_gregorian_utc() == ( -1000, 7, 3, 12, 0, 0 ));
    }

    #[test]
    fn test_as_utc() {
        let a_dt = Builder::from_gregorian_utc( 1977, 2, 18, 3, 37, 40 )
                    .dynamical_time().build().unwrap();
        let as_utc = a_dt.as_utc().unwrap();
        let as_utc2 = Builder::from_gregorian_utc( 1977, 2, 18, 3, 36, 52 ).build().unwrap();
        assert!( approx_eq( as_utc.julian_day_number(), 
            as_utc2.julian_day_number(), 1.0e-5));
    }

    #[test]
    fn test_as_dt() {
        let a_utc = Builder::from_gregorian_utc( 1977, 2, 18, 3, 36, 52 ).build().unwrap();
        let a_dt = Builder::from_gregorian_utc( 1977, 2, 18, 3, 37, 40 )
                    .dynamical_time().build().unwrap();
        let as_dt = a_utc.as_dt().unwrap();
        
        assert!( approx_eq( as_dt.julian_day_number(), 
            a_dt.julian_day_number(), 1.0e-5));
    }
}

///
/// Calculate JD0, or the Julian day number of January 0.0 for a given year. 
///
/// This is the same as December 31.0 for the previous year. 
///
/// Algorithm adapted from chapter 7, page 62 of Astronomical Algorithms, 2nd Edition
/// by Jean Meeus.
///
/// The argument year is in the Gregorian Calendar, and the return value is in Universal Time, not
/// Dynamical Time.
///
pub fn julian_day_zero( year: i32 ) -> AstroResult<AstroTime> {
    use std::f64;

    let y = (year - 1) as f64;
    let a = f64::floor( y / 100.0 );

    Builder::from_julian_date(f64::floor(365.25 * y) - a + f64::floor(a / 4.0) + 1_721_424.5).build()
}

/// Calculate the day of the year in the Gregorian Calendar
pub fn day_of_year_gregorian( year: i32, month: i32, day: i32 ) -> AstroResult<i32> {
    use std::f64;

    if is_valid_gregorian(year, month, day) {
        #[allow(non_snake_case)]
        let K: f64 = if is_gregorian_leap_year( year ) { 1.0 } else { 2.0 };

        Ok((f64::floor( ( month * 275 ) as f64 / 9.0 ) - 
            K * f64::floor(( month + 9) as f64 / 12.0 )) as i32 + day - 30)
        
    } else {
        Err( AstroAlgorithmsError::InvalidGregorianDate(year, month, day))
    }
}

/// Calculate the month and day in the Gregorian calendar from the year and
/// day of the year.
pub fn month_and_day_gregorian( year: i32, day_of_year: i32 ) -> AstroResult<(i32, i32)> {
    use std::f64;

    #[allow(non_snake_case)]
    let K: f64 = if is_gregorian_leap_year( year ) { 1.0 } else { 2.0 };
    #[allow(non_snake_case)]
    let mut M: i32 = f64::floor(( 9.0 * ( K + day_of_year as f64 ) / 275.0 ) +
        0.98 ) as i32;
    if day_of_year < 32 { M = 1; }

    #[allow(non_snake_case)]
    let D: i32 = ( day_of_year as f64 - f64::floor( 275.0 * M as f64 / 9.0 ) + 
        K as f64 * f64::floor(( M as f64 + 9.0 ) / 12.0) + 30.0 ) as i32;

    if is_valid_gregorian( year, M, D) {
        Ok((M,D))
    } else {
        Err( AstroAlgorithmsError::InvalidGregorianDate(year, M, D))
    }
}

/// Is this a leap year in the Gregorian calendar
pub fn is_gregorian_leap_year( year: i32 ) -> bool {
    if year % 4 != 0 { false }
    else if year % 100 != 0 { true }
    else if year % 400 == 0 { true }
    else { false }
}

/// Is this a leap year in the Julian calendar
#[inline]
pub fn is_julian_leap_year( year: i32 ) -> bool {
    year % 4 == 0
}

/// Validate a date given in the Gregorian calendar
pub fn is_valid_gregorian( year: i32, month: i32, day: i32 ) -> bool {
    if month < 1 || month > 12 || day < 1 || day > days_per_month_gregorian(month, year) { false }
    else { true }
}

/// Validate a date given in the Julian calendar
pub fn is_valid_julian( year: i32, month:i32, day: i32 ) -> bool {
    if month < 1 || month > 12 || day < 1 || day > days_per_month_julian(month, year) { false }
    else { true }
}

/// Validate a time.
pub fn is_valid_time( hour: i32, minute: i32, second: i32) -> bool {
    match hour {
        0...23 => (),
        _ => return false,
    }
    match minute {
        0...59 => (),
        _ => return false,
    }
    match second {
        0...59 => (),
        _ => return false,
    }
    true
}

// The days per month in the Gregorian calendar.
fn days_per_month_gregorian( month: i32, year: i32 ) -> i32 {

    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => { 
                if is_gregorian_leap_year( year ) { 29 }
                else { 28 }
             },
        // Should not be able to panic if dates were validated before using this function.
        _ => panic!("Invalid month.")
    }
}

// The days per month in the Julian calendar.
fn days_per_month_julian( month: i32, year: i32 ) -> i32 {

    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => { 
                if is_julian_leap_year( year ) { 29 }
                else { 28 }
             },
        // Should not be able to panic if dates were validated before using this function.
        _ => panic!("Invalid month.")
    }
}

// calculate the fraction of the day
fn day_fraction( hour: i32, minute: i32, second: i32 ) -> f64 {
    // Asserts should not be an issue if times were validated before calling this function.
    // Since this is private the module author controls validation before use.
    debug_assert!( hour >= 0 && hour < 24 );
    debug_assert!( minute >= 0 && minute < 60 );
    debug_assert!( second >= 0 && second < 60 );

    (hour as f64 + ( minute as f64 + second as f64 / 60.0 ) / 60.0 ) / 24.0
}

// given the fraction of a day, calculate the hour-minutes-seconds
fn to_hms( day_fraction: f64 ) -> (i32, i32, i32 ) {
    // Assert should not be an issue if times were validated before calling this function.
    // Since this is private the module author controls validation before use.
    debug_assert!( day_fraction < 1.0 );

    let mut remainder = day_fraction - f64::floor( day_fraction );
    let hour = f64::floor( remainder * 24.0 ) as i32;
    remainder -= hour as f64 / 24.0;
    let minute = f64::floor( remainder * 1_440.0 ) as i32;
    remainder -= minute as f64 / 1_440.0;
    let second = f64::floor( remainder * 86_400.0 + 0.5 ) as i32;

    ( hour, minute, second )
}

// test approximate equality, only used in unit tests.
#[cfg(test)]
fn approx_eq( left: f64, right: f64, tol: f64 ) -> bool {
    (left - right ).abs() < tol
}

// These tests work with functions not belonging to an impl
#[cfg(test)]
mod tests {
    use astro_time::*;

    #[test]
    fn test_julian_day_zero() {
        assert!(
            Builder::from_gregorian_utc( 2016, 12, 31, 0, 0, 0 ).build()
            .unwrap() == julian_day_zero( 2017 ).unwrap()
        );
    }

    #[test]
    fn test_day_of_year_gregorian() {
        assert!( day_of_year_gregorian( 1978, 11, 14 ).unwrap() == 318 );
        assert!( day_of_year_gregorian( 1988,  4, 22 ).unwrap() == 113 );

        if let AstroAlgorithmsError::InvalidGregorianDate(year, month, day) = 
        day_of_year_gregorian( 1988,  4, 31 ).unwrap_err() {
            assert!(year == 1988 && month == 4 && day == 31);
        } else {
            panic!("Wrong error type returned.");
        }
    }

    #[test]
    fn test_month_and_day_gregorian() {

        assert!( month_and_day_gregorian( 1978, 318 ).unwrap() == ( 11, 14 ));
        assert!( month_and_day_gregorian( 1988, 113 ).unwrap() == (  4, 22 ));
        assert!( month_and_day_gregorian( 1988, 366 ).unwrap() == ( 12, 31 ));

        if let AstroAlgorithmsError::InvalidGregorianDate(_, _, _) = 
        month_and_day_gregorian( 1989,  366 ).unwrap_err() { } else {
            panic!("Wrong error type returned.");
        }
    }

    #[test]
    fn test_is_gregorian_leap_year(){
        assert!( is_gregorian_leap_year( 1996 ));
        assert!( is_gregorian_leap_year( 2008 ));
        assert!( is_gregorian_leap_year( 2000 ));
        assert!( !is_gregorian_leap_year( 2100 ));
        assert!( !is_gregorian_leap_year( 2009 ));
        assert!( !is_gregorian_leap_year( 2010 ));
    }

    #[test]
    fn test_is_julian_leap_year(){
        assert!( is_julian_leap_year( 1996 ));
        assert!( is_julian_leap_year( 2008 ));
        assert!( is_julian_leap_year( 2000 ));
        assert!( is_julian_leap_year( 2100 ));
        assert!( !is_julian_leap_year( 2009 ));
        assert!( !is_julian_leap_year( 2010 ));
    }

    #[test]
    fn test_is_valid_gregorian() {
        assert!( is_valid_gregorian( 2017, 2, 15 ));
        assert!( is_valid_gregorian( 2017, 3, 31 ));
        assert!( is_valid_gregorian( 2017, 1, 31 ));
        assert!( is_valid_gregorian( 2017, 4, 30 ));
        assert!( is_valid_gregorian( 2017, 2, 28 ));
        assert!( !is_valid_gregorian( 2017, 2, 29 ));
        assert!( !is_valid_gregorian( 2017, 4, 31 ));
        assert!( !is_valid_gregorian( 2017, 13, 56 ));
        assert!( !is_valid_gregorian( 2017, 6, 40 ));
        assert!( is_valid_gregorian( 2012, 2, 29 ));
        assert!( !is_valid_gregorian(2000, 13, 5 ));
        assert!( !is_valid_gregorian(2000, 4, 31 ));
        assert!( !is_valid_gregorian(2000, 3, 32 ));
        assert!( !is_valid_gregorian(2000, 2, 30 ));
        assert!( !is_valid_gregorian(2100, 2, 29 ));
        assert!( is_valid_gregorian(2000, 2, 29 ));
        assert!( is_valid_gregorian(2000, 1, 15 ));
    }

    #[test]
    fn test_is_valid_julian() {
        assert!( is_valid_julian( 2017, 2, 15 ));
        assert!( is_valid_julian( 2017, 3, 31 ));
        assert!( is_valid_julian( 2017, 1, 31 ));
        assert!( is_valid_julian( 2017, 4, 30 ));
        assert!( is_valid_julian( 2017, 2, 28 ));
        assert!( !is_valid_julian( 2017, 2, 29 ));
        assert!( !is_valid_julian( 2017, 4, 31 ));
        assert!( !is_valid_julian( 2017, 13, 56 ));
        assert!( !is_valid_julian( 2017, 6, 40 ));
        assert!( is_valid_julian( 2012, 2, 29 ));
        assert!( !is_valid_julian(2000, 13, 5 ));
        assert!( !is_valid_julian(2000, 4, 31 ));
        assert!( !is_valid_julian(2000, 3, 32 ));
        assert!( !is_valid_julian(2000, 2, 30 ));
        assert!( is_valid_julian(2100, 2, 29 ));
        assert!( is_valid_julian(2000, 2, 29 ));
        assert!( is_valid_julian(2000, 1, 15 ));
    }

    #[test]
    fn test_is_valid_time() {
        for hour in 0..24 {
            for minute in 0..60 {
                for second in 0..60 {
                    assert!(is_valid_time(hour, minute, second));
                }
            }
        }

        assert!(!is_valid_time(-4,1,1));
        assert!(!is_valid_time(4,-1,1));
        assert!(!is_valid_time(4,1,-1));
        assert!(!is_valid_time(24,1,1));
        assert!(!is_valid_time(4,60,1));
        assert!(!is_valid_time(4,1,60));
    }
}
