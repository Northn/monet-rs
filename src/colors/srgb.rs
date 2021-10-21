use crate::colors::color::{Color, coerce_in};
use crate::colors::linearsrgb::LinearSrgb;

#[derive(Copy, Clone)]
pub struct Srgb {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

impl Srgb {
    pub fn new(r: u8, g: u8, b: u8) -> Srgb {
        Srgb {
            r: (r as f64) / 255.0,
            g: (g as f64) / 255.0,
            b: (b as f64) / 255.0
        }
    }

    pub fn quantize8(&self) -> u32 {
        let mut ret: u32 = 0xFF;
        ret <<= 8;
        ret |= Srgb::quantize8_(self.r);
        ret <<= 8;
        ret |= Srgb::quantize8_(self.g);
        ret <<= 8;
        ret |= Srgb::quantize8_(self.b);

        ret
    }
}

impl Color for Srgb {
    fn to_linearsrgb(&self) -> LinearSrgb {
        LinearSrgb {
            r: Srgb::f_inv(self.r),
            g: Srgb::f_inv(self.g),
            b: Srgb::f_inv(self.b)
        }
    }
}

// Companions
impl Srgb {
    // Linear -> sRGB
    pub fn f(x: f64) -> f64 {
        if x >= 0.0031308 {
            1.055 * x.powf(1.0 / 2.4) - 0.055
        } else {
            12.92 * x
        }
    }

    // sRGB -> linear
    pub fn f_inv(x: f64) -> f64 {
        if x >= 0.04045 {
            ((x + 0.055) / 1.055).powf(2.4)
        } else {
            x / 12.92
        }
    }

    // Clamp out-of-bounds values
    fn quantize8_(n: f64) -> u32 {
        coerce_in((n * 255.0).round(), 0.0, 255.0) as u32
    }
}
