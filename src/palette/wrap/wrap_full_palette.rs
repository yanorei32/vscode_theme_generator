use serde::{Deserialize, Serialize};

use linearize::StaticMap;
use palette::Srgba;

use crate::{
    model::{Color, HexStr, ActualThemeMode},
    palette::full_palette::FullPalette,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WrapFullPalette {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub dark: bool,

    #[serde(flatten)]
    pub color_map: StaticMap<Color, Vec<HexStr>>,

    pub fg: Vec<HexStr>,
}

impl From<FullPalette> for WrapFullPalette {
    fn from(v: FullPalette) -> Self {
        Self {
            schema: "https://raw.githubusercontent.com/ecto0310/vscode_theme_generator/refs/heads/main/schema/full_palette.json".to_string(),
            dark: v.actual_mode == ActualThemeMode::Dark,
            fg: v.fg.iter().map(|c| HexStr(Srgba::from(*c).into())).collect(),
            color_map: v.color_map.map_values(|v| v.iter().map(|v| HexStr(Srgba::from(*v).into())).collect()),
        }
    }
}
