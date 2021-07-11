use std::ops::Range;
use super::mass;
use super::mass::Mass;
use super::temperature;
use super::temperature::Temperature;

pub enum SpectralType {
    O, B, A, F, G, K, M,                            // Main sequence stars.
    WR,                                             // Wolf-Rayet stars.
    L, T, Y,                                        // Cool red and brown dwarf stars.
    C,                                              // Carbon stars.
    S, MS, SC,                                      // Intermediary carbon related classes.
    DA, DB, DO, DQ, DZ, DC, DX, DAB, DAO, DAZ, DBZ, // White dwarf stars.
    P, Q                                            // Non-stellar spectral types.
}

impl SpectralType {
    pub fn is_main_sequence(self) -> bool {
        match self {
            SpectralType::O |
            SpectralType::B |
            SpectralType::A |
            SpectralType::F |
            SpectralType::G |
            SpectralType::K |
            SpectralType::M => true,
            _ => false
        }
    }

    pub fn is_white_dwarf(self) -> bool {
        match self {
            SpectralType::DA  |
            SpectralType::DB  |
            SpectralType::DO  |
            SpectralType::DQ  |
            SpectralType::DZ  |
            SpectralType::DC  |
            SpectralType::DX  |
            SpectralType::DAB |
            SpectralType::DAO |
            SpectralType::DAZ |
            SpectralType::DBZ => true,
            _ => false
        }
    }

    pub fn is_non_stellar(self) -> bool {
        match self {
            SpectralType::P |
            SpectralType::Q => true,
            _ => false
        }
    }

    pub fn effective_temperature(self) -> Range<Temperature> {
        match self {
            SpectralType::O => Temperature::K(      30_000.0) .. temperature::MAX,
            SpectralType::B => Temperature::K_range(10_000.0  .. 30_000.0),
            SpectralType::A => Temperature::K_range( 7_500.0  .. 10_000.0),
            SpectralType::F => Temperature::K_range( 6_000.0  ..  7_500.0),
            SpectralType::G => Temperature::K_range( 5_200.0  ..  6_000.0),
            SpectralType::K => Temperature::K_range( 3_700.0  ..  5_200.0),
            SpectralType::M => Temperature::K_range( 2_400.0  ..  3_700.0),
            _               => temperature::MIN .. temperature::MAX // We default to just "some temperature" since we don't know here.
         }
    }

    pub fn main_sequence_mass(self) -> Range<Mass> {
        match self {
            SpectralType::O => Mass::Msol( 16.0) .. mass::MAX,
            SpectralType::B => Mass::range( 2.1  .. 16.0,  mass::Scale::SolarMass),
            SpectralType::A => Mass::range( 1.4  ..  2.1,  mass::Scale::SolarMass),
            SpectralType::F => Mass::range( 1.04 ..  1.4,  mass::Scale::SolarMass),
            SpectralType::G => Mass::range( 0.8  ..  1.04, mass::Scale::SolarMass),
            SpectralType::K => Mass::range( 0.45 ..  0.8,  mass::Scale::SolarMass),
            SpectralType::M => Mass::range( 0.08 ..  0.45, mass::Scale::SolarMass),
            _               => mass::MIN .. mass::MAX  // We default to just "some mass" since we don't know here (even potentially negative mass).
        }
    }

    pub fn main_sequence_fraction(self) -> f32 {
        match self {
            SpectralType::O => 0.00_00003,
            SpectralType::B => 0.00_13,
            SpectralType::A => 0.00_6,
            SpectralType::F => 0.03,
            SpectralType::G => 0.07_6,
            SpectralType::K => 0.12_1,
            SpectralType::M => 0.76_45, // Most main sequence stars are M-types.
            _               => 0.0 // Not main sequence.
        }
    }
}
