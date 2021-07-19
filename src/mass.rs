use std::default::Default;
use std::convert::{From, Into};
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub enum Scale {
    Gram, Kilogram,
    Ounce, Pound,
    LunarMass, EarthMass, JovianMass, SolarMass
}

impl Default for Scale {
    fn default() -> Scale {
        Scale::Gram
    }
}

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Mass {
    /// We see that f64's maximum is approximately 1.8*10^308.
    /// If we use this for grams, we convert into 9.05*10^274 solar masses.
    /// The entire mass of The Milky Way Galaxy is approximately 1.5 trillion solar masses.
    /// Therefore we feel f64 is acceptable here.
    grams: f64,
    scale: Scale
}

const GRAMS_TO_KILOGRAMS: f64 = 1_000.0;
const KILOGRAMS_TO_GRAMS: f64 = 1.0 / GRAMS_TO_KILOGRAMS;
const OUNCES_TO_GRAMS: f64 = 28.34952;
const POUNDS_TO_GRAMS: f64 = 453.5924;
const LUNAR_MASSES_TO_GRAMS: f64 = 7.342e22 * 1_000.0;
const EARTH_MASSES_TO_GRAMS: f64 = 5.9722e24 * 1_000.0;
const JOVIAN_MASSES_TO_GRAMS: f64 = 1.89813e27 * 1_000.0;
const SOLAR_MASSES_TO_GRAMS: f64 = 1.98847e30 * 1_000.0;

pub const MIN: Mass = Mass { grams: std::f64::MIN, scale: Scale::Gram };
pub const MAX: Mass = Mass { grams: std::f64::MAX, scale: Scale::Gram };

impl Mass {
    pub fn new(grams: f64) -> Mass {
        Mass::grams(grams)
    }

    pub fn scaled(mass: f64, scale: Scale) -> Mass {
        match scale {
            Scale::Gram        => Mass::grams(mass),
            Scale::Kilogram    => Mass::kilograms(mass),
            Scale::Ounce       => Mass::ounces(mass),
            Scale::Pound       => Mass::pounds(mass),
            Scale::LunarMass   => Mass::lunar_masses(mass),
            Scale::EarthMass   => Mass::earth_masses(mass),
            Scale::JovianMass  => Mass::jovian_masses(mass),
            Scale::SolarMass   => Mass::solar_masses(mass),
        }
    }

    pub fn grams(grams: f64) -> Mass {
        Mass {
            grams: grams,
            scale: Scale::Gram
        }
    }

    pub fn kilograms(kilograms: f64) -> Mass {
        Mass {
            grams: kilograms / GRAMS_TO_KILOGRAMS,
            scale: Scale::Kilogram
        }
    }

    pub fn ounces(ounces: f64) -> Mass {
        Mass {
            grams: ounces * OUNCES_TO_GRAMS,
            scale: Scale::Ounce
        }
    }

    pub fn pounds(pounds: f64) -> Mass {
        Mass {
            grams: pounds * POUNDS_TO_GRAMS,
            scale: Scale::Pound
        }
    }

    pub fn lunar_masses(lunar_masses: f64) -> Mass {
        Mass {
            grams: lunar_masses * LUNAR_MASSES_TO_GRAMS,
            scale: Scale::LunarMass
        }
    }

    pub fn earth_masses(earth_masses: f64) -> Mass {
        Mass {
            grams: earth_masses * EARTH_MASSES_TO_GRAMS,
            scale: Scale::EarthMass
        }
    }

    pub fn jovian_masses(jovian_masses: f64) -> Mass {
        Mass {
            grams: jovian_masses * JOVIAN_MASSES_TO_GRAMS,
            scale: Scale::JovianMass
        }
    }

    pub fn solar_masses(solar_masses: f64) -> Mass {
        Mass {
            grams: solar_masses * SOLAR_MASSES_TO_GRAMS,
            scale: Scale::SolarMass
        }
    }

    pub fn g(g: f64) -> Mass {
        Mass::grams(g)
    }

    pub fn kg(kg: f64) -> Mass {
        Mass::kilograms(kg)
    }

    pub fn oz(oz: f64) -> Mass {
        Mass::ounces(oz)
    }

    pub fn lb(lb: f64) -> Mass {
        Mass::pounds(lb)
    }

    #[allow(non_snake_case)]
    pub fn Mlunar(lunar_masses: f64) -> Mass {
        Mass::lunar_masses(lunar_masses)
    }

    #[allow(non_snake_case)]
    pub fn Mearth(earth_masses: f64) -> Mass {
        Mass::earth_masses(earth_masses)
    }

    #[allow(non_snake_case)]
    pub fn Mjovian(jovian_masses: f64) -> Mass {
        Mass::jovian_masses(jovian_masses)
    }

    #[allow(non_snake_case)]
    pub fn Msol(solar_masses: f64) -> Mass {
        Mass::solar_masses(solar_masses)
    }

    pub fn range(range: std::ops::Range<f64>, scale: Scale) -> std::ops::Range<Mass> {
        Mass::scaled(range.start, scale) .. Mass::scaled(range.end, scale)
    }
}


impl From<f64> for Mass {
    fn from(value: f64) -> Self {
        Mass::grams(value)
    }
}

impl Into<f64> for Mass {
    fn into(self) -> f64 {
        match self.scale {
            Scale::Gram        => self.grams,
            Scale::Kilogram    => self.grams / GRAMS_TO_KILOGRAMS,
            Scale::Ounce       => self.grams * OUNCES_TO_GRAMS,
            Scale::Pound       => self.grams * POUNDS_TO_GRAMS,
            Scale::LunarMass   => self.grams * LUNAR_MASSES_TO_GRAMS,
            Scale::EarthMass   => self.grams * EARTH_MASSES_TO_GRAMS,
            Scale::JovianMass  => self.grams * JOVIAN_MASSES_TO_GRAMS,
            Scale::SolarMass   => self.grams * SOLAR_MASSES_TO_GRAMS
        }
    }
}

impl fmt::Display for Mass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value: f64 = Mass::into(*self);
        match self.scale {
            Scale::Gram        => write!(f, "{} g", value),
            Scale::Kilogram    => write!(f, "{} kg", value),
            Scale::Ounce       => write!(f, "{} oz", value),
            Scale::Pound       => write!(f, "{} lb", value),
            Scale::LunarMass   => write!(f, "{} M☽", value),
            Scale::EarthMass   => write!(f, "{} M♁", value),
            Scale::JovianMass  => write!(f, "{} M♃", value),
            Scale::SolarMass   => write!(f, "{} M☉", value)
        }
    }
}
