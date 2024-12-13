use serde::{Deserialize, Serialize};

use crate::{
    color::wrap::wrap_srgb::WrapSrgb, model::ActualThemeMode, palette::base_palette::PaletteColor,
    palette::full_palette::FullPalette,
};
use linearize::StaticMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WrapFullPalette {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub dark: bool,
    pub fg: Vec<WrapSrgb>,
    pub base_color_table: StaticMap<PaletteColor, Vec<WrapSrgb>>,
}

impl From<FullPalette> for WrapFullPalette {
    fn from(v: FullPalette) -> Self {
        Self {
            schema: "https://raw.githubusercontent.com/ecto0310/vscode_theme_generator/refs/heads/main/schema/full_palette.json".to_string(),
            dark: v.actual_mode == ActualThemeMode::Dark,
            fg: v
                .fg
                .into_iter()
                .map(|f| f.into())
                .collect::<Vec<WrapSrgb>>(),
            base_color_table: v.base_color_table.map_values(|v| v.iter().map(|v| WrapSrgb::from(*v)).collect()),
        }
    }
}
