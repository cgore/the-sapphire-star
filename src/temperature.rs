use std::convert::{TryFrom, Into};
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
enum Scale {
    Celsius,
    Fahrenheit,
    Kelvin,
    Rankine
}

#[derive(Clone, Copy, PartialEq)]
pub struct Temperature {
    kelvin: f32,
    scale: Scale
}

impl Temperature {
    pub fn new(kelvin: f32) -> Temperature {
        Temperature::kelvin(kelvin)
    }

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

impl TryFrom<f32> for Temperature {
    type Error = ();
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value >= 0.0 {
            Ok(Temperature::kelvin(value))
        } else {
            Err(())
        }
    }
}

impl Into<f32> for Temperature {
    fn into(self) -> f32 {
        match self.scale {
            Scale::Celsius    => self.kelvin - 273.15,
            Scale::Fahrenheit => self.kelvin * (9.0/5.0) - 459.67,
            Scale::Kelvin     => self.kelvin,
            Scale::Rankine    => self.kelvin * (9.0/5.0)
        }
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value: f32 = Temperature::into(*self);
        match self.scale {
            Scale::Celsius    => write!(f, "{} °C", value),
            Scale::Fahrenheit => write!(f, "{} °F", value),
            Scale::Kelvin     => write!(f, "{} K",  value),
            Scale::Rankine    => write!(f, "{} °R", value)
        }
    }
}
