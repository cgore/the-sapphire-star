// We see that f64's maximum is approximately 1.8*10^308.
// If we use this for grams, we convert into 9.05*10^274 solar masses.
// The entire mass of The Milky Way Galaxy is approximately 1.5 trillion solar masses.
// Therefore we feel f64 is acceptable here.

pub enum Scale {
    Gram, Kilogram,
    Ounce, Pound,
    EarthMass, SolarMass
}

pub struct Mass {
    grams: f64,
    scale: Scale
}

impl Mass {
    pub fn new(grams: f64) -> Mass {
        Mass::grams(grams)
    }

    pub fn grams(grams: f64) -> Mass {
        Mass {
            grams: grams,
            scale: Scale::Gram
        }
    }

    pub fn kilograms(kilograms: f64) -> Mass {
        Mass {
            grams: kilograms / 1_000.0,
            scale: Scale::Kilogram
        }
    }

    pub fn ounces(ounces: f64) -> Mass {
        Mass {
            grams: ounces * 28.34952,
            scale: Scale::Ounce
        }
    }

    pub fn pounds(pounds: f64) -> Mass {
        Mass {
            grams: pounds * 453.5924,
            scale: Scale::Pound
        }
    }

    pub fn earth_masses(earth_masses: f64) -> Mass {
        Mass {
            grams: earth_masses * 5.9722e24 * 1_000.0,
            scale: Scale::EarthMass
        }
    }

    pub fn solar_masses(solar_masses: f64) -> Mass {
        Mass {
            grams: solar_masses * 1.98847e30 * 1_000.0,
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
    pub fn Mearth(earth_masses: f64) -> Mass {
        Mass::earth_masses(earth_masses)
    }

    #[allow(non_snake_case)]
    pub fn Msol(solar_masses: f64) -> Mass {
        Mass::solar_masses(solar_masses)
    }
}
