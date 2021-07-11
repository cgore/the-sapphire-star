// We see that f64's maximum is approximately 1.8*10^308.
// If we use this for grams, we convert into 9.05*10^274 solar masses.
// The entire mass of The Milky Way Galaxy is approximately 1.5 trillion solar masses.
// Therefore we feel f64 is acceptable here.

pub enum Scale {
    Gram,
    KiloGram,
    Pound,
    SolarMass
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
}
