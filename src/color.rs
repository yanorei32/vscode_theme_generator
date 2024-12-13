use enum_iterator::Sequence;
use linearize::Linearize;
use palette::Srgb;
use serde::{Deserialize, Deserializer, Serialize};
use std::str::FromStr;

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

#[derive(Debug, Clone, Copy)]
pub struct SrgbA {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl SrgbA {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn alpha(&self, alpha: f32) -> Self {
        let mut color = *self;
        color.alpha = alpha;
        color
    }
}

impl From<Srgb> for SrgbA {
    fn from(v: Srgb) -> Self {
        Self {
            red: v.red,
            green: v.green,
            blue: v.blue,
            alpha: 1.0,
        }
    }
}

impl From<SrgbA> for Srgb {
    fn from(v: SrgbA) -> Self {
        Srgb::new(v.red, v.green, v.blue)
    }
}

impl Serialize for SrgbA {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let red = (self.red.clamp(0.0, 1.0) * 255.0) as i32;
        let green = (self.green.clamp(0.0, 1.0) * 255.0) as i32;
        let blue = (self.blue.clamp(0.0, 1.0) * 255.0) as i32;
        let alpha = (self.alpha.clamp(0.0, 1.0) * 255.0) as i32;
        let str = if alpha == 255 {
            format!("#{:>02x}{:>02x}{:>02x}", red, green, blue)
        } else {
            format!("#{:>02x}{:>02x}{:>02x}{:>02x}", red, green, blue, alpha)
        };
        serializer.serialize_str(&str)
    }
}

impl<'de> Deserialize<'de> for SrgbA {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?;
        let red = u8::from_str_radix(&str[1..3], 16).unwrap_or(0) as f32 / 255.0;
        let green = u8::from_str_radix(&str[3..5], 16).unwrap_or(0) as f32 / 255.0;
        let blue = u8::from_str_radix(&str[5..7], 16).unwrap_or(0) as f32 / 255.0;
        let alpha = if str.len() == 8 {
            u8::from_str_radix(&str[7..9], 16).unwrap_or(0) as f32 / 255.0
        } else {
            0.0
        };

        Ok(SrgbA::new(red, green, blue, alpha))
    }
}
