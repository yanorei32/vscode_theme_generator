use enum_iterator::Sequence;
use linearize::Linearize;
use palette::{Srgb, Srgba, WithAlpha};
use serde::{
    de::{self, Deserializer, Unexpected, Visitor},
    ser::Serializer,
    Deserialize, Serialize,
};
use std::str::FromStr;
use std::fmt;

#[derive(Debug, Clone, Copy, Linearize, Eq, PartialEq, Sequence, Serialize, Deserialize)]
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

impl FromStr for Color {
    type Err = ();
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "bg" => Self::Bg,
            "gray" => Self::Gray,
            "blue" => Self::Blue,
            "green" => Self::Green,
            "yellow" => Self::Yellow,
            "orange" => Self::Orange,
            "red" => Self::Red,
            "purple" => Self::Purple,
            "pink" => Self::Pink,
            _ => return Err(()),
        })
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

pub trait ReplaceAlphaExt {
    fn alpha(&self, alpha: f32) -> Self;
}

impl ReplaceAlphaExt for HexStr {
    fn alpha(&self, alpha: f32) -> Self {
        let alpha = (u8::MAX as f32 * alpha) as u8;
        HexStr(self.0.color.with_alpha(alpha))
    }
}

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
