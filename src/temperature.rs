#[derive(PartialEq)]
pub struct Temperature {
    kelvin: f32
}

impl Temperature {
    pub fn kelvin(kelvin: f32) -> Temperature {
        Temperature {
            kelvin: kelvin
        }
    }

    pub fn celsius(celsius: f32) -> Temperature {
        Temperature {
            kelvin: celsius + 273.15
        }
    }

    pub fn fahrenheit(fahrenheit: f32) -> Temperature {
        Temperature {
            kelvin: (fahrenheit + 459.67) * (5.0/9.0)
        }
    }

    pub fn rankine(rankine: f32) -> Temperature {
        Temperature {
            kelvin: (rankine + 459.67)
        }
    }
}
