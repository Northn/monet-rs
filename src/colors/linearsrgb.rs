use crate::colors::color::Color;
use crate::colors::srgb::Srgb;
use crate::colors::ciexyz::CieXyz;
use crate::colors::oklab::Oklab;

pub struct LinearSrgb {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

impl LinearSrgb {
    pub fn to_srgb(&self) -> Srgb {
        Srgb {
            r: Srgb::f(self.r),
            g: Srgb::f(self.g),
            b: Srgb::f(self.b)
        }
    }

    // Unused

    pub fn to_ciexyz(&self) -> CieXyz {
        CieXyz {
            x: 0.4124564 * self.r + 0.3575761 * self.g + 0.1804375 * self.b,
            y: 0.2126729 * self.r + 0.7151522 * self.g + 0.0721750 * self.b,
            z: 0.0193339 * self.r + 0.1191920 * self.g + 0.9503041 * self.b
        }
    }

    pub fn to_oklab(&self) -> Oklab {
        Oklab::lms_to_oklab(
            0.4122214708 * self.r + 0.5363325363 * self.g + 0.0514459929 * self.b,
            0.2119034982 * self.r + 0.6806995451 * self.g + 0.1073969566 * self.b,
            0.0883024619 * self.r + 0.2817188376 * self.g + 0.6299787005 * self.b
        ) // TODO: Check it again
    }
}

impl Color for LinearSrgb {
    fn to_linearsrgb(&self) -> LinearSrgb {
        // Cringe
        LinearSrgb {
            r: self.r,
            g: self.g,
            b: self.b
        }
    }
}
