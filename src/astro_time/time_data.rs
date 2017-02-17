//!
//! Module contains a list of dates and delta-t values, which are converted to
//! Julian Days and delta-t values for converting between UTC and dynamical
//! time.
//!
//! Authors: Ryan Leach
//!
//! Copyright: Ryan Leach, 2017
//!
//! License: [BSD 3-clause](https://opensource.org/licenses/BSD-3-Clause)
//!

use super::AstroTmBldr;

lazy_static! {
    pub static ref TIME_DELTA: Vec<(f64,f64)> = { 

        let time_delta_date_list = vec![
            // Values copied from Table 10.A on page 79 of Astronomical Algorithms, 
            // 2nd Edition by Jean Meeus.
            (1620, 1, 1, 121.0),
            (1622, 1, 1, 112.0),
            (1624, 1, 1, 103.0),
            (1626, 1, 1,  95.0), 
            (1628, 1, 1,  88.0),
            (1630, 1, 1,  82.0),
            (1632, 1, 1,  77.0), 
            (1634, 1, 1,  72.0),
            (1636, 1, 1,  68.0), 
            (1638, 1, 1,  63.0),
            (1640, 1, 1,  60.0),
            (1642, 1, 1,  56.0),
            (1644, 1, 1,  53.0),
            (1646, 1, 1,  51.0),
            (1648, 1, 1,  48.0),
            (1650, 1, 1,  46.0),
            (1652, 1, 1,  44.0),
            (1654, 1, 1,  42.0),
            (1656, 1, 1,  40.0),
            (1658, 1, 1,  38.0),
            (1660, 1, 1,  35.0),
            (1662, 1, 1,  33.0),
            (1664, 1, 1,  31.0),
            (1666, 1, 1,  29.0),
            (1668, 1, 1,  26.0),
            (1670, 1, 1,  24.0),
            (1672, 1, 1,  22.0),
            (1674, 1, 1,  20.0),
            (1676, 1, 1,  18.0),
            (1678, 1, 1,  16.0),
            (1680, 1, 1,  14.0),
            (1682, 1, 1,  12.0),
            (1684, 1, 1,  11.0),
            (1686, 1, 1,  10.0),
            (1688, 1, 1,   9.0),
            (1690, 1, 1,   8.0),
            (1692, 1, 1,   7.0),
            (1694, 1, 1,   7.0),
            (1696, 1, 1,   7.0),
            (1698, 1, 1,   7.0),
            (1700, 1, 1,   7.0),
            (1702, 1, 1,   7.0),
            (1704, 1, 1,   8.0),
            (1706, 1, 1,   8.0), 
            (1708, 1, 1,   9.0),
            (1710, 1, 1,   9.0),
            (1712, 1, 1,   9.0), 
            (1714, 1, 1,   9.0),
            (1716, 1, 1,   9.0), 
            (1718, 1, 1,  10.0),
            (1720, 1, 1,  10.0),
            (1722, 1, 1,  10.0),
            (1724, 1, 1,  10.0),
            (1726, 1, 1,  10.0),
            (1728, 1, 1,  10.0),
            (1730, 1, 1,  10.0),
            (1732, 1, 1,  10.0),
            (1734, 1, 1,  11.0),
            (1736, 1, 1,  11.0),
            (1738, 1, 1,  11.0),
            (1740, 1, 1,  11.0),
            (1742, 1, 1,  11.0),
            (1744, 1, 1,  12.0),
            (1746, 1, 1,  12.0),
            (1748, 1, 1,  12.0),
            (1750, 1, 1,  12.0),
            (1752, 1, 1,  13.0),
            (1754, 1, 1,  13.0),
            (1756, 1, 1,  13.0),
            (1758, 1, 1,  14.0),
            (1760, 1, 1,  14.0),
            (1762, 1, 1,  14.0),
            (1764, 1, 1,  14.0),
            (1766, 1, 1,  15.0),
            (1768, 1, 1,  15.0),
            (1770, 1, 1,  15.0),
            (1772, 1, 1,  15.0),
            (1774, 1, 1,  15.0),
            (1776, 1, 1,  16.0),
            (1778, 1, 1,  16.0),
            (1780, 1, 1,  16.0),
            (1782, 1, 1,  16.0),
            (1784, 1, 1,  16.0),
            (1786, 1, 1,  16.0), 
            (1788, 1, 1,  16.0),
            (1790, 1, 1,  16.0),
            (1792, 1, 1,  15.0), 
            (1794, 1, 1,  15.0),
            (1796, 1, 1,  14.0), 
            (1798, 1, 1,  13.0),
            (1800, 1, 1,  13.1),
            (1802, 1, 1,  12.5),
            (1804, 1, 1,  12.2),
            (1806, 1, 1,  12.0),
            (1808, 1, 1,  12.0),
            (1810, 1, 1,  12.0),
            (1812, 1, 1,  12.0),
            (1814, 1, 1,  12.0),
            (1816, 1, 1,  12.0),
            (1818, 1, 1,  11.9),
            (1820, 1, 1,  11.6),
            (1822, 1, 1,  11.0),
            (1824, 1, 1,  10.2),
            (1826, 1, 1,   9.2),
            (1828, 1, 1,   8.2),
            (1830, 1, 1,   7.1),
            (1832, 1, 1,   6.2),
            (1834, 1, 1,   5.6),
            (1836, 1, 1,   5.4),
            (1838, 1, 1,   5.3),
            (1840, 1, 1,   5.4),
            (1842, 1, 1,   5.6),
            (1844, 1, 1,   5.9),
            (1846, 1, 1,   6.2),
            (1848, 1, 1,   6.5),
            (1850, 1, 1,   6.8),
            (1852, 1, 1,   7.1),
            (1854, 1, 1,   7.3),
            (1856, 1, 1,   7.5),
            (1858, 1, 1,   7.6),
            (1860, 1, 1,   7.7),
            (1862, 1, 1,   7.3),
            (1864, 1, 1,   6.2),
            (1866, 1, 1,   5.2), 
            (1868, 1, 1,   2.7),
            (1870, 1, 1,   1.4),
            (1872, 1, 1,  -1.2), 
            (1874, 1, 1,  -2.8),
            (1876, 1, 1,  -3.8), 
            (1878, 1, 1,  -4.8),
            (1880, 1, 1,  -5.5),
            (1882, 1, 1,  -5.3),
            (1884, 1, 1,  -5.6),
            (1886, 1, 1,  -5.7),
            (1888, 1, 1,  -5.9),
            (1890, 1, 1,  -6.0),
            (1892, 1, 1,  -6.3),
            (1894, 1, 1,  -6.5),
            (1896, 1, 1,  -6.2),
            (1898, 1, 1,  -4.7),
            (1900, 1, 1,  -2.8),
            (1902, 1, 1,  -0.1),
            (1904, 1, 1,   2.6),
            (1906, 1, 1,   5.3),
            (1908, 1, 1,   7.7),
            (1910, 1, 1,  10.4),
            (1912, 1, 1,  13.3),
            (1914, 1, 1,  16.0),
            (1916, 1, 1,  18.2),
            (1918, 1, 1,  20.2),
            (1920, 1, 1,  21.1),
            (1922, 1, 1,  22.4),
            (1924, 1, 1,  23.5),
            (1926, 1, 1,  23.8),
            (1928, 1, 1,  24.3),
            (1930, 1, 1,  24.0),
            (1932, 1, 1,  23.9),
            (1934, 1, 1,  23.9),
            (1936, 1, 1,  23.7),
            (1938, 1, 1,  24.0),
            (1940, 1, 1,  24.3),
            (1942, 1, 1,  25.3),
            (1944, 1, 1,  26.2),
            (1946, 1, 1,  27.3), 
            (1948, 1, 1,  28.2),
            (1950, 1, 1,  29.1),
            (1952, 1, 1,  30.0), 
            (1954, 1, 1,  30.7),
            (1956, 1, 1,  31.4), 
            (1958, 1, 1,  32.2),
            (1960, 1, 1,  33.1),
            (1962, 1, 1,  34.0),
            (1964, 1, 1,  35.0),
            (1966, 1, 1,  36.5),
            (1968, 1, 1,  38.3),
            (1970, 1, 1,  40.2),
            (1972, 1, 1,  42.2),
            // Comment out these values because they are covered by USN data.
            // (1974, 1, 1,  44.5),
            // (1976, 1, 1,  46.5),
            // (1978, 1, 1,  48.5),
            // (1980, 1, 1,  50.5),
            // (1982, 1, 1,  52.2),
            // (1984, 1, 1,  53.8),
            // (1986, 1, 1,  54.9),
            // (1988, 1, 1,  55.8),
            // (1990, 1, 1,  56.9),
            // (1992, 1, 1,  58.3),
            // (1994, 1, 1,  60.0),
            // (1996, 1, 1,  61.6),
            // (1998, 1, 1,  63.0),
            // (2000, 1, 1,  63.8),
            // (2002, 1, 1,  64.3),
            // (2004, 1, 1,  64.6),
            // (2006, 1, 1,  64.8),
            // (2008, 1, 1,  65.5),
            // (2010, 1, 1,  66.1),
            //
            // Values downloaded from the US Navy
            // http://www.usno.navy.mil/USNO/earth-orientation/eo-products/long-term
            (1973,  2,  1,  43.4724),
            (1973,  3,  1,  43.5648),
            (1973,  4,  1,  43.6737),
            (1973,  5,  1,  43.7782),
            (1973,  6,  1,  43.8763),
            (1973,  7,  1,  43.9562),
            (1973,  8,  1,  44.0315),
            (1973,  9,  1,  44.1132),
            (1973, 10,  1,  44.1982),
            (1973, 11,  1,  44.2952),
            (1973, 12,  1,  44.3936),
            (1974,  1,  1,  44.4841),
            (1974,  2,  1,  44.5646),
            (1974,  3,  1,  44.6425),
            (1974,  4,  1,  44.7386),
            (1974,  5,  1,  44.8370),
            (1974,  6,  1,  44.9302),
            (1974,  7,  1,  44.9986),
            (1974,  8,  1,  45.0584),
            (1974,  9,  1,  45.1284),
            (1974, 10,  1,  45.2064),
            (1974, 11,  1,  45.2980),
            (1974, 12,  1,  45.3897),
            (1975,  1,  1,  45.4761),
            (1975,  2,  1,  45.5633),
            (1975,  3,  1,  45.6450),
            (1975,  4,  1,  45.7375),
            (1975,  5,  1,  45.8284),
            (1975,  6,  1,  45.9133),
            (1975,  7,  1,  45.9820),
            (1975,  8,  1,  46.0408),
            (1975,  9,  1,  46.1067),
            (1975, 10,  1,  46.1825),
            (1975, 11,  1,  46.2789),
            (1975, 12,  1,  46.3713),
            (1976,  1,  1,  46.4567),
            (1976,  2,  1,  46.5445),
            (1976,  3,  1,  46.6311),
            (1976,  4,  1,  46.7302),
            (1976,  5,  1,  46.8284),
            (1976,  6,  1,  46.9247),
            (1976,  7,  1,  46.9970),
            (1976,  8,  1,  47.0709),
            (1976,  9,  1,  47.1451),
            (1976, 10,  1,  47.2362),
            (1976, 11,  1,  47.3413),
            (1976, 12,  1,  47.4319),
            (1977,  1,  1,  47.5214),
            (1977,  2,  1,  47.6049),
            (1977,  3,  1,  47.6837),
            (1977,  4,  1,  47.7781),
            (1977,  5,  1,  47.8771),
            (1977,  6,  1,  47.9687),
            (1977,  7,  1,  48.0348),
            (1977,  8,  1,  48.0942),
            (1977,  9,  1,  48.1608),
            (1977, 10,  1,  48.2460),
            (1977, 11,  1,  48.3439),
            (1977, 12,  1,  48.4355),
            (1978,  1,  1,  48.5344),
            (1978,  2,  1,  48.6325),
            (1978,  3,  1,  48.7294),
            (1978,  4,  1,  48.8365),
            (1978,  5,  1,  48.9353),
            (1978,  6,  1,  49.0319),
            (1978,  7,  1,  49.1013),
            (1978,  8,  1,  49.1591),
            (1978,  9,  1,  49.2286),
            (1978, 10,  1,  49.3070),
            (1978, 11,  1,  49.4018),
            (1978, 12,  1,  49.4945),
            (1979,  1,  1,  49.5862),
            (1979,  2,  1,  49.6805),
            (1979,  3,  1,  49.7602),
            (1979,  4,  1,  49.8556),
            (1979,  5,  1,  49.9489),
            (1979,  6,  1,  50.0347),
            (1979,  7,  1,  50.1019),
            (1979,  8,  1,  50.1622),
            (1979,  9,  1,  50.2260),
            (1979, 10,  1,  50.2968),
            (1979, 11,  1,  50.3831),
            (1979, 12,  1,  50.4599),
            (1980,  1,  1,  50.5387),
            (1980,  2,  1,  50.6161),
            (1980,  3,  1,  50.6866),
            (1980,  4,  1,  50.7658),
            (1980,  5,  1,  50.8454),
            (1980,  6,  1,  50.9187),
            (1980,  7,  1,  50.9761),
            (1980,  8,  1,  51.0278),
            (1980,  9,  1,  51.0843),
            (1980, 10,  1,  51.1538),
            (1980, 11,  1,  51.2319),
            (1980, 12,  1,  51.3063),
            (1981,  1,  1,  51.3808),
            (1981,  2,  1,  51.4526),
            (1981,  3,  1,  51.5160),
            (1981,  4,  1,  51.5985),
            (1981,  5,  1,  51.6809),
            (1981,  6,  1,  51.7573),
            (1981,  7,  1,  51.8133),
            (1981,  8,  1,  51.8532),
            (1981,  9,  1,  51.9014),
            (1981, 10,  1,  51.9603),
            (1981, 11,  1,  52.0328),
            (1981, 12,  1,  52.0985),
            (1982,  1,  1,  52.1668),
            (1982,  2,  1,  52.2316),
            (1982,  3,  1,  52.2938),
            (1982,  4,  1,  52.3680),
            (1982,  5,  1,  52.4465),
            (1982,  6,  1,  52.5180),
            (1982,  7,  1,  52.5752),
            (1982,  8,  1,  52.6178),
            (1982,  9,  1,  52.6668),
            (1982, 10,  1,  52.7340),
            (1982, 11,  1,  52.8056),
            (1982, 12,  1,  52.8792),
            (1983,  1,  1,  52.9565),
            (1983,  2,  1,  53.0445),
            (1983,  3,  1,  53.1268),
            (1983,  4,  1,  53.2197),
            (1983,  5,  1,  53.3024),
            (1983,  6,  1,  53.3747),
            (1983,  7,  1,  53.4335),
            (1983,  8,  1,  53.4778),
            (1983,  9,  1,  53.5300),
            (1983, 10,  1,  53.5845),
            (1983, 11,  1,  53.6523),
            (1983, 12,  1,  53.7256),
            (1984,  1,  1,  53.7882),
            (1984,  2,  1,  53.8367),
            (1984,  3,  1,  53.8830),
            (1984,  4,  1,  53.9443),
            (1984,  5,  1,  54.0042),
            (1984,  6,  1,  54.0536),
            (1984,  7,  1,  54.0856),
            (1984,  8,  1,  54.1084),
            (1984,  9,  1,  54.1463),
            (1984, 10,  1,  54.1914),
            (1984, 11,  1,  54.2452),
            (1984, 12,  1,  54.2958),
            (1985,  1,  1,  54.3427),
            (1985,  2,  1,  54.3911),
            (1985,  3,  1,  54.4320),
            (1985,  4,  1,  54.4898),
            (1985,  5,  1,  54.5456),
            (1985,  6,  1,  54.5977),
            (1985,  7,  1,  54.6355),
            (1985,  8,  1,  54.6532),
            (1985,  9,  1,  54.6776),
            (1985, 10,  1,  54.7174),
            (1985, 11,  1,  54.7741),
            (1985, 12,  1,  54.8253),
            (1986,  1,  1,  54.8713),
            (1986,  2,  1,  54.9161),
            (1986,  3,  1,  54.9581),
            (1986,  4,  1,  54.9997),
            (1986,  5,  1,  55.0476),
            (1986,  6,  1,  55.0912),
            (1986,  7,  1,  55.1132),
            (1986,  8,  1,  55.1328),
            (1986,  9,  1,  55.1532),
            (1986, 10,  1,  55.1898),
            (1986, 11,  1,  55.2416),
            (1986, 12,  1,  55.2838),
            (1987,  1,  1,  55.3222),
            (1987,  2,  1,  55.3613),
            (1987,  3,  1,  55.4063),
            (1987,  4,  1,  55.4629),
            (1987,  5,  1,  55.5111),
            (1987,  6,  1,  55.5524),
            (1987,  7,  1,  55.5812),
            (1987,  8,  1,  55.6004),
            (1987,  9,  1,  55.6262),
            (1987, 10,  1,  55.6656),
            (1987, 11,  1,  55.7168),
            (1987, 12,  1,  55.7698),
            (1988,  1,  1,  55.8197),
            (1988,  2,  1,  55.8615),
            (1988,  3,  1,  55.9130),
            (1988,  4,  1,  55.9663),
            (1988,  5,  1,  56.0220),
            (1988,  6,  1,  56.0700),
            (1988,  7,  1,  56.0939),
            (1988,  8,  1,  56.1105),
            (1988,  9,  1,  56.1314),
            (1988, 10,  1,  56.1611),
            (1988, 11,  1,  56.2068),
            (1988, 12,  1,  56.2583),
            (1989,  1,  1,  56.3000),
            (1989,  2,  1,  56.3399),
            (1989,  3,  1,  56.3790),
            (1989,  4,  1,  56.4283),
            (1989,  5,  1,  56.4804),
            (1989,  6,  1,  56.5352),
            (1989,  7,  1,  56.5697),
            (1989,  8,  1,  56.5983),
            (1989,  9,  1,  56.6328),
            (1989, 10,  1,  56.6739),
            (1989, 11,  1,  56.7332),
            (1989, 12,  1,  56.7972),
            (1990,  1,  1,  56.8553),
            (1990,  2,  1,  56.9111),
            (1990,  3,  1,  56.9755),
            (1990,  4,  1,  57.0471),
            (1990,  5,  1,  57.1136),
            (1990,  6,  1,  57.1738),
            (1990,  7,  1,  57.2226),
            (1990,  8,  1,  57.2597),
            (1990,  9,  1,  57.3073),
            (1990, 10,  1,  57.3643),
            (1990, 11,  1,  57.4334),
            (1990, 12,  1,  57.5016),
            (1991,  1,  1,  57.5653),
            (1991,  2,  1,  57.6333),
            (1991,  3,  1,  57.6973),
            (1991,  4,  1,  57.7711),
            (1991,  5,  1,  57.8407),
            (1991,  6,  1,  57.9058),
            (1991,  7,  1,  57.9576),
            (1991,  8,  1,  57.9975),
            (1991,  9,  1,  58.0426),
            (1991, 10,  1,  58.1043),
            (1991, 11,  1,  58.1679),
            (1991, 12,  1,  58.2389),
            (1992,  1,  1,  58.3092),
            (1992,  2,  1,  58.3833),
            (1992,  3,  1,  58.4537),
            (1992,  4,  1,  58.5401),
            (1992,  5,  1,  58.6228),
            (1992,  6,  1,  58.6917),
            (1992,  7,  1,  58.7410),
            (1992,  8,  1,  58.7836),
            (1992,  9,  1,  58.8406),
            (1992, 10,  1,  58.8986),
            (1992, 11,  1,  58.9714),
            (1992, 12,  1,  59.0438),
            (1993,  1,  1,  59.1218),
            (1993,  2,  1,  59.2003),
            (1993,  3,  1,  59.2747),
            (1993,  4,  1,  59.3574),
            (1993,  5,  1,  59.4434),
            (1993,  6,  1,  59.5242),
            (1993,  7,  1,  59.5850),
            (1993,  8,  1,  59.6344),
            (1993,  9,  1,  59.6928),
            (1993, 10,  1,  59.7588),
            (1993, 11,  1,  59.8386),
            (1993, 12,  1,  59.9111),
            (1994,  1,  1,  59.9845),
            (1994,  2,  1,  60.0564),
            (1994,  3,  1,  60.1231),
            (1994,  4,  1,  60.2042),
            (1994,  5,  1,  60.2804),
            (1994,  6,  1,  60.3530),
            (1994,  7,  1,  60.4012),
            (1994,  8,  1,  60.4440),
            (1994,  9,  1,  60.4900),
            (1994, 10,  1,  60.5578),
            (1994, 11,  1,  60.6324),
            (1994, 12,  1,  60.7059),
            (1995,  1,  1,  60.7853),
            (1995,  2,  1,  60.8664),
            (1995,  3,  1,  60.9387),
            (1995,  4,  1,  61.0277),
            (1995,  5,  1,  61.1103),
            (1995,  6,  1,  61.1870),
            (1995,  7,  1,  61.2454),
            (1995,  8,  1,  61.2881),
            (1995,  9,  1,  61.3378),
            (1995, 10,  1,  61.4036),
            (1995, 11,  1,  61.4760),
            (1995, 12,  1,  61.5525),
            (1996,  1,  1,  61.6287),
            (1996,  2,  1,  61.6846),
            (1996,  3,  1,  61.7433),
            (1996,  4,  1,  61.8132),
            (1996,  5,  1,  61.8823),
            (1996,  6,  1,  61.9497),
            (1996,  7,  1,  61.9969),
            (1996,  8,  1,  62.0343),
            (1996,  9,  1,  62.0714),
            (1996, 10,  1,  62.1202),
            (1996, 11,  1,  62.1810),
            (1996, 12,  1,  62.2382),
            (1997,  1,  1,  62.2950),
            (1997,  2,  1,  62.3506),
            (1997,  3,  1,  62.3995),
            (1997,  4,  1,  62.4754),
            (1997,  5,  1,  62.5463),
            (1997,  6,  1,  62.6136),
            (1997,  7,  1,  62.6571),
            (1997,  8,  1,  62.6942),
            (1997,  9,  1,  62.7383),
            (1997, 10,  1,  62.7926),
            (1997, 11,  1,  62.8567),
            (1997, 12,  1,  62.9146),
            (1998,  1,  1,  62.9659),
            (1998,  2,  1,  63.0217),
            (1998,  3,  1,  63.0807),
            (1998,  4,  1,  63.1462),
            (1998,  5,  1,  63.2053),
            (1998,  6,  1,  63.2599),
            (1998,  7,  1,  63.2844),
            (1998,  8,  1,  63.2961),
            (1998,  9,  1,  63.3126),
            (1998, 10,  1,  63.3422),
            (1998, 11,  1,  63.3871),
            (1998, 12,  1,  63.4339),
            (1999,  1,  1,  63.4673),
            (1999,  2,  1,  63.4979),
            (1999,  3,  1,  63.5319),
            (1999,  4,  1,  63.5679),
            (1999,  5,  1,  63.6104),
            (1999,  6,  1,  63.6444),
            (1999,  7,  1,  63.6642),
            (1999,  8,  1,  63.6739),
            (1999,  9,  1,  63.6926),
            (1999, 10,  1,  63.7147),
            (1999, 11,  1,  63.7518),
            (1999, 12,  1,  63.7927),
            (2000,  1,  1,  63.8285),
            (2000,  2,  1,  63.8557),
            (2000,  3,  1,  63.8804),
            (2000,  4,  1,  63.9075),
            (2000,  5,  1,  63.9393),
            (2000,  6,  1,  63.9691),
            (2000,  7,  1,  63.9799),
            (2000,  8,  1,  63.9833),
            (2000,  9,  1,  63.9938),
            (2000, 10,  1,  64.0093),
            (2000, 11,  1,  64.0400),
            (2000, 12,  1,  64.0670),
            (2001,  1,  1,  64.0908),
            (2001,  2,  1,  64.1068),
            (2001,  3,  1,  64.1282),
            (2001,  4,  1,  64.1584),
            (2001,  5,  1,  64.1833),
            (2001,  6,  1,  64.2094),
            (2001,  7,  1,  64.2117),
            (2001,  8,  1,  64.2073),
            (2001,  9,  1,  64.2116),
            (2001, 10,  1,  64.2223),
            (2001, 11,  1,  64.2500),
            (2001, 12,  1,  64.2761),
            (2002,  1,  1,  64.2998),
            (2002,  2,  1,  64.3192),
            (2002,  3,  1,  64.3450),
            (2002,  4,  1,  64.3735),
            (2002,  5,  1,  64.3943),
            (2002,  6,  1,  64.4151),
            (2002,  7,  1,  64.4132),
            (2002,  8,  1,  64.4118),
            (2002,  9,  1,  64.4097),
            (2002, 10,  1,  64.4168),
            (2002, 11,  1,  64.4329),
            (2002, 12,  1,  64.4511),
            (2003,  1,  1,  64.4734),
            (2003,  2,  1,  64.4893),
            (2003,  3,  1,  64.5053),
            (2003,  4,  1,  64.5269),
            (2003,  5,  1,  64.5471),
            (2003,  6,  1,  64.5597),
            (2003,  7,  1,  64.5512),
            (2003,  8,  1,  64.5371),
            (2003,  9,  1,  64.5359),
            (2003, 10,  1,  64.5415),
            (2003, 11,  1,  64.5544),
            (2003, 12,  1,  64.5654),
            (2004,  1,  1,  64.5736),
            (2004,  2,  1,  64.5891),
            (2004,  3,  1,  64.6015),
            (2004,  4,  1,  64.6176),
            (2004,  5,  1,  64.6374),
            (2004,  6,  1,  64.6549),
            (2004,  7,  1,  64.6530),
            (2004,  8,  1,  64.6379),
            (2004,  9,  1,  64.6372),
            (2004, 10,  1,  64.6400),
            (2004, 11,  1,  64.6543),
            (2004, 12,  1,  64.6723),
            (2005,  1,  1,  64.6876),
            (2005,  2,  1,  64.7052),
            (2005,  3,  1,  64.7313),
            (2005,  4,  1,  64.7575),
            (2005,  5,  1,  64.7811),
            (2005,  6,  1,  64.8001),
            (2005,  7,  1,  64.7995),
            (2005,  8,  1,  64.7876),
            (2005,  9,  1,  64.7831),
            (2005, 10,  1,  64.7921),
            (2005, 11,  1,  64.8096),
            (2005, 12,  1,  64.8311),
            (2006,  1,  1,  64.8452),
            (2006,  2,  1,  64.8597),
            (2006,  3,  1,  64.8850),
            (2006,  4,  1,  64.9175),
            (2006,  5,  1,  64.9480),
            (2006,  6,  1,  64.9794),
            (2006,  7,  1,  64.9895),
            (2006,  8,  1,  65.0028),
            (2006,  9,  1,  65.0138),
            (2006, 10,  1,  65.0371),
            (2006, 11,  1,  65.0773),
            (2006, 12,  1,  65.1122),
            (2007,  1,  1,  65.1464),
            (2007,  2,  1,  65.1833),
            (2007,  3,  1,  65.2145),
            (2007,  4,  1,  65.2494),
            (2007,  5,  1,  65.2921),
            (2007,  6,  1,  65.3279),
            (2007,  7,  1,  65.3413),
            (2007,  8,  1,  65.3452),
            (2007,  9,  1,  65.3496),
            (2007, 10,  1,  65.3711),
            (2007, 11,  1,  65.3972),
            (2007, 12,  1,  65.4296),
            (2008,  1,  1,  65.4573),
            (2008,  2,  1,  65.4868),
            (2008,  3,  1,  65.5152),
            (2008,  4,  1,  65.5450),
            (2008,  5,  1,  65.5781),
            (2008,  6,  1,  65.6127),
            (2008,  7,  1,  65.6288),
            (2008,  8,  1,  65.6370),
            (2008,  9,  1,  65.6493),
            (2008, 10,  1,  65.6760),
            (2008, 11,  1,  65.7097),
            (2008, 12,  1,  65.7461),
            (2009,  1,  1,  65.7768),
            (2009,  2,  1,  65.8025),
            (2009,  3,  1,  65.8237),
            (2009,  4,  1,  65.8595),
            (2009,  5,  1,  65.8973),
            (2009,  6,  1,  65.9323),
            (2009,  7,  1,  65.9509),
            (2009,  8,  1,  65.9534),
            (2009,  9,  1,  65.9628),
            (2009, 10,  1,  65.9839),
            (2009, 11,  1,  66.0147),
            (2009, 12,  1,  66.0420),
            (2010,  1,  1,  66.0699),
            (2010,  2,  1,  66.0961),
            (2010,  3,  1,  66.1310),
            (2010,  4,  1,  66.1683),
            (2010,  5,  1,  66.2072),
            (2010,  6,  1,  66.2356),
            (2010,  7,  1,  66.2409),
            (2010,  8,  1,  66.2335),
            (2010,  9,  1,  66.2349),
            (2010, 10,  1,  66.2441),
            (2010, 11,  1,  66.2751),
            (2010, 12,  1,  66.3054),
            (2011,  1,  1,  66.3246),
            (2011,  2,  1,  66.3406),
            (2011,  3,  1,  66.3624),
            (2011,  4,  1,  66.3957),
            (2011,  5,  1,  66.4289),
            (2011,  6,  1,  66.4619),
            (2011,  7,  1,  66.4749),
            (2011,  8,  1,  66.4751),
            (2011,  9,  1,  66.4829),
            (2011, 10,  1,  66.5056),
            (2011, 11,  1,  66.5383),
            (2011, 12,  1,  66.5706),
            (2012,  1,  1,  66.6030),
            (2012,  2,  1,  66.6340),
            (2012,  3,  1,  66.6569),
            (2012,  4,  1,  66.6925),
            (2012,  5,  1,  66.7289),
            (2012,  6,  1,  66.7579),
            (2012,  7,  1,  66.7708),
            (2012,  8,  1,  66.7740),
            (2012,  9,  1,  66.7846),
            (2012, 10,  1,  66.8103),
            (2012, 11,  1,  66.8400),
            (2012, 12,  1,  66.8779),
            (2013,  1,  1,  66.9069),
            (2013,  2,  1,  66.9443),
            (2013,  3,  1,  66.9763),
            (2013,  4,  1,  67.0258),
            (2013,  5,  1,  67.0716),
            (2013,  6,  1,  67.1100),
            (2013,  7,  1,  67.1266),
            (2013,  8,  1,  67.1331),
            (2013,  9,  1,  67.1458),
            (2013, 10,  1,  67.1717),
            (2013, 11,  1,  67.2091),
            (2013, 12,  1,  67.2460),
            (2014,  1,  1,  67.2810),
            (2014,  2,  1,  67.3136),
            (2014,  3,  1,  67.3457),
            (2014,  4,  1,  67.3890),
            (2014,  5,  1,  67.4318),
            (2014,  6,  1,  67.4666),
            (2014,  7,  1,  67.4858),
            (2014,  8,  1,  67.4989),
            (2014,  9,  1,  67.5111),
            (2014, 10,  1,  67.5353),
            (2014, 11,  1,  67.5711),
            (2014, 12,  1,  67.6070),
            (2015,  1,  1,  67.6439),
            (2015,  2,  1,  67.6765),
            (2015,  3,  1,  67.7117),
            (2015,  4,  1,  67.7591),
            (2015,  5,  1,  67.8012),
            (2015,  6,  1,  67.8402),
            (2015,  7,  1,  67.8606),
            (2015,  8,  1,  67.8822),
            (2015,  9,  1,  67.9120),
            (2015, 10,  1,  67.9546),
            (2015, 11,  1,  68.0055),
            (2015, 12,  1,  68.0514),
            (2016,  1,  1,  68.1024),
            (2016,  2,  1,  68.1577),
            (2016,  3,  1,  68.2044),
            (2016,  4,  1,  68.2665),
            (2016,  5,  1,  68.3188),
            (2016,  6,  1,  68.3703),
            (2016,  7,  1,  68.3964),
            (2016,  8,  1,  68.4094),
            (2016,  9,  1,  68.4305),
            (2016, 10,  1,  68.4630),
            (2016, 11,  1,  68.5078),
            (2016, 12,  1,  68.5537),
            (2017,  1,  1,  68.5928)
        ];

        let mut list: Vec<(f64, f64)> = 
            Vec::with_capacity( time_delta_date_list.len() );

        for ( year, month, day, delta_t ) in time_delta_date_list {

            let jd = AstroTmBldr::from_gregorian_utc( year, month, day, 0, 0, 0)
            .build();

            match jd {
                  Ok( val ) => list.push(( val.julian_day_number(), delta_t )),
                  // Ok to panic here, should always catch this during testing.
                  Err( err) => panic!("Error: {:?}", err),
            }
        }

        list
    };
}
