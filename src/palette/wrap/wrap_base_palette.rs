use serde::{Deserialize, Serialize};

use crate::{
    color::wrap::wrap_srgb::WrapSrgb,
    palette::base_palette::{BasePalette, PaletteColor},
    model::ActualThemeMode,
};

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
            dark: v.actual_mode == ActualThemeMode::Dark,
            bg: v.color_table[PaletteColor::Bg].into(),
            gray: v.color_table[PaletteColor::Gray].into(),
            blue: v.color_table[PaletteColor::Blue].into(),
            green: v.color_table[PaletteColor::Green].into(),
            yellow: v.color_table[PaletteColor::Yellow].into(),
            orange: v.color_table[PaletteColor::Orange].into(),
            red: v.color_table[PaletteColor::Red].into(),
            purple: v.color_table[PaletteColor::Purple].into(),
            pink: v.color_table[PaletteColor::Pink].into(),
        }
    }
}
