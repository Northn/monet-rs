use crate::colors::linearsrgb::LinearSrgb;
//use crate::colors::oklab::{Oklab, ClipMethod};

pub fn coerce_in(val: f64, min: f64, max: f64) -> f64 {
    if max >= val && min <= val {
        val
    } else if val < min {
        min
    } else {
        max
    }
}

pub trait Color {
    fn to_linearsrgb(&self) -> LinearSrgb;

    // Ununsed

    // fn clip_to_linearsrgb(&self, method: ClipMethod, alpha: f64, oklab: Option<Oklab>) -> LinearSrgb {
    //     Oklab::clip(self.to_linearsrgb(), method, alpha, oklab)
    // }
}
