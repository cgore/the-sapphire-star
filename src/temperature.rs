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

    pub fn to_kelvin(temperature: Temperature) -> Temperature {
        Temperature {
            kelvin: temperature.kelvin,
            scale: Scale::Kelvin
        }
    }

    pub fn to_celsius(temperature: Temperature) -> Temperature {
        Temperature {
            kelvin: temperature.kelvin,
            scale: Scale::Celsius
        }
    }

    pub fn to_fahrenheit(temperature: Temperature) -> Temperature {
        Temperature {
            kelvin: temperature.kelvin,
            scale: Scale::Fahrenheit
        }
    }

    pub fn to_rankine(temperature: Temperature) -> Temperature {
        Temperature {
            kelvin: temperature.kelvin,
            scale: Scale::Rankine
        }
    }
}
