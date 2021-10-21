use std::collections::HashMap;
use crate::colors::srgb::Srgb;

pub type ColorSwatch = HashMap<u16, Srgb>;

pub struct ColorScheme {
    pub neutral1: ColorSwatch,
    pub neutral2: ColorSwatch,
    
    pub accent1: ColorSwatch,
    pub accent2: ColorSwatch,
    pub accent3: ColorSwatch
}
