use std::default::Default;

#[derive(Clone, Copy, PartialEq)]
pub enum Scale {
    Millimeter, Centimeter, Meter, Kilometer, Megameter,
    Inch, Hand, Foot, Cubit, Yard, Mile, NauticalMile,
    EarthRadius, SolarRadius, AstronomicalUnit, Parsec, Kiloparsec, Megaparsec,
    LightSecond, LightMinute, LightDay, LightWeek, LightYear
}

impl Default for Scale {
    fn default() -> Scale {
        Scale::Meter
    }
}

pub const MILLIMETERS_TO_METERS:    f64 =             1_000.0;
pub const METERS_TO_MILLIMETERS:    f64 = 1.0 / MILLIMETERS_TO_METERS;
pub const CENTIMETERS_TO_METERS:    f64 =               100.0;
pub const METERS_TO_CENTIMETERS:    f64 = 1.0 / CENTIMETERS_TO_METERS;
pub const KILOMETERS_TO_METERS:     f64 =             1_000.0;
pub const MEGMETERS_TO_METERS:      f64 =         1_000_000.0;
pub const INCHES_TO_METERS:         f64 =                 0.0254;
pub const HANDS_TO_METERS:          f64 =                 0.1016;
pub const FEET_TO_METERS:           f64 =                 0.3048;
pub const CUBITS_TO_METERS:         f64 =                 0.4572;
pub const YARDS_TO_METERS:          f64 =                 0.9144;
pub const MILES_TO_METERS:          f64 =              1609.344;
pub const NAUTICAL_MILES_TO_METERS: f64 =              1852.001;
pub const EARTH_RADII_TO_METERS:    f64 =         6_378_100.0; // nominal "zero tide" equatorial from IAU
pub const SOLAR_RADII_TO_METERS:    f64 =       695_700_000.0; // IAU nominal 2015
pub const AU_TO_METERS:             f64 =   149_597_870_700.0; // IAU nominal 2012
pub const PARSECS_TO_METERS:        f64 =                 3.085_677_581_491_3673e16; // Derived from IAU nominal 2012 AU
pub const KILOPARSECS_TO_METERS:    f64 = PARSECS_TO_METERS *     1_000.0;
pub const MEGAPARSECS_TO_METERS:    f64 = PARSECS_TO_METERS * 1_000_000.0;
pub const LIGHT_SECONDS_TO_METERS:  f64 =       299_792_458.0;
pub const LIGHT_MINUTES_TO_METERS:  f64 = LIGHT_SECONDS_TO_METERS * 60.0;
pub const LIGHT_HOURS_TO_METERS:    f64 = LIGHT_MINUTES_TO_METERS * 60.0;
pub const LIGHT_DAYS_TO_METERS:     f64 = LIGHT_HOURS_TO_METERS   * 24.0;
pub const LIGHT_WEEKS_TO_METERS:    f64 = LIGHT_DAYS_TO_METERS    *  7.0;
pub const LIGHT_YEARS_TO_METERS:    f64 = 9_460_730_000_000.0;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Length {
    /// We see that f64's maximum is approximately 1.8*10^308.
    /// If we use this for meters, we convert this to 1.9*10^292 light years.
    /// The entire diameter of the observable universe is only 93 BLy across.
    /// Therefore we feel f64 is acceptable here.
    meters: f64,
    scale: Scale
}

impl Length {
}
