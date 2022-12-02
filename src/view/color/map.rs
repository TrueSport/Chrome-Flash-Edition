use syntect::highlighting::Theme;
use crate::view::color::to_rgb_color;
use crate::view::color::{Colors, RGBColor};

pub trait ColorMap {
    fn map_colors(&self, colors: Colors) -> Colors;
}

impl ColorMap for Theme {
    fn map_colors(&self, colors: Colors) -> Colors {
        let fg = self.
            settings.
            foreground.
            map(to_rgb_color).
            unwrap_or(RGBColor(255, 255, 255));

        let bg = self.
            settings.
            background.
            map(to_rgb_color).
            unwrap_or(RGBColor(0, 0, 0));

        let alt_bg = self.
            settings.
            line_highlight.
            map(to_rgb_color).
            unwrap_or(RGBColor(55, 55, 55));

   