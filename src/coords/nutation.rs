//!
//! Adjustments to right ascension and declination due to nutation.
//!
//! Authors: Ryan Leach
//!
//! Copyright: Ryan Leach, 2017
//!
//! License: [BSD 3-clause](https://opensource.org/licenses/BSD-3-Clause)
//!

use std::fmt;

use super::super::angles::{RadianAngle, HMSAngle, DMSAngle};
use super::super::astro_time::AstroTime;

/// Data relating to nutation for a given date.
#[derive(Debug, Clone, Copy)]
pub struct Nutation {
    delta_lon: RadianAngle,
    delta_obl: RadianAngle,
    obliquity_ec: RadianAngle,
    epoch: AstroTime,
}

impl fmt::Display for Nutation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Nutation { delta_lon: dl, delta_obl: dob, obliquity_ec: obliq, epoch: e } = *self;

        write!(f,
               "Nutation for {}\n  \u{0394}\u{03C8}: {}\n  \
                \u{0394}\u{03B5}: {}\n  and \u{03B5}\u{2080}: {}\n",
               e,
               HMSAngle::from(dl),
               DMSAngle::from(dob),
               DMSAngle::from(obliq))
    }
}
