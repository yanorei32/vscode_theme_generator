use palette::Srgb;
use crate::model::{ColorMap, Theme};

#[derive(Debug, Clone)]
pub struct BasePalette {
    theme: Theme,
    color_map: ColorMap<Srgb>,
}

impl BasePalette {
    pub fn new(theme: Theme, color_map: ColorMap<Srgb>) -> Self {
        Self { theme, color_map }
    }

    pub fn theme(&self) -> Theme {
        self.theme
    }

    pub fn take(self) -> (Theme, ColorMap<Srgb>) {
        (self.theme, self.color_map)
    }

    // for "impl From<&BasePalette> for FullPalette"
    pub(in crate::model) fn color_map(&self) -> &ColorMap<Srgb> {
        &self.color_map
    }
}
