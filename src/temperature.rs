#[derive(PartialEq)]
enum Scale {
    Celsius,
    Fahrenheit,
    Kelvin,
    Rankine
}

#[derive(PartialEq)]
pub struct Temperature {
    kelvin: f32,
    scale: Scale
}

impl Temperature {
    pub fn kelvin(kelvin: f32) -> Temperature {
        Temperature {
            kelvin: kelvin,
            scale: Scale::Kelvin
        }
    }

    pub fn celsius(celsius: f32) -> Temperature {
        Temperature {
            kelvin: celsius + 273.15,
            scale: Scale::Celsius
        }
    }

    pub fn fahrenheit(fahrenheit: f32) -> Temperature {
        Temperature {
            kelvin: (fahrenheit + 459.67) * (5.0/9.0),
            scale: Scale::Fahrenheit
        }
    }

    pub fn rankine(rankine: f32) -> Temperature {
        Temperature {
            kelvin: (rankine + 459.67),
            scale: Scale::Rankine
        }
    }

    pub fn to_kelvin(self) -> Temperature {
        Temperature {
            kelvin: self.kelvin,
            scale: Scale::Kelvin
        }
    }

    pub fn to_celsius(self) -> Temperature {
        Temperature {
            kelvin: self.kelvin,
            scale: Scale::Celsius
        }
    }

    pub fn to_fahrenheit(self) -> Temperature {
        Temperature {
            kelvin: self.kelvin,
            scale: Scale::Fahrenheit
        }
    }

    pub fn to_rankine(self) -> Temperature {
        Temperature {
            kelvin: self.kelvin,
            scale: Scale::Rankine
        }
    }
}
