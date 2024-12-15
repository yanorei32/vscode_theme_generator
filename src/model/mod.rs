use std::fmt::{Display, Error as FmtError, Formatter};

mod hexstr;
pub use hexstr::HexStr;

mod color;
pub use color::Color;

mod colormap;
pub use colormap::{ColorMap, ColorMapExt};

mod base_palette;
pub use base_palette::BasePalette;

mod full_palette;
pub use full_palette::{FullPalette, FullPaletteValue, VARIANTS as FULL_PALETTE_VARIANTS};

use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ThemeDetectionStrategy {
    Auto,
    Dark,
    Light,
}

impl From<Theme> for ThemeDetectionStrategy {
    fn from(value: Theme) -> Self {
        match value {
            Theme::Dark => Self::Dark,
            Theme::Light => Self::Light,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
}

impl Theme {
    pub fn dark(&self) -> bool {
        *self == Self::Dark
    }

    pub fn from_dark(b: bool) -> Self {
        match b {
            true => Self::Dark,
            false => Self::Light,
        }
    }
}

impl Display for Theme {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            Self::Dark => write!(f, "dark"),
            Self::Light => write!(f, "light"),
        }
    }
}
