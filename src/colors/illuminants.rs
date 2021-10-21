use crate::colors::ciexyz::CieXyz;

pub struct Illuminants {}

impl Illuminants {
    pub const D65: CieXyz = CieXyz {
        x: 0.95047,
        y: 1.0,
        z: 1.08883
    };
}
