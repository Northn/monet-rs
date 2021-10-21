# MonetRs

MonetRs is a library implementation of kdrag0n's custom Monet implementation for Android, based on his [android12-extensions module](https://github.com/kdrag0n/android12-extensions), rewritten to Rust. With MonetRs you can generate color palettes based on your color.

It's licensed with the MIT license for use in third party projects.

## Usage

```
use monet_rs::theme::dynamic_color_scheme::DynamicColorScheme;
use monet_rs::theme::target_colors::TargetColors;
use monet_rs::colors::srgb::Srgb;

const CHROMA_MULTIPLIER: f64 = 1.0;

let accents = DynamicColorScheme::new(TargetColors::new(CHROMA_MULTIPLIER), Srgb::new(r, g, b), CHROMA_MULTIPLIER, true);

let color: u32 = accents.get_accent1(0     /* See supported colors */).quantize8();  // returns in ARGB format
let color: u32 = accents.get_accent2(100   /* See supported colors */).quantize8();  // returns in ARGB format
let color: u32 = accents.get_accent3(500   /* See supported colors */).quantize8();  // returns in ARGB format

let color: u32 = accents.get_neutral1(600  /* See supported colors */).quantize8();  // returns in ARGB format
let color: u32 = accents.get_neutral2(700  /* See supported colors */).quantize8();  // returns in ARGB format

/* Supported colors: 0, 10, 50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000 */
```

## Projects using MonetRs

[MoonMonet](https://github.com/Northn/MoonMonet)
