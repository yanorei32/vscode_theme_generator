use std::fs::File;
use std::io::Write;
use std::path::Path;

use linearize::StaticMap;
use palette::Srgba;
use serde::{Deserialize, Serialize};

use crate::{
    io::{ExportExt, LoadExt},
    model::{ActualThemeMode, Color, HexStr},
    palette::{BasePalette, FullPalette},
};

impl LoadExt for BasePalette {
    fn load(path: &Path) -> anyhow::Result<Self> {
        let palette = std::fs::read_to_string(path)?;
        let palette: BasePaletteExportable = serde_json::from_str(&palette)?;

        let actual_mode = if palette.dark {
            ActualThemeMode::Dark
        } else {
            ActualThemeMode::Light
        };

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
    pub color_map: StaticMap<Color, HexStr>,
}

impl From<BasePalette> for BasePaletteExportable {
    fn from(v: BasePalette) -> Self {
        Self {
            schema: "https://raw.githubusercontent.com/ecto0310/vscode_theme_generator/refs/heads/main/schema/palette.json".to_string(),
            dark: v.dark(),
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
    pub(in crate::io) color_map: StaticMap<Color, Vec<HexStr>>,
    pub(in crate::io) fg: Vec<HexStr>,
}

impl From<FullPalette> for FullPaletteExportable {
    fn from(v: FullPalette) -> Self {
        Self {
            schema: "https://raw.githubusercontent.com/ecto0310/vscode_theme_generator/refs/heads/main/schema/full_palette.json".to_string(),
            dark: v.actual_mode == ActualThemeMode::Dark,
            fg: v.fg.iter().map(|c| HexStr(Srgba::from(*c).into())).collect(),
            color_map: v.color_map.map_values(|v| v.iter().map(|v| HexStr(Srgba::from(*v).into())).collect()),
        }
    }
}
