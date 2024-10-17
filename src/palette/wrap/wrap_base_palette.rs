use serde::{Deserialize, Serialize};

use crate::{color::wrap::wrap_srgb::WrapSrgb, palette::base_palette::BasePalette};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WrapBasePalette {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub dark: bool,
    pub bg: WrapSrgb,
    pub gray: WrapSrgb,
    pub blue: WrapSrgb,
    pub green: WrapSrgb,
    pub yellow: WrapSrgb,
    pub orange: WrapSrgb,
    pub red: WrapSrgb,
    pub purple: WrapSrgb,
    pub pink: WrapSrgb,
}

impl From<BasePalette> for WrapBasePalette {
    fn from(v: BasePalette) -> Self {
        Self {
            schema: "https://raw.githubusercontent.com/ecto0310/vscode_theme_generator/refs/heads/main/schema/palette.json".to_string(),
            dark: v.dark,
            bg: v.bg.into(),
            gray: v.gray.into(),
            blue: v.blue.into(),
            green: v.green.into(),
            yellow: v.yellow.into(),
            orange: v.orange.into(),
            red: v.red.into(),
            purple: v.purple.into(),
            pink: v.pink.into(),
        }
    }
}
