use crate::model::{ColorMap, Theme};

#[derive(Debug, Clone, Copy)]
pub struct BasePalette {
    theme: Theme,
    color_map: ColorMap,
}

impl BasePalette {
    pub fn new(theme: Theme, color_map: ColorMap) -> Self {
        Self { theme, color_map }
    }

    pub fn theme(&self) -> Theme {
        self.theme
    }

    pub fn take(self) -> (Theme, ColorMap) {
        (self.theme, self.color_map)
    }
}
