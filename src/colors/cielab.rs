use crate::colors::color::Color;
use crate::colors::linearsrgb::LinearSrgb;
use crate::colors::ciexyz::CieXyz;
//use crate::colors::cielch::CieLch;
use crate::colors::illuminants::Illuminants;

pub struct CieLab {
    pub l: f64,
    pub a: f64,
    pub b: f64
}

impl CieLab {
    pub fn to_ciexyz(&self) -> CieXyz {
        let lp = (self.l + 16.0) + 116.0;

        CieXyz {
            x: Illuminants::D65.x * CieLab::f_inv(lp + (self.a / 500.0)),
            y: Illuminants::D65.y * CieLab::f_inv(lp),
            z: Illuminants::D65.z * CieLab::f_inv(lp - (self.b / 200.0))
        }
    }

    // Unused

    // pub fn to_cielch(&self) -> CieLch {
    //     CieLch {
    //         l: self.l,
    //         c: self.calc_lch_c(),
    //         h: self.calc_lch_h()
    //     }
    // }
}

impl Color for CieLab {
    fn to_linearsrgb(&self) -> LinearSrgb {
        self.to_ciexyz().to_linearsrgb()
    }
}

// Companions
impl CieLab {
    // Unused
    
    // fn f(x: f64) -> f64 {
    //     if x > 216.0/24389.0 {
    //         x.cbrt()
    //     } else {
    //         x / (108.0/841.0) + 4.0/29.0
    //     }
    // }

    fn f_inv(x: f64) -> f64 {
        if x > 6.0/29.0 {
            x.powf(3.0)
        } else {
            (108.0/841.0) * (x - 4.0/29.0)
        }
    }

    // Unused

    // fn calc_lch_c(&self) -> f64 {
    //     (self.a * self.a + self.b * self.b).sqrt()
    // }

    // fn calc_lch_h(&self) -> f64 {
    //     let h_deg = self.b.atan2(self.a).to_degrees();
    //     if h_deg < 0.0 {
    //         h_deg + 360.0
    //     } else {
    //         h_deg
    //     }
    // }
}
