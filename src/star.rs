use std::ops::Range;
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
}
