use syntect::highlighting::Theme;
use crate::view::color::to_rgb_color;
use crate::view::color::{Colors, RGBColor};

pub trait ColorMap {
    fn map_colors(&self, colors: Colors) -> Colors;
}

impl ColorMap for Theme {
    fn map_colors(&self, colors: Colors) -> Colors {
        let fg = self.
            se