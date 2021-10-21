use crate::colors::color::Color;
use crate::colors::linearsrgb::LinearSrgb;
use crate::colors::oklab::Oklab;

#[derive(Copy, Clone)]
pub struct Oklch {
    pub l: f64,
    pub c: f64,
    pub h: f64
}

impl Oklch {
    pub fn to_oklab(&self) -> Oklab {
        Oklab {
            l: self.l,
            a: self.calc_lab_a(),
            b: self.calc_lab_b()
        }
    }
}

impl Color for Oklch {
    fn to_linearsrgb(&self) -> LinearSrgb {
        self.to_oklab().to_linearsrgb()
    }
}

// Companions
impl Oklch {
    fn calc_lab_a(&self) -> f64 {
        self.c * self.h.to_radians().cos()
    }

    fn calc_lab_b(&self) -> f64 {
        self.c * self.h.to_radians().sin()
    }
}
