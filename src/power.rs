use std::default::Default;

#[derive(Clone, Copy, PartialEq)]
pub enum Scale {
    Watt,
    Kilowatt,
    Megawatt,
    Gigawatt,
    Terawatt,
    Petawatt,
    Horsepower,
    SolarLuminosity
}

impl Default for Scale {
    fn default() -> Scale {
        Scale::Watt
    }
}

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Power {
    watts: f64,
    scale: Scale
}

pub const KILOWATTS_TO_WATTS: f64 = 1.0e3;
pub const MEGAWATTS_TO_WATTS: f64 = 1.0e6;
pub const GIGAWATTS_TO_WATTS: f64 = 1.0e9;
pub const TERAWATTS_TO_WATTS: f64 = 1.0e12;
pub const PETAWATTS_TO_WATTS: f64 = 1.0e15;
pub const HORSEPOWER_TO_WATTS: f64 = 745.7;
pub const SOLAR_LUMINOSITY_TO_WATTS: f64 = 3.848e26;

pub const ZERO: Power = Power { watts: 0.0,           scale: Scale::Watt };
pub const MAX:  Power = Power { watts: std::f64::MAX, scale: Scale::Watt };

impl Power {
    pub fn scaled(power: f64, scale: Scale) -> Power {
        match scale {
            Scale::Watt            => Power { watts: power,                             scale: scale },
            Scale::Kilowatt        => Power { watts: power * KILOWATTS_TO_WATTS,        scale: scale },
            Scale::Megawatt        => Power { watts: power * MEGAWATTS_TO_WATTS,        scale: scale },
            Scale::Gigawatt        => Power { watts: power * GIGAWATTS_TO_WATTS,        scale: scale },
            Scale::Terawatt        => Power { watts: power * TERAWATTS_TO_WATTS,        scale: scale },
            Scale::Petawatt        => Power { watts: power * PETAWATTS_TO_WATTS,        scale: scale },
            Scale::Horsepower      => Power { watts: power * HORSEPOWER_TO_WATTS,       scale: scale },
            Scale::SolarLuminosity => Power { watts: power * SOLAR_LUMINOSITY_TO_WATTS, scale: scale }
        }
    }

    pub fn new(watts: f64) -> Power {
        Power::watts(watts)
    }

    pub fn watts(watts: f64) -> Power {
        Power::scaled(watts, Scale::Watt)
    }

    pub fn kilowatts(kilowatts: f64) -> Power {
        Power::scaled(kilowatts, Scale::Kilowatt)
    }

    pub fn megawatts(megawatts: f64) -> Power {
        Power::scaled(megawatts, Scale::Megawatt)
    }

    pub fn gigawatts(gigawatts: f64) -> Power {
        Power::scaled(gigawatts, Scale::Gigawatt)
    }

    pub fn terawatts(terawatts: f64) -> Power {
        Power::scaled(terawatts, Scale::Terawatt)
    }

    pub fn petawatts(petawatts: f64) -> Power {
        Power::scaled(petawatts, Scale::Petawatt)
    }

    pub fn horsepower(horsepower: f64) -> Power {
        Power::scaled(horsepower, Scale::Horsepower)
    }

    pub fn solar_luminosity(solar_luminosity: f64) -> Power {
        Power::scaled(solar_luminosity, Scale::SolarLuminosity)
    }

    #[allow(non_snake_case)]
    pub fn W(watts: f64) -> Power {
        Power::watts(watts)
    }

    #[allow(non_snake_case)]
    pub fn kW(kilowatts: f64) -> Power {
        Power::kilowatts(kilowatts)
    }

    #[allow(non_snake_case)]
    pub fn MW(megawatts: f64) -> Power {
        Power::megawatts(megawatts)
    }

    #[allow(non_snake_case)]
    pub fn GW(gigawatts: f64) -> Power {
        Power::gigawatts(gigawatts)
    }

    #[allow(non_snake_case)]
    pub fn TW(terawatts: f64) -> Power {
        Power::terawatts(terawatts)
    }

    #[allow(non_snake_case)]
    pub fn PW(petawatts: f64) -> Power {
        Power::petawatts(petawatts)
    }

    pub fn hp(horsepower: f64) -> Power {
        Power::horsepower(horsepower)
    }

    #[allow(non_snake_case)]
    pub fn Lsol(solar_luminosity: f64) -> Power {
        Power::solar_luminosity(solar_luminosity)
    }

    pub fn range(range: std::ops::Range<f64>, scale: Scale) -> std::ops::Range<Power> {
        Power::scaled(range.start, scale) .. Power::scaled(range.end, scale)
    }
}
