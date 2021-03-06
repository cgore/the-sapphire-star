use std::default::Default;

#[derive(Clone, Copy, PartialEq)]
pub enum Scale {
    Millimeter, Centimeter, Meter, Kilometer, Megameter,
    Inch, Hand, Foot, Cubit, Yard, Mile, NauticalMile,
    EarthRadius, SolarRadius, AstronomicalUnit, Parsec, Kiloparsec, Megaparsec,
    LightSecond, LightMinute, LightHour, LightDay, LightWeek, LightYear
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
pub const MEGAMETERS_TO_METERS:     f64 =         1_000_000.0;
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

pub const ZERO: Length = Length { meters: 0.0,           scale: Scale::Meter };
pub const MAX:  Length = Length { meters: std::f64::MAX, scale: Scale::Meter };

impl Length {
    pub fn scaled(length: f64, scale: Scale) -> Length {
        let length_in_meters = match scale {
            Scale::Millimeter       => length / MILLIMETERS_TO_METERS,
            Scale::Centimeter       => length / CENTIMETERS_TO_METERS,
            Scale::Meter            => length,
            Scale::Kilometer        => length * KILOMETERS_TO_METERS,
            Scale::Megameter        => length * MEGAMETERS_TO_METERS,
            Scale::Inch             => length * INCHES_TO_METERS,
            Scale::Hand             => length * HANDS_TO_METERS,
            Scale::Foot             => length * FEET_TO_METERS,
            Scale::Cubit            => length * CUBITS_TO_METERS,
            Scale::Yard             => length * YARDS_TO_METERS,
            Scale::Mile             => length * MILES_TO_METERS,
            Scale::NauticalMile     => length * NAUTICAL_MILES_TO_METERS,
            Scale::EarthRadius      => length * EARTH_RADII_TO_METERS,
            Scale::SolarRadius      => length * SOLAR_RADII_TO_METERS,
            Scale::AstronomicalUnit => length * AU_TO_METERS,
            Scale::Parsec           => length * PARSECS_TO_METERS,
            Scale::Kiloparsec       => length * KILOPARSECS_TO_METERS,
            Scale::Megaparsec       => length * MEGAPARSECS_TO_METERS,
            Scale::LightSecond      => length * LIGHT_SECONDS_TO_METERS,
            Scale::LightMinute      => length * LIGHT_MINUTES_TO_METERS,
            Scale::LightHour        => length * LIGHT_HOURS_TO_METERS,
            Scale::LightDay         => length * LIGHT_DAYS_TO_METERS,
            Scale::LightWeek        => length * LIGHT_WEEKS_TO_METERS,
            Scale::LightYear        => length * LIGHT_YEARS_TO_METERS,
        };
        Length { meters: length_in_meters, scale: scale }
    }

    pub fn solar_radii(solar_radii: f64) -> Length {
        Length::scaled(solar_radii, Scale::SolarRadius)
    }

    #[allow(non_snake_case)]
    pub fn Rsol(solar_radii: f64) -> Length {
        Length::solar_radii(solar_radii)
    }

    pub fn range(range: std::ops::Range<f64>, scale: Scale) -> std::ops::Range<Length> {
        Length::scaled(range.start, scale) .. Length::scaled(range.end, scale)
    }
}
