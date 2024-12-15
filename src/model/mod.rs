use std::fmt::{Display, Error as FmtError, Formatter};

mod hexstr;
pub use hexstr::HexStr;

use clap::ValueEnum;
use enum_iterator::Sequence;
use linearize::{Linearize, StaticCopyMap};
use palette::Srgb;
use serde::{Deserialize, Serialize};

pub type ColorMap = StaticCopyMap<Color, Srgb>;

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

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Linearize, Sequence, Serialize, Deserialize, ValueEnum,
)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    // Fg is always automatically calculated by FullPalette
    Bg,
    Gray,
    Blue,
    Green,
    Yellow,
    Orange,
    Red,
    Purple,
    Pink,
}

impl Color {
    pub fn colorized_iter() -> impl Iterator<Item = Self> {
        enum_iterator::all::<Self>().filter(|v| v.is_colorized())
    }

    pub fn is_bg_color(&self) -> bool {
        *self == Self::Bg
    }

    pub fn is_colorized(&self) -> bool {
        !matches!(self, Self::Bg | Self::Gray)
    }
}
