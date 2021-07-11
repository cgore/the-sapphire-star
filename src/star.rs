use std::ops::Range;
use super::temperature::Temperature;

// pub enum SpectralType {
//     O,
//     B,
//     A,
//     F,
//     G,
//     K,
//     M,
//     WR,
//     L,
//     T,
//     Y,
//     C,
//     S,
//     MS,
//     DA,
//     DB,
//     DO,
//     DQ,
//     DZ,
//     DC,
//     DX,
//     DAB,
//     DAO,
//     DAZ,
//     DBZ,
//     P,
//     Q
// }

pub struct SpectralType {
    effective_temperature: Range<Temperature>
}
