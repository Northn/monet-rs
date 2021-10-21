use crate::colors::color::Color;
use crate::colors::linearsrgb::LinearSrgb;
use crate::colors::cielab::CieLab;

pub struct CieLch {
    pub l: f64,
    pub c: f64,
    pub h: f64
}

impl CieLch {
    pub fn to_cielab(&self) -> CieLab {
        CieLab {
            l: self.l,
            a: self.calc_lab_a(),
            b: self.calc_lab_b()
        }
    }
}

impl Color for CieLch {
    fn to_linearsrgb(&self) -> LinearSrgb {
        self.to_cielab().to_linearsrgb()
    }
}

// Companions
impl CieLch {
    fn calc_lab_a(&self) -> f64 {
        self.c * self.h.to_radians().cos()
    }

    fn calc_lab_b(&self) -> f64 {
        self.c * self.h.to_radians().sin()
    }
}
