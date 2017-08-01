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

use super::super::angles::{RadianAngle, DMSAngle, Angle};
use super::super::astro_time::AstroTime;
use super::super::error::*;

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
                \u{0394}\u{03B5}: {}\n  \u{03B5}\u{2080}: {}\n  \u{03B5}: {}\n",
               e,
               DMSAngle::from(dl).map_to_longitude_range(),
               DMSAngle::from(dob).map_to_latitude_range().unwrap(),
               DMSAngle::from(obliq).map_to_latitude_range().unwrap(),
               DMSAngle::from(obliq + dob).map_to_latitude_range().unwrap())
    }
}

/// Calculate nutation effects for a given date.
pub fn calculate_nutation_data_for_date(epoch: AstroTime) -> AstroResult<Nutation> {
    // Chapter 22 of Meeus
    #[allow(non_snake_case)]
    let T = epoch.as_dt()?;
    #[allow(non_snake_case)]
    let T = (T.julian_day_number() - 2_451_545.0) / 36525.0;

    #[allow(non_snake_case)]
    let D = 297.85036 + T * (445_267.111_480 + T * (-0.001_914_2 + T / 189_474.0));
    #[allow(non_snake_case)]
    let M = 357.527_72 + T * (35_999.050_340 + T * (-0.000_160_3 - T / 300_000.0));
    #[allow(non_snake_case)]
    let Mprm = 134.962_98 + T * (477_198.867_398 + T * (-0.008_697_2 + T / 56_250.0));
    #[allow(non_snake_case)]
    let F = 93.271_91 + T * (483_202.017_538 + T * (-0.003_682_5 + T / 327_270.0));
    let omega = 125.044_52 + T * (-1_934.136_261 + T * (0.002_070_8 + T / 450_000.0));

    // Pre-calculate some coefficients
    #[allow(non_snake_case)]
    let m2D = -2.0 * D;
    #[allow(non_snake_case)]
    let p2D = 2.0 * D;
    #[allow(non_snake_case)]
    let m1D = -D;
    #[allow(non_snake_case)]
    let p2M = 2.0 * M;
    #[allow(non_snake_case)]
    let m1M = -M;
    #[allow(non_snake_case)]
    let m1Mprm = -Mprm;
    #[allow(non_snake_case)]
    let p2Mprm = 2.0 * Mprm;
    #[allow(non_snake_case)]
    let m2Mprm = -2.0 * Mprm;
    #[allow(non_snake_case)]
    let p3Mprm = 3.0 * Mprm;
    #[allow(non_snake_case)]
    let p2F = 2.0 * F;
    #[allow(non_snake_case)]
    let m2F = -2.0 * F;
    #[allow(non_snake_case)]
    let p2omega = 2.0 * omega;

    #[cfg_attr(rustfmt, rustfmt_skip)]
    let periodic_terms_for_nutation = [
      //(m2D + p2M + m2Mprm + p2F + p2omega, -171_996.0 - 174.2 * T, 92_025.0 + 8.9 * T), // demo
        (                             omega, -171_996.0 - 174.2 * T, 92_025.0 + 8.9 * T), // row  1
        (m2D                + p2F + p2omega,  -13_187.0   - 1.6 * T,  5_736.0 - 3.1 * T), // row  2
        (                     p2F + p2omega,   -2_274.0   - 0.2 * T,    977.0 - 0.5 * T), // row  3
        (                           p2omega,    2_062.0   + 0.2 * T,   -895.0 + 0.5 * T), // row  4
        (        M                         ,    1_426.0   - 3.4 * T,     54.0 - 0.1 * T), // row  5
        (              Mprm                ,      712.0   + 0.1 * T,     -7.0          ), // row  6
        (m2D +   M          + p2F + p2omega,     -517.0   + 1.2 * T,    224.0 - 0.6 * T), // row  7
        (                     p2F +   omega,     -386.0   - 0.4 * T,    200.0          ), // row  8
        (              Mprm + p2F + p2omega,     -301.0            ,    129.0 - 0.1 * T), // row  9
        (m2D + m1M          + p2F + p2omega,      217.0   - 0.5 * T,    -95.0 + 0.3 * T), // row 10
        (m2D       +   Mprm                ,     -158.0            ,      0.0          ), // row 11
        (m2D                + p2F +   omega,      129.0   + 0.1 * T,    -70.0          ), // row 12
        (            m1Mprm + p2F + p2omega,      123.0            ,    -53.0          ), // row 13
        (p2D                               ,       63.0            ,      0.0          ), // row 14
        (             Mprm        +   omega,       63.0   + 0.1 * T,    -33.0          ), // row 15
        (p2D       + m1Mprm + p2F + p2omega,      -59.0            ,     26.0          ), // row 16
        (            m1Mprm       +   omega,      -58.0   - 0.1 * T,     32.0          ), // row 17
        (              Mprm + p2F +   omega,      -51.0            ,     27.0          ), // row 18
        (m2D       + p2Mprm                ,       48.0            ,      0.0          ), // row 19
        (            m2Mprm + p2F +   omega,       46.0            ,    -24.0          ), // row 20
        (p2D                + p2F + p2omega,      -38.0            ,     16.0          ), // row 21
        (            p2Mprm + p2F + p2omega,      -31.0            ,     13.0          ), // row 22
        (            p2Mprm                ,       29.0            ,      0.0          ), // row 23
        (m2D       +   Mprm + p2F + p2omega,       29.0            ,    -12.0          ), // row 24
        (                     p2F          ,       26.0            ,      0.0          ), // row 25
        (m2D                + p2F          ,      -22.0            ,      0.0          ), // row 26
        (            m1Mprm + p2F +   omega,       21.0            ,    -10.0          ), // row 27
        (      p2M                         ,       17.0   - 0.1 * T,      0.0          ), // row 28
        (p2D       + m1Mprm       +   omega,       16.0            ,     -8.0          ), // row 29
        (m2D + p2M          + p2F + p2omega,      -16.0   + 0.1 * T,      7.0          ), // row 30
        (        M                +   omega,      -15.0            ,      9.0          ), // row 31
      //-------------------------------------------------------------------------------------------
        (m2D       +   Mprm       +   omega,      -13.0            ,      7.0          ), // row 32
        (      m1M                +   omega,      -12.0            ,      6.0          ), // row 33
        (            p2Mprm + m2F          ,       11.0            ,      0.0          ), // row 34
        (p2D       + m1Mprm + p2F +   omega,      -10.0            ,      5.0          ), // row 35
        (p2D       +   Mprm + p2F + p2omega,       -8.0            ,      3.0          ), // row 36
        (        M          + p2F + p2omega,        7.0            ,     -3.0          ), // row 37
        (m2D +   M +   Mprm                ,       -7.0            ,      0.0          ), // row 38
        (      m1M          + p2F + p2omega,       -7.0            ,      3.0          ), // row 39
        (p2D                + p2F +   omega,       -7.0            ,      3.0          ), // row 40
        (p2D       +   Mprm                ,        6.0            ,      0.0          ), // row 41
        (m2D       + p2Mprm + p2F + p2omega,        6.0            ,     -3.0          ), // row 42
        (m2D       +   Mprm + p2F +   omega,        6.0            ,     -3.0          ), // row 43
        (p2D       + m2Mprm       +   omega,       -6.0            ,      3.0          ), // row 44
        (p2D                      +   omega,       -6.0            ,      3.0          ), // row 45
        (      m1M +   Mprm                ,        5.0            ,      0.0          ), // row 46
        (m2D + m1M          + p2F +   omega,       -5.0            ,      3.0          ), // row 47
        (m2D                      +   omega,       -5.0            ,      3.0          ), // row 48
        (            p2Mprm + p2F +   omega,       -5.0            ,      3.0          ), // row 49
        (m2D       + p2Mprm       +   omega,        4.0            ,      0.0          ), // row 50
        (m2D +   M          + p2F +   omega,        4.0            ,      0.0          ), // row 51
        (              Mprm + m2F          ,        4.0            ,      0.0          ), // row 52
        (m1D       +   Mprm                ,       -4.0            ,      0.0          ), // row 53
        (m2D +   M                         ,       -4.0            ,      0.0          ), // row 54
        (  D                               ,       -4.0            ,      0.0          ), // row 55
        (              Mprm + p2F          ,        3.0            ,      0.0          ), // row 56
        (            m2Mprm + p2F + p2omega,       -3.0            ,      0.0          ), // row 57
        (m1D + m1M +   Mprm                ,       -3.0            ,      0.0          ), // row 58
        (        M +   Mprm                ,       -3.0            ,      0.0          ), // row 59
        (      m1M +   Mprm + p2F + p2omega,       -3.0            ,      0.0          ), // row 60
        (p2D + m1M + m1Mprm + p2F + p2omega,       -3.0            ,      0.0          ), // row 61
        (            p3Mprm + p2F + p2omega,       -3.0            ,      0.0          ), // row 62
        (p2D + m1M          + p2F + p2omega,       -3.0            ,      0.0          ), // row 63
    ];

    let mut delta_psi = 0.0;
    let mut delta_eps = 0.0;
    for &(arg, coeff_sin, coeff_cos) in periodic_terms_for_nutation.iter() {
        delta_psi += coeff_sin * arg.to_radians().sin();
        delta_eps += coeff_cos * arg.to_radians().cos();
    }

    // Convert from units of 0.0001" to units of 1"
    delta_psi /= 10_000.0;
    delta_eps /= 10_000.0;

    let delta_psi = RadianAngle::from(DMSAngle::new(0, 0, delta_psi));
    let delta_eps = RadianAngle::from(DMSAngle::new(0, 0, delta_eps));

    // Calculate eps0
    #[allow(non_snake_case)]
    let U = T / 100.0;
    let eps_cor =
        U *
        (-4_680.93 +
         U *
         (-1.55 +
          U *
          (1999.25 +
           U *
           (-51.38 +
            U * (-249.67 + U * (-39.05 + U * (7.12 + U * (27.87 + U * (5.79 + U * 2.45)))))))));
    let eps0 = RadianAngle::from(DMSAngle::new(23, 26, 21.448) + DMSAngle::new(0, 0, eps_cor));

    Ok(Nutation {
           delta_lon: delta_psi,
           delta_obl: delta_eps,
           obliquity_ec: eps0,
           epoch: epoch,
       })
}

// Apply Nutation

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::test_util::*;
    use super::super::super::astro_time::*;

    #[test]
    fn test_calculate_nutation_data_for_date() {

        // From exampe on pg 148 of Meeus.
        let valid_time =
            Builder::from_gregorian_utc(1987, 4, 10, 0, 0, 0).dynamical_time().build().unwrap();

        let nutation = calculate_nutation_data_for_date(valid_time).unwrap();

        println!("Nutation: {}", nutation);

        let Nutation { delta_lon, delta_obl, obliquity_ec, epoch: _ } = nutation;
        let delta_lon = DMSAngle::from(delta_lon.map_to_longitude_range()).seconds();
        assert!(approx_eq(delta_lon, -3.788, 1.0e-3));

        let delta_obl = DMSAngle::from(delta_obl.map_to_latitude_range().unwrap()).seconds();
        assert!(approx_eq(delta_obl, 9.443, 1.0e-3));

        let obliquity_ec = DMSAngle::from(obliquity_ec.map_to_latitude_range().unwrap());
        assert!(obliquity_ec.degrees() == 23 && obliquity_ec.minutes() == 26 &&
                approx_eq(obliquity_ec.seconds(), 27.407, 1.0e-3));
    }
}
