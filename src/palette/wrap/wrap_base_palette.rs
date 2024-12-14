use serde::{Deserialize, Serialize};


use palette::Srgba;

use crate::{
    color::{Color, HexStr},
    model::ActualThemeMode,
    palette::base_palette::BasePalette,
};

use linearize::StaticMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WrapBasePalette {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub dark: bool,

    #[serde(flatten)]
    pub color_table: StaticMap<Color, HexStr>,
}

impl From<BasePalette> for WrapBasePalette {
    fn from(v: BasePalette) -> Self {
        Self {
            schema: "https://raw.githubusercontent.com/ecto0310/vscode_theme_generator/refs/heads/main/schema/palette.json".to_string(),
            dark: v.actual_mode == ActualThemeMode::Dark,
            color_table: v.color_table.map_values(|v| HexStr(Srgba::from(v).into())),
        }
    }
}
