use std::fmt::{self, Display, Error as FmtError, Formatter};

use clap::ValueEnum;
use enum_iterator::Sequence;
use linearize::{Linearize, StaticMap};
use palette::{Srgb, Srgba, WithAlpha};
use serde::{
    de::{self, Deserializer, Unexpected, Visitor},
    ser::Serializer,
    Deserialize, Serialize,
};

pub type ColorMap = StaticMap<Color, Srgb>;

#[derive(Debug, Clone, ValueEnum, PartialEq, Eq)]
pub enum ThemeDetectionPolicy {
    Auto,
    Dark,
    Light,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

#[derive(Debug, Clone, Copy, Linearize, Eq, PartialEq, Sequence, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum Color {
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
    pub fn is_bg_color(&self) -> bool {
        *self == Self::Bg
    }

    pub fn is_colorized(&self) -> bool {
        !matches!(self, Self::Bg | Self::Gray)
    }
}

fn srgba_format_short_hex(c: Srgba<u8>) -> String {
    if c.alpha != 0 {
        format!("#{:x}", c)
    } else {
        format!("#{:x}", c.color)
    }
}

fn srgba_from_hex_str(s: &str) -> Result<Srgba<u8>, ()> {
    // try parse as Srgba
    let srgba: Result<Srgba<u8>, _> = s[1..].parse();
    if let Ok(srgba) = srgba {
        return Ok(srgba);
    }

    // try parse as Srgb
    let srgb: Result<Srgb<u8>, _> = s[1..].parse();
    if let Ok(srgb) = srgb {
        return Ok(srgb.with_alpha(0u8));
    }

    Err(())
}

#[derive(Debug, Clone, Copy)]
pub struct HexStr(pub Srgba<u8>);

impl Serialize for HexStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&srgba_format_short_hex(self.0))
    }
}

impl<'de> Visitor<'de> for HexStr {
    type Value = HexStr;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("str formatted RGBA (#11223344, #1234) or RGB(#112233, #123)")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Self(srgba_from_hex_str(s).map_err(|_| {
            de::Error::invalid_value(Unexpected::Str(s), &self)
        })?))
    }
}

impl<'de> Deserialize<'de> for HexStr {
    fn deserialize<D>(deserializer: D) -> Result<HexStr, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(HexStr(Srgba::new(0, 0, 0, 0)))
    }
}
