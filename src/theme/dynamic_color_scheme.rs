use crate::colors::color::{Color, coerce_in};
use crate::colors::oklch::Oklch;
use crate::colors::srgb::Srgb;
use crate::colors::linearsrgb::LinearSrgb;
//use crate::colors::oklab::ClipMethod;
use crate::theme::target_colors::TargetColors;
use crate::theme::color_scheme::{ColorScheme, ColorSwatch};

pub struct DynamicColorScheme {
    pub targets: ColorScheme,
    pub seed_color: Srgb,
    pub chroma_factor: f64,

    accurate_shades: bool,
    seed_neutral: Oklch,
    seed_accent: Oklch
}

impl DynamicColorScheme {
    // Hue shift for the tertiary accent color (accent3), in degrees.
    // 60 degrees = shifting by a secondary color
    const ACCENT3_HUE_SHIFT_DEGREES: f64 = 60.0;

    // Threshold for matching CIELAB L* targets. Colors with lightness delta
    // under this value are considered to match the reference lightness.
    const TARGET_LSTAR_THRESHOLD: f64 = 0.01;

    // Threshold for terminating the binary search if min and max are too close.
    // The search is very unlikely to make progress after this point, so we
    // just terminate it and return the best L* value found.
    const TARGET_L_EPSILON: f64 = 0.001;

    pub fn new(targets: ColorScheme, seed_color: Srgb, chroma_factor: f64, accurate_shades: bool) -> DynamicColorScheme {
        let mut seed_neutral: Oklch = seed_color.to_linearsrgb().to_oklab().to_oklch();
        seed_neutral.c = seed_neutral.c * chroma_factor;
        DynamicColorScheme {
            targets: targets,
            seed_color: seed_color,
            chroma_factor: chroma_factor,
            accurate_shades: accurate_shades,
            seed_neutral: seed_neutral,
            seed_accent: seed_neutral
        }
    }

    pub fn get_accent1(&self, accent: u16) -> Srgb {
        let ret = self.transform_swatch(self.targets.accent1.clone(), self.seed_accent);
        match ret.get(&accent) {
            Some(v) => *v,
            None    => panic!("Accent not exists")
        }
    }

    pub fn get_accent2(&self, accent: u16) -> Srgb {
        let ret = self.transform_swatch(self.targets.accent2.clone(), self.seed_accent);
        match ret.get(&accent) {
            Some(v) => *v,
            None    => panic!("Accent not exists")
        }
    }

    pub fn get_accent3(&self, accent: u16) -> Srgb {
        let mut seed_a3: Oklch = self.seed_accent;
        seed_a3.h += DynamicColorScheme::ACCENT3_HUE_SHIFT_DEGREES;
        let ret = self.transform_swatch(self.targets.accent3.clone(), seed_a3);
        match ret.get(&accent) {
            Some(v) => *v,
            None    => panic!("Accent not exists")
        }
    }

    pub fn get_neutral1(&self, accent: u16) -> Srgb {
        let ret = self.transform_swatch(self.targets.neutral1.clone(), self.seed_neutral);
        match ret.get(&accent) {
            Some(v) => *v,
            None    => panic!("Accent not exists")
        }
    }

    pub fn get_neutral2(&self, accent: u16) -> Srgb {
        let ret = self.transform_swatch(self.targets.neutral2.clone(), self.seed_neutral);
        match ret.get(&accent) {
            Some(v) => *v,
            None    => panic!("Accent not exists")
        }
    }

    fn transform_swatch(&self, mut swatch: ColorSwatch, seed: Oklch) -> ColorSwatch {
        for (shade, color) in swatch.iter_mut() {
            let target = color.to_linearsrgb().to_oklab().to_oklch();
            let target_l_star = TargetColors::get_lstar_lightness(*shade);
            let new_lch = self.transform_color(target, &seed, target_l_star);
            let new_srgb = new_lch.to_linearsrgb().to_srgb();

            *color = new_srgb;
        }
        swatch
    }

    fn transform_color(&self, target: Oklch, seed: &Oklch, reference: f64) -> LinearSrgb {
        let c = coerce_in(seed.c, 0.0, target.c);
        let h = seed.h;
        let l = if self.accurate_shades {
            DynamicColorScheme::search_l_star(reference, c, h)
        } else {
            target.l
        };

        Oklch {l: l, c: c, h: h}.to_linearsrgb()
    }

    fn search_l_star(target_l_star: f64, c: f64, h: f64) -> f64 {
        let mut min: f64 = -0.5;
        let mut max: f64 = 1.5;

        let mut best_l: f64 = f64::MAX;
        let mut best_l_delta: f64 = f64::INFINITY;

        loop {
            let mid = (min + max) / 2.0;

            let srgb = Oklch {l: mid, c: c, h: h}.to_oklab().to_linearsrgb().to_srgb();

            let lstar = srgb.to_linearsrgb().to_ciexyz().to_cielab().l;
            let delta = (lstar - target_l_star).abs();

            if delta < best_l_delta {
                best_l = mid;
                best_l_delta = delta;
            }

            if delta <= DynamicColorScheme::TARGET_LSTAR_THRESHOLD {
                return mid;
            } else if (min - max).abs() <=  DynamicColorScheme::TARGET_L_EPSILON {
                return best_l;
            } else if lstar < target_l_star {
                min = mid;
            } else if lstar > target_l_star {
                max = mid;
            }
        }
    }
}
