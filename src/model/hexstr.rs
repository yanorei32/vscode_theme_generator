use std::fmt;

use palette::{Srgb, Srgba, WithAlpha};
use serde::{
    de::{self, Deserializer, Unexpected, Visitor},
    ser::Serializer,
    Deserialize, Serialize,
};

#[derive(Debug, Clone, Copy)]
pub struct HexStr(pub Srgba<u8>);

fn srgba_format_short_hex(c: Srgba<u8>) -> String {
    if c.alpha != 255 {
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

impl Serialize for HexStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&srgba_format_short_hex(self.0))
    }
}

struct HexStrVisitor;

impl<'de> Visitor<'de> for HexStrVisitor {
    type Value = HexStr;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("str formatted RGBA (#11223344, #1234) or RGB(#112233, #123)")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(HexStr(srgba_from_hex_str(s).map_err(|_| {
            de::Error::invalid_value(Unexpected::Str(s), &self)
        })?))
    }
}

impl<'de> Deserialize<'de> for HexStr {
    fn deserialize<D>(deserializer: D) -> Result<HexStr, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(HexStrVisitor)
    }
}
