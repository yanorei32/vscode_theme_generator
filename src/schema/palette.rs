use serde::{Deserialize, Serialize};

use linearize::StaticMap;
use palette::Srgba;

use crate::{
    model::{Color, HexStr, ActualThemeMode},
    palette::{BasePalette, FullPalette},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasePaletteFile {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub dark: bool,

    #[serde(flatten)]
    pub color_map: StaticMap<Color, HexStr>,
}

impl From<BasePalette> for BasePaletteFile {
    fn from(v: BasePalette) -> Self {
        Self {
            schema: "https://raw.githubusercontent.com/ecto0310/vscode_theme_generator/refs/heads/main/schema/palette.json".to_string(),
            dark: v.dark(),
            color_map: v.take().1.map_values(|v| HexStr(Srgba::from(v).into()))
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FullPaletteFile {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub dark: bool,

    #[serde(flatten)]
    pub color_map: StaticMap<Color, Vec<HexStr>>,

    pub fg: Vec<HexStr>,
}

impl From<FullPalette> for FullPaletteFile {
    fn from(v: FullPalette) -> Self {
        Self {
            schema: "https://raw.githubusercontent.com/ecto0310/vscode_theme_generator/refs/heads/main/schema/full_palette.json".to_string(),
            dark: v.actual_mode == ActualThemeMode::Dark,
            fg: v.fg.iter().map(|c| HexStr(Srgba::from(*c).into())).collect(),
            color_map: v.color_map.map_values(|v| v.iter().map(|v| HexStr(Srgba::from(*v).into())).collect()),
        }
    }
}
