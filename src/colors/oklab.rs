use crate::colors::color::Color;
use crate::colors::linearsrgb::LinearSrgb;
//use crate::colors::ciexyz::CieXyz;
use crate::colors::oklch::Oklch;
//use crate::colors::saturation_coefficients::SaturationCoefficients;

pub struct Oklab {
    pub l: f64,
    pub a: f64,
    pub b: f64
}

impl Oklab {
    // Unused
    
    // pub fn to_ciexyz(&self) -> CieXyz {
    //     let l = self.oklab_to_l();
    //     let m = self.oklab_to_m();
    //     let s = self.oklab_to_s();

    //     CieXyz {
    //         x:  1.2270138511 * l - 0.5577999807 * m + 0.2812561490 * s,
    //         y: -0.0405801784 * l + 1.1122568696 * m - 0.0716766787 * s,
    //         z: -0.0763812845 * l - 0.4214819784 * m + 1.5861632204 * s,
    //     }
    // }

    pub fn to_oklch(&self) -> Oklch {
        Oklch {
            l: self.l,
            c: self.calc_lch_c(),
            h: self.calc_lch_h()
        }
    }
}

impl Color for Oklab {
    fn to_linearsrgb(&self) -> LinearSrgb {
        let l = self.oklab_to_l();
        let m = self.oklab_to_m();
        let s = self.oklab_to_s();

        LinearSrgb {
            r:  4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
            g: -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
            b: -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s
        }
    }
}

// Companions
impl Oklab {
    fn oklab_to_l(&self) -> f64 {
        (self.l + 0.3963377774 * self.a + 0.2158037573 * self.b).powf(3.0)
    }

    fn oklab_to_m(&self) -> f64 {
        (self.l - 0.1055613458 * self.a - 0.0638541728 * self.b).powf(3.0)
    }

    fn oklab_to_s(&self) -> f64 {
        (self.l - 0.0894841775 * self.a - 1.2914855480 * self.b).powf(3.0)
    }

    pub fn lms_to_oklab(l: f64, m: f64, s: f64) -> Oklab {
        let lp = l.cbrt();
        let mp = m.cbrt();
        let sp = s.cbrt();

        Oklab {
            l: 0.2104542553 * lp + 0.7936177850 * mp - 0.0040720468 * sp,
            a: 1.9779984951 * lp - 2.4285922050 * mp + 0.4505937099 * sp,
            b: 0.0259040371 * lp + 0.7827717662 * mp - 0.8086757660 * sp,
        }
    }

    fn calc_lch_c(&self) -> f64 {
        (self.a * self.a + self.b * self.b).sqrt()
    }

    fn calc_lch_h(&self) -> f64 {
        let h_deg = self.b.atan2(self.a).to_degrees();
        if h_deg < 0.0 {
            h_deg + 360.0
        } else {
            h_deg
        }
    }
}

// Unused, but can be used for ZCAM implementation.

// pub enum ClipMethod {
//     PreserveLightness,

//     // ProjectToMid,
//     // ProjectToLcusp,

//     // AdaptiveTowardsMid,
//     AdaptiveTowardsLcusp
// }

// pub struct LC {
//     l: f64,
//     c: f64
// }

// OklabGamut
// impl Oklab {
//     const CLIP_EPSILON: f64 = 0.00001;

//     pub fn clip(rgb: LinearSrgb, method: ClipMethod, alpha: f64, oklab: Option<Oklab>) -> LinearSrgb {
//         if Oklab::f64_in_range(rgb.r, 0.0, 1.0) && Oklab::f64_in_range(rgb.g, 0.0, 1.0) && Oklab::f64_in_range(rgb.b, 0.0, 1.0) {
//             return rgb;
//         }

//         let lab = match oklab {
//             None => rgb.to_oklab(),
//             Some(t) => t
//         };

//         let l = lab.l;
//         let c = Oklab::CLIP_EPSILON.max((lab.a * lab.a + lab.b * lab.b).sqrt());

//         let a_ = lab.a / c;
//         let b_ = lab.b / c;

//         let cusp = Oklab::find_cusp(a_, b_);

//         let l0 = match method {
//             ClipMethod::PreserveLightness   => coerce_in(l, 0.0, 1.0),
//             ClipMethod::AdaptiveTowardsLcusp=> {
//                 let ld = l - cusp.l;
//                 let _k = if ld > 0.0 {
//                     1.0 -  cusp.l
//                 } else {
//                     cusp.l
//                 };
//                 let k = 2.0 * _k;
//                 let e1 = 0.5 * k + ld.abs() + alpha * c/k;

//                 cusp.l + 0.5 * (ld * (e1 - (e1 * e1 - 2.0 * k * ld.abs()).sqrt()))
//             },
            
//             // Others are probably useless?

//             // ClipMethod::ProjectToMid        => 0.5,
//             // ClipMethod::ProjectToLcusp      => cusp.l,

//             #[allow(unreachable_patterns)]
//             _                               => panic!("Other")
//         };

//         let t = Oklab::find_gamut_intersection(cusp, a_, b_, l, c, l0);
//         let l_clipped = l0 * (1.0 - t) + t * l;
//         let c_clipped = t * c;

//         Oklab {
//             l: l_clipped,
//             a: c_clipped * a_,
//             b: c_clipped * b_
//         }.to_linearsrgb()
//     }

//     fn find_gamut_intersection(cusp: LC, a: f64, b: f64, l1: f64, c1: f64, l0: f64) -> f64 {
//         if ((l1 - l0) * cusp.c - (cusp.l - l0) * c1) <= 0.0 {
//             // Lower half
//             return cusp.c * l0 / (c1 * cusp.l + cusp.c * (l0 - l1));
//         };

//         // Upper half

//         // First intersect with triangle
//         let t = cusp.c * (l0 - 1.0) / (c1 * (cusp.l - 1.0) + cusp.c * (l0 - l1));

//         // Then one step Halley's method
//         {
//             let d_l = l1 - l0;
//             let d_c = c1;

//             let k_l =  0.3963377774 * a + 0.2158037573 * b;
//             let k_m = -0.1055613458 * a - 0.0638541728 * b;
//             let k_s = -0.0894841775 * a - 1.2914855480 * b;

//             let l_dt = d_l + d_c * k_l;
//             let m_dt = d_l + d_c * k_m;
//             let s_dt = d_l + d_c * k_s;

//             // If higher accuracy is required, 2 or 3 iterations of the following block can be used:
//             {
//                 let l = l0 * (1.0 - t) + t * l1;
//                 let c = t * c1;

//                 let l_ = l + c * k_l;
//                 let m_ = l + c * k_m;
//                 let s_ = l + c * k_s;

//                 let l = l_.powf(3.0);
//                 let m = m_.powf(3.0);
//                 let s = s_.powf(3.0);

//                 let ldt = 3.0 * l_dt * l_.powf(2.0);
//                 let mdt = 3.0 * m_dt * m_.powf(2.0);
//                 let sdt = 3.0 * s_dt * s_.powf(2.0);

//                 let ldt2 = 6.0 * l_dt.powf(2.0) * l_;
//                 let mdt2 = 6.0 * m_dt.powf(2.0) * m_;
//                 let sdt2 = 6.0 * s_dt.powf(2.0) * s_;

//                 let t_r = Oklab::halley_term(
//                     l, m, s, ldt, mdt, sdt, ldt2, mdt2, sdt2,
//                     4.0767416621, -3.3077115913, 0.2309699292,
//                 );
//                 let t_g = Oklab::halley_term(
//                     l, m, s, ldt, mdt, sdt, ldt2, mdt2, sdt2,
//                     -1.2681437731, 2.6097574011, -0.3413193965,
//                 );
//                 let t_b = Oklab::halley_term(
//                     l, m, s, ldt, mdt, sdt, ldt2, mdt2, sdt2,
//                     -0.0041960863, -0.7034186147, 1.7076147010,
//                 );

//                 t + t_r.min(t_g.min(t_b))
//             }
//         }
//     }

//     fn halley_term(
//         l: f64, m: f64, s: f64,
//         ldt: f64, mdt: f64, sdt: f64,
//         ldt2: f64, mdt2: f64, sdt2: f64,
//         coeff1: f64, coeff2: f64, coeff3: f64
//     ) -> f64 {
//         let n = coeff1 * l + coeff2 * m + coeff3 * s - 1.0;
//         let n1 = coeff1 * ldt + coeff2 * mdt + coeff3 * sdt;
//         let n2 = coeff1 * ldt2 + coeff2 * mdt2 + coeff3 * sdt2;

//         let u_n = n1 / (n1 * n1 - 0.5 * n * n2);
//         let t_n = -n * u_n;

//         if u_n >= 0.0 {
//             t_n
//         } else {
//             f64::MAX
//         }
//     }

//     fn find_cusp(a: f64, b: f64) -> LC {
//         let s_cusp = Oklab::compute_max_saturation(a, b);

//         let rgb_at_max = Oklab {
//             l: 1.0,
//             a: s_cusp * a,
//             b: s_cusp * b
//         }.to_linearsrgb();
//         let l_cusp = (1.0 / rgb_at_max.b.max(rgb_at_max.r.max(rgb_at_max.g))).cbrt();
//         let s_cusp = l_cusp * s_cusp;

//         LC {
//             l: l_cusp,
//             c: s_cusp
//         }
//     }

//     fn compute_max_saturation(a: f64, b: f64) -> f64 {
//         let coeff = if -1.88170328 * a - 0.80936493 * b > 1.0 {
//             SaturationCoefficients::red()
//         } else if 1.81444104 * a - 1.19445276 * b > 1.0 {
//             SaturationCoefficients::green()
//         } else {
//             SaturationCoefficients::blue()
//         };

//         let s = coeff.k0 + coeff.k1 * a + coeff.k2 * b + coeff.k3 * a * a + coeff.k4 * a * b;

//         let k_l =  0.3963377774 * a + 0.2158037573 * b;
//         let k_m = -0.1055613458 * a - 0.0638541728 * b;
//         let k_s = -0.0894841775 * a - 1.2914855480 * b;

//         {
//             let l_ = 1.0 + s * k_l;
//             let m_ = 1.0 + s * k_m;
//             let s_ = 1.0 + s * k_s;

//             let l = l_.powf(3.0);
//             let m = m_.powf(3.0);
//             let s = s_.powf(3.0);

//             let l_d_s = 3.0 * k_l * l_.powf(2.0);
//             let m_d_s = 3.0 * k_m * m_.powf(2.0);
//             let s_d_s = 3.0 * k_s * s_.powf(2.0);

//             let l_d_s2 = 6.0 * k_l.powf(2.0) * l_;
//             let m_d_s2 = 6.0 * k_m.powf(2.0) * m_;
//             let s_d_s2 = 6.0 * k_s.powf(2.0) * s_;

//             let f  = coeff.wl * l     + coeff.wm * m     + coeff.ws * s;
//             let f1 = coeff.wl * l_d_s  + coeff.wm * m_d_s  + coeff.ws * s_d_s;
//             let f2 = coeff.wl * l_d_s2 + coeff.wm * m_d_s2 + coeff.ws * s_d_s2;

//             s - f * f1 / (f1*f1 - 0.5 * f * f2)
//         }
//     }

//     fn f64_in_range(f: f64, min: f64, max: f64) -> bool {
//         f >= min && f <= max
//     }
// }
