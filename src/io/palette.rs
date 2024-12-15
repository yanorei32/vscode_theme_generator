use std::fs::File;
use std::io::Write;
use std::path::Path;

use linearize::StaticCopyMap;
use palette::Srgba;
use serde::{Deserialize, Serialize};

use crate::{
    io::{ExportExt, LoadExt},
    model::{Color, HexStr, Theme},
    palette::{BasePalette, FullPalette},
};

impl LoadExt for BasePalette {
    fn load(path: &Path) -> anyhow::Result<Self> {
        let palette = std::fs::read_to_string(path)?;
        let palette: BasePaletteExportable = serde_json::from_str(&palette)?;

        let actual_mode = Theme::from_dark(palette.dark);
        let color_map = palette.color_map.map_values(|v| v.0.color.into());

        Ok(Self::from_parts(actual_mode, color_map))
    }
}

impl ExportExt for BasePalette {
    fn export(&self, path: &Path) -> anyhow::Result<()> {
        let palette = BasePaletteExportable::from(self.clone());
        let palette = serde_json::to_string(&palette)?;
        File::create(path)?.write_all(palette.as_bytes())?;
        Ok(())
    }
}

impl ExportExt for FullPalette {
    fn export(&self, path: &Path) -> anyhow::Result<()> {
        let palette = FullPaletteExportable::from(self.clone());
        let palette = serde_json::to_string(&palette)?;
        File::create(path)?.write_all(palette.as_bytes())?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BasePaletteExportable {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub dark: bool,

    #[serde(flatten)]
    pub color_map: StaticCopyMap<Color, HexStr>,
}

impl From<BasePalette> for BasePaletteExportable {
    fn from(v: BasePalette) -> Self {
        Self {
            schema: "https://raw.githubusercontent.com/ecto0310/vscode_theme_generator/refs/heads/main/schema/palette.json".to_string(),
            dark: v.theme().dark(),
            color_map: v.take().1.map_values(|v| HexStr(Srgba::from(v).into()))
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(in crate::io) struct FullPaletteExportable {
    #[serde(rename = "$schema")]
    schema: String,
    dark: bool,

    // TODO: 構造が変
    #[serde(flatten)]
    pub(in crate::io) color_map: StaticCopyMap<Color, [HexStr; 5]>,
    pub(in crate::io) fg: [HexStr; 5],
}

impl From<FullPalette> for FullPaletteExportable {
    fn from(v: FullPalette) -> Self {
        Self {
            schema: "https://raw.githubusercontent.com/ecto0310/vscode_theme_generator/refs/heads/main/schema/full_palette.json".to_string(),
            dark: v.theme.dark(),
            fg: v.fg.map(|c| HexStr(Srgba::from(c).into())),
            color_map: v.color_map.map_values(|v| v.map(|v| HexStr(Srgba::from(v).into()))),
        }
    }
}
