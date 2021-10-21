use crate::colors::color::Color;
use crate::colors::linearsrgb::LinearSrgb;
//use crate::colors::oklab::Oklab;
use crate::colors::cielab::CieLab;
use crate::colors::illuminants::Illuminants;

pub struct CieXyz {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl CieXyz {
    // Unused
    
    // pub fn to_oklab(&self) -> Oklab {
    //     Oklab::lms_to_oklab(
    //         0.8189330101 * self.x + 0.3618667424 * self.y - 0.1288597137 * self.z,
    //         0.0329845436 * self.x + 0.9293118715 * self.y + 0.0361456387 * self.z,
    //         0.0482003018 * self.x + 0.2643662691 * self.y + 0.6338517070 * self.z
    //     )
    // }

    pub fn to_cielab(&self) -> CieLab {
        CieLab {
            l: 116.0 * CieXyz::f(self.y / Illuminants::D65.y) - 16.0,
            a: 500.0 * (CieXyz::f(self.x / Illuminants::D65.x) - CieXyz::f(self.y / Illuminants::D65.y)),
            b: 200.0 * (CieXyz::f(self.y / Illuminants::D65.y) - CieXyz::f(self.z / Illuminants::D65.z)),
        }
    }
}

impl Color for CieXyz {
    fn to_linearsrgb(&self) -> LinearSrgb {
        LinearSrgb {
            r:  3.2404542 * self.x + -1.5371385 * self.y + -0.4985314 * self.z,
            g: -0.9692660 * self.x +  1.8760108 * self.y +  0.0415560 * self.z,
            b:  0.0556434 * self.x + -0.2040259 * self.y +  1.0572252 * self.z
        }
    }
}

// Companions
impl CieXyz {
    fn f(x: f64) -> f64 {
        if x > 216.0/24389.0 {
            x.cbrt()
        } else {
            x / (108.0/841.0) + 4.0/29.0
        }
    }

    // Ununsed

    // fn f_inv(x: f64) -> f64 {
    //     if x > 6.0/29.0 {
    //         x.powf(3.0)
    //     } else {
    //         (108.0/841.0) * (x - 4.0/29.0)
    //     }
    // }
}
