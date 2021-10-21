use crate::colors::color::Color;
use crate::colors::oklch::Oklch;
use crate::theme::color_scheme::ColorSwatch;
use crate::theme::color_scheme::ColorScheme;

pub struct TargetColors {}

impl TargetColors {
    const ACCENT1_CHROMA: f64 = 0.1328123146401862;
    const ACCENT2_CHROMA: f64 = TargetColors::ACCENT1_CHROMA / 3.0;
    const ACCENT3_CHROMA: f64 = TargetColors::ACCENT2_CHROMA * 2.0;

    const NEUTRAL1_CHROMA: f64 = TargetColors::ACCENT1_CHROMA / 12.0;
    const NEUTRAL2_CHROMA: f64 = TargetColors::NEUTRAL1_CHROMA * 2.0;
    
    pub fn new(chroma_factor: f64) -> ColorScheme {
        ColorScheme {
            accent1: TargetColors::shades_with_chroma(chroma_factor, TargetColors::ACCENT1_CHROMA),
            accent2: TargetColors::shades_with_chroma(chroma_factor, TargetColors::ACCENT2_CHROMA),
            accent3: TargetColors::shades_with_chroma(chroma_factor, TargetColors::ACCENT3_CHROMA),
            neutral1: TargetColors::shades_with_chroma(chroma_factor, TargetColors::NEUTRAL1_CHROMA),
            neutral2: TargetColors::shades_with_chroma(chroma_factor, TargetColors::NEUTRAL2_CHROMA),
        }
    }

    fn shades_with_chroma(chroma_factor: f64, chroma: f64) -> ColorSwatch {
        // Adjusted chroma
        let chroma_adj = chroma * chroma_factor;

        let mut ret: ColorSwatch = ColorSwatch::new();
        ret.insert(0, Oklch {l: 1.0, c: chroma_adj, h: 0.0}.to_linearsrgb().to_srgb());
        ret.insert(10, Oklch {l: 0.9880873963836093, c: chroma_adj, h: 0.0}.to_linearsrgb().to_srgb());
        ret.insert(50, Oklch {l: 0.9551400440214246, c: chroma_adj, h: 0.0}.to_linearsrgb().to_srgb());

        ret.insert(100, Oklch {l: 0.9127904082618294, c: chroma_adj, h: 0.0}.to_linearsrgb().to_srgb());
        ret.insert(200, Oklch {l: 0.8265622041716898, c: chroma_adj, h: 0.0}.to_linearsrgb().to_srgb());
        ret.insert(300, Oklch {l: 0.7412252673769428, c: chroma_adj, h: 0.0}.to_linearsrgb().to_srgb());

        ret.insert(400, Oklch {l: 0.653350946076347, c: chroma_adj, h: 0.0}.to_linearsrgb().to_srgb());
        ret.insert(500, Oklch {l: 0.5624050605208273, c: chroma_adj, h: 0.0}.to_linearsrgb().to_srgb());
        ret.insert(600, Oklch {l: 0.48193149058901036, c: chroma_adj, h: 0.0}.to_linearsrgb().to_srgb());

        ret.insert(700, Oklch {l: 0.39417829080418526, c: chroma_adj, h: 0.0}.to_linearsrgb().to_srgb());
        ret.insert(800, Oklch {l: 0.3091856317280812, c: chroma_adj, h: 0.0}.to_linearsrgb().to_srgb());
        ret.insert(900, Oklch {l: 0.22212874192541768, c: chroma_adj, h: 0.0}.to_linearsrgb().to_srgb());

        ret.insert(1000, Oklch {l: 0.0, c: chroma_adj, h: 0.0}.to_linearsrgb().to_srgb());

        ret
    }

    pub fn get_lstar_lightness(accent: u16) -> f64 {
        match accent {
            0    => 100.0,
            10   =>  99.0,
            50   =>  95.0,
            100  =>  90.0,
            200  =>  80.0,
            300  =>  70.0,
            400  =>  60.0,
            500  =>  49.6,
            600  =>  40.0,
            700  =>  30.0,
            800  =>  20.0,
            900  =>  10.0,
            1000 =>   0.0,
            _    => panic!("Accent not exists")
        }
    }
}